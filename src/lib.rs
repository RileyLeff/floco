//! This is my rust crate floco, it's pretty sick

#![warn(missing_docs)]
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
pub struct Floco<F, C>(F, PhantomData<C>)
where
    F: Float,
    C: Constrained<F>;

/// This is a doc?
impl<F, C> Floco<F, C>
where
    F: Float,
    C: Constrained<F>,
{
    fn get(&self) -> F {
        self.0
    }

    #[allow(dead_code)]
    fn try_new(value: F) -> Result<Self, C::Error> {
        C::try_new(value)
    }
}

/// This is a doc?
impl<F, C> Serialize for Floco<F, C>
where
    F: Float + Serialize,
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
    F: Float + Deserialize<'de>,
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

impl<F, C> Default for Floco<F, C>
where
    F: Float,
    C: Constrained<F>,
{
    fn default() -> Self {
        Floco::<F, C>(<C as Constrained<F>>::get_default(), PhantomData)
    }
}

impl<C> TryFrom<f32> for Floco<f32, C>
where
    C: Constrained<f32>,
{
    type Error = C::Error;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        C::try_new(value)
    }
}

impl<C> TryFrom<f64> for Floco<f64, C>
where
    C: Constrained<f64>,
{
    type Error = C::Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        C::try_new(value)
    }
}

/// TODO: doc this
pub trait Constrained<F>: Sized
where
    F: Float,
{
    /// TODO: doc this
    type Error: Display;

    /// TODO: doc this
    fn is_valid(value: F) -> bool;

    /// TODO: doc this
    fn emit_error(value: F) -> Self::Error;

    /// TODO: doc this
    fn get_default() -> F {
        F::zero()
    }

    /// TODO: doc this
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
    use crate::{Constrained, Floco};
    use core::fmt::Debug;
    use half::f16;
    use num_traits::Float;

    struct Foo;

    impl Constrained<f64> for Foo {
        type Error = &'static str;

        fn get_default() -> f64 {
            99.9f64
        }

        fn is_valid(value: f64) -> bool {
            value.is_normal() && value.is_sign_positive()
        }

        fn emit_error(_value: f64) -> Self::Error {
            "wow this is a bad foo"
        }
    }

    struct Bar;

    impl Constrained<f64> for Bar {
        type Error = &'static str;

        fn get_default() -> f64 {
            -50.0f64
        }

        fn is_valid(value: f64) -> bool {
            value.is_normal() && value.is_sign_negative()
        }

        fn emit_error(_value: f64) -> Self::Error {
            "yikes this is a bad bar"
        }
    }

    struct Qux;
    impl<F: Float + Debug> Constrained<F> for Qux {
        type Error = &'static str;

        fn is_valid(value: F) -> bool {
            value.is_sign_positive()
        }

        fn emit_error(_value: F) -> Self::Error {
            "omg this is a bad qux"
        }
    }

    #[test]
    fn floco_denies_invalid_f64() {
        // Foo is restricted to positive normal f64s
        let should_be_error = Foo::try_new(-9.2);
        assert!(should_be_error.is_err())
    }

    #[test]
    fn default_from_marker_works() {
        assert_eq!(Floco::<f64, Bar>::default().get(), -50.0f64)
    }

    #[test]
    fn deserialization_respects_floco_validation() {
        let to_be_deserialized = "42.1";
        let should_be_error: Result<(Floco<f64, Bar>, usize), _> =
            serde_json_core::from_str(to_be_deserialized);
        assert!(should_be_error.is_err())
    }

    #[test]
    fn serialization_grabs_inner_float() {
        let to_be_serialized = Foo::try_new(42.0f64).unwrap();
        let mut buff = [0u8; 4];
        let expected_buff: [u8; 4] = [52, 50, 46, 48];
        let _ = serde_json_core::to_slice(&to_be_serialized, &mut buff).unwrap();
        assert_eq!(expected_buff, buff)
    }

    #[test]
    fn tryfrom_works_with_floco() {
        let lorem: f64 = 2.1;
        let ipsum = Floco::<f64, Foo>::try_from(lorem).unwrap();
        let sit = Foo::try_new(lorem).unwrap();
        assert_eq!(ipsum.get(), sit.get())
    }

    #[test]
    fn tryinto_works_with_floco() {
        let lorem: f64 = 2.1;
        let ipsum: Result<Floco<f64, Bar>, <Bar as Constrained<f64>>::Error> = lorem.try_into();
        assert!(ipsum.is_err())
    }

    #[test]
    fn exotic_float_works_on_generic_marker_impl() {
        let lorem: f16 = f16::ONE;
        let ipsum = Qux::try_new(lorem);
        assert!(ipsum.is_ok())
    }
}
