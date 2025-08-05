// src/types.rs

use core::fmt::{Debug, Display};
use core::marker::PhantomData;
use core::ops::{Add, Deref, Div, Mul, Sub};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

/// A default, structured error type for validation failures.
#[derive(Debug, Error, Clone)]
#[error("Validation failed for value '{value:?}': {message}")]
pub struct ValidationError<T: Debug> {
    pub value: T,
    pub message: &'static str,
}

/// A wrapper type that contains a value and a PhantomData marker that implements the [Constrained] trait.
#[derive(Debug, Clone)]
pub struct Floco<T, C>(pub T, pub PhantomData<C>);

impl<T, C> Floco<T, C>
where
    // The `PartialOrd` bound has been removed to support types like `Vec<T>`.
    T: Debug + Clone,
    C: Constrained<T>,
{
    /// Fallible constructor. This is the idiomatic way to create a Floco.
    pub fn try_new(value: T) -> Result<Self, C::Error> {
        if C::is_valid(&value) {
            Ok(Floco(value, PhantomData))
        } else {
            Err(C::emit_error(value))
        }
    }
}

impl<T, C> Deref for Floco<T, C> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Serde implementations
impl<T, C> Serialize for Floco<T, C>
where
    T: Debug + Clone + Serialize,
    C: Constrained<T>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T, C> Deserialize<'de> for Floco<T, C>
where
    T: Debug + Clone + Deserialize<'de>,
    C: Constrained<T>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Self::try_new(value).map_err(serde::de::Error::custom)
    }
}

// Arithmetic implementations for the core Floco type
// These retain their `PartialOrd` bound because it makes sense for arithmetic.
impl<T, C> Add for Floco<T, C>
where
    T: Add<Output = T> + PartialOrd + Debug + Clone,
    C: Constrained<T>,
{
    type Output = Result<Self, C::Error>;
    fn add(self, other: Self) -> Result<Self, C::Error> {
        Self::try_new(self.0.add(other.0))
    }
}

impl<T, C> Sub for Floco<T, C>
where
    T: Sub<Output = T> + PartialOrd + Debug + Clone,
    C: Constrained<T>,
{
    type Output = Result<Self, C::Error>;
    fn sub(self, other: Self) -> Result<Self, C::Error> {
        Self::try_new(self.0.sub(other.0))
    }
}

impl<T, C> Mul for Floco<T, C>
where
    T: Mul<Output = T> + PartialOrd + Debug + Clone,
    C: Constrained<T>,
{
    type Output = Result<Self, C::Error>;
    fn mul(self, other: Self) -> Result<Self, C::Error> {
        Self::try_new(self.0.mul(other.0))
    }
}

impl<T, C> Div for Floco<T, C>
where
    T: Div<Output = T> + PartialOrd + Debug + Clone,
    C: Constrained<T>,
{
    type Output = Result<Self, C::Error>;
    fn div(self, other: Self) -> Result<Self, C::Error> {
        Self::try_new(self.0.div(other.0))
    }
}

/// Defines valid conditions and errors for a Floco marker type.
#[cfg_attr(feature = "const-validation", const_trait)]
pub trait Constrained<T>: Sized
where
    // The `PartialOrd` bound has been removed here as well.
    T: Debug + Clone,
{
    type Error: Display + Debug = ValidationError<T>;
    fn is_valid(value: &T) -> bool;
    fn emit_error(value: T) -> Self::Error;
}
