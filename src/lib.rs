//! Floco validates ***flo***ats against user-defined ***co***nstraints.
//!
//! # Quick Start
//!
//! ```
//! use floco::{Floco, Constrained};
//!
//! // We want to represent a value as f64, but we don't want to allow:
//! //      - values below 5.0f64
//! //      - values above 7.2f64
//!
//! // We define an empty struct.
//! // This won't contain data, but will contain validation criteria in its impl.
//! struct Foo;
//!
//! // The Constrained trait defines the above constraints, an error type, and a default value.
//! impl Constrained<f64> for Foo {
//!     
//!     type Error = &'static str;
//!     
//!     fn is_valid(value: f64) -> bool {
//!         value >= 5.0f64 && value <= 7.2f64
//!     }
//!
//!     fn emit_error(_value: f64) -> Self::Error {
//!         "yikes this is a bad foo"
//!     }
//!     
//!     // Optionally, you can set a custom default.
//!     // Floco::<F, YourType>::Default will respect the default value impl for YourType.
//!     fn get_default() -> f64 {
//!         5.2f64
//!      }
//! }
//!
//! // Now we can use Foo to constrain a Floco
//! let this_will_be_ok = Floco::<f64, Foo>::try_new(6.8);
//! let this_will_be_err = Floco::<f64, Foo>::try_new(4.2);
//!
//! ```
//!
//! # Overview
//!
//! This crate provides a struct that wraps a floating-point number alongside a PhantomData marker
//! type. The marker type defines arbitrary validation conditions for the inner float.
//! These validation conditions are invoked during construction, conversion, and deserialization.
//!
//! The marker type also provides an Error type and an optional default value (defaults to zero).
//!
//! Floco is no_std compatible by default, but has optional support for the standard library behind
//! a feature flag. This doesn't add any functionality, just changes math ops from libm to std and
//! changes the errors from thiserror-core to thiserror. Floco should compile on stable if std is
//! enabled, but will require the [error_in_core][`eiclink`] feature for no_std builds.
//!
//! Floco is compatible with any type that implements the [float][`ntFloatlink`] trait from
//! the num_traits crate. TryFrom conversions are implemented from f32 and f64 for
//! convenience.
//!
//! # Roadmap
//! - At some point I intend to implement the ops traits on the Floco struct.
//! - At some point I intend to add a macro to reduce the newtype boilerplate.
//! - I want to create a similar struct that also contains generic [uom][`uomlink`] dimensions, but might just put that in a separate crate.
//! - Not sure what to do with the Copy trait. Need to think that through.
//!
//! # Alternative / Related Crates
//! - [prae][`PraeLink`] uses a macro to create distinct types rather than making a single type generic across arbitrary marker impls. Prae is incompatible with no_std.
//! - [tightness][`TightnessLink`] is a predecessor to prae.
//! - [typed_floats][`TypedFloatLink`] provides 12 useful pre-made restricted float types. See the useful "Similar crates" section at the end of the TypedFloats readme.
//!
//! # Inspired By
//! - [Tightness Driven Development in Rust][`TightnessPost`]
//! - [this stackoverflow comment][`SOComment`]
//! - [this reddit comment][`RedditComment`]
//!
//! [`PraeLink`]: https://github.com/teenjuna/prae
//! [`TightnessLink`]: https://github.com/PabloMansanet/tightness
//! [`TypedFloatLink`]: https://github.com/tdelmas/typed_floats
//! [`TightnessPost`]: https://www.ecorax.net/tightness/
//! [`RedditComment`]: https://www.reddit.com/r/rust/comments/abmilm/bounded_numeric_types/ed1fs0f/
//! [`SOComment`]: https://stackoverflow.com/questions/57440412/implementing-constructor-function-in-rust-trait#comment101360200_57440412
//! [`eiclink`]:  https://github.com/rust-lang/rust/issues/103765
//! [`ntFloatlink`]: https://docs.rs/num-traits/latest/num_traits/float/trait.Float.html
//! [`uomlink`]: https://github.com/iliekturtles/uom

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

