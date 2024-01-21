// configure no_std if none of the std features are active
#![cfg_attr(all(not(feature = "std_math"), not(feature = "std_serde")), no_std)]

// ensure at least one of libm or std_math are enabled
#[cfg(all(not(feature = "libm"), not(feature = "std_math")))]
compile_error!("One of the 'libm' or 'std_math' features must be enabled.");

// ensure libm and std_math can't be used concurrently
#[cfg(all(feature = "libm", feature = "std_math"))]
compile_error!(
    "The 'libm' (enabled by default) and 'std_math' features cannot be enabled simultaneously."
);

// use the std version if available
#[cfg(feature = "std")]
use std::fmt::{Debug, Display};
#[cfg(feature = "std")]
use std::marker::PhantomData;

// otherwise use the core version
#[cfg(not(feature = "std"))]
use core::fmt::{Debug, Display};
#[cfg(not(feature = "std"))]
use core::marker::PhantomData;

use num_traits::Float;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// This is a doc?
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Floco<F: Float + Debug, C: Constrained<F>>(F, PhantomData<C>);

/// This is a doc?
impl<F: Float + Debug, C: Constrained<F>> Floco<F, C> {
    fn get(&self) -> F {
        self.0
    }
}

/// This is a doc?
impl<F, C> Serialize for Floco<F, C>
where
    F: Float + Debug + Serialize,
    C: Constrained<F>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.get().serialize(serializer)
    }
}

/// This is a doc?
impl<'de, F, C> Deserialize<'de> for Floco<F, C>
where
    F: Float + Debug + Deserialize<'de>,
    C: Constrained<F>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = F::deserialize(deserializer)?;
        C::try_new(value).map_err(serde::de::Error::custom)
    }
}

impl<C: Constrained<f32>> Default for Floco<f32, C> {
    fn default() -> Self {
        Floco::<f32, C>(<C as Constrained<f32>>::DEFAULT_F32, PhantomData)
    }
}

impl<C: Constrained<f64>> Default for Floco<f64, C> {
    fn default() -> Self {
        Floco::<f64, C>(<C as Constrained<f64>>::DEFAULT_F64, PhantomData)
    }
}

impl<C: Constrained<f32>> TryFrom<f32> for Floco<f32, C> {
    type Error = C::Error;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        C::try_new(value)
    }
}

impl<C: Constrained<f64>> TryFrom<f64> for Floco<f64, C> {
    type Error = C::Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        C::try_new(value)
    }
}

pub trait Constrained<F: Float + Debug>: Sized {
    type Error: Debug + Display;

    const DEFAULT_F32: f32;

    const DEFAULT_F64: f64;

    fn is_valid(value: F) -> bool;

    fn emit_error(value: F) -> Self::Error;

    fn try_new(value: F) -> Result<Floco<F, Self>, Self::Error> {
        if Self::is_valid(value) {
            Ok(Floco::<F, Self>(value, PhantomData))
        } else {
            Err(Self::emit_error(value))
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