use floatd::FloatD;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A wrapper type that contains a floating point value and a PhantomData marker that implements the [Constrained] trait.
/// The marker type's implementation determines whether the float value is valid to construct an instance of a Floco.
#[derive(Debug)]
pub struct Floco<F, C>(pub F, pub PhantomData<C>)
where
    F: FloatD,
    C: Constrained<F>;

impl<F, C> Floco<F, C>
where
    F: FloatD,
    C: Constrained<F>,
{
    /// Extracts the inner float value from a Floco wrapper instance.
    pub fn get(&self) -> F {
        self.0
    }

    /// Updates a floco's inner value if the new value is deemed valid.
    pub fn mutate(&mut self, new_val: F) -> Result<(), C::Error> {
        if C::is_valid(new_val) {
            self.0 = new_val;
            Ok(())
        } else {
            Err(C::emit_error(new_val))
        }
    }

    /// Updates a floco's inner value without checking for validity. Use wisely.
    pub fn mutate_unchecked(&mut self, new_val: F) {
        self.0 = new_val;
    }

    /// Fallible constructor. Equivalent to the try_new in the marker type's impl.
    #[allow(dead_code)]
    pub fn try_new(value: F) -> Result<Self, C::Error> {
        C::try_new(value)
    }
}

/// Serialization across arbitrary constraints.
impl<F, C> Serialize for Floco<F, C>
where
    F: FloatD + Serialize,
    C: Constrained<F>,
{
    /// Serializing a Floco object extracts the inner float.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.get().serialize(serializer)
    }
}

/// Constrained deserialization, with constraints checked against the marker type.
impl<'de, F, C> Deserialize<'de> for Floco<F, C>
where
    F: FloatD + Deserialize<'de>,
    C: Constrained<F>,
{
    /// Deserializing a number into a Floco instance activates the constraining type's validity check.
    /// Will return a Result<Err> if the validity criteria are not met.
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
    F: FloatD,
    C: Constrained<F>,
{
    /// Default values are also grabbed from the marker implementation.
    /// The default impl is zero, as in f32 and f64.
    /// This crate uses a default impl for the Default trait instead of looking for hard-coded const values to accomodate exotically-sized floats.
    fn default() -> Self {
        Floco::<F, C>(<C as Constrained<F>>::get_default(), PhantomData)
    }
}

/// Convenince impl of TryFrom for f32-based conversions.
impl<C> TryFrom<f32> for Floco<f32, C>
where
    C: Constrained<f32>,
{
    type Error = C::Error;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        C::try_new(value)
    }
}

/// Convenince impl of TryFrom for f64-based conversions.
impl<C> TryFrom<f64> for Floco<f64, C>
where
    C: Constrained<f64>,
{
    type Error = C::Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        C::try_new(value)
    }
}

/// Defines valid conditions and errors for a Floco marker type.
/// Also has overridable default impls for a fallible constructor and a default value.
pub trait Constrained<F>: Sized
where
    F: FloatD,
{
    /// For example, one could use &str, anyhow, or a thiserror enum.
    type Error: Display;

    /// Function to determine whether a value is valid.
    fn is_valid(value: F) -> bool;

    /// Define the error behavior when values do not meet the constraint criteria.
    fn emit_error(value: F) -> Self::Error;

    /// Define a default value for a constraint type.
    fn get_default() -> F {
        F::zero()
    }

    /// Fallible constructor for a Floco of this constraint type.
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
    use floatd::FloatD;
    use half::f16;

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
    impl<F: FloatD + Debug> Constrained<F> for Qux {
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

    #[test]
    fn checked_mutability_works() {
        let mut ipsum = Qux::try_new(1.0f64).unwrap();
        let _x = ipsum.mutate(2.0f64);
        assert!(ipsum.0 == 2.0f64);
    }

    #[test]
    fn checked_mutability_catches_errors() {
        let mut ipsum = Qux::try_new(1.0f64).unwrap();
        let x = ipsum.mutate(-2.0f64);
        assert!(x.is_err());
    }

    #[test]
    fn unchecked_mutability_works() {
        let mut ipsum = Qux::try_new(1.0f64).unwrap();
        let _x = ipsum.mutate_unchecked(2.0f64);
        assert!(ipsum.0 == 2.0f64);
    }
}
