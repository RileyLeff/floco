//! Core implementation of the Floco library.

use core::{
    fmt::{Debug, Display},
    marker::PhantomData,
    // Note: AddAssign, SubAssign, etc., have been removed as they are infallible traits.
    ops::{Add, Deref, Div, Mul, Sub},
};

use num_traits::Float;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

/// A default, structured error type for validation failures.
#[derive(Debug, Error, Clone, Copy)]
#[error("Validation failed for value '{value:?}': {message}")]
pub struct ValidationError<F: Debug> {
    /// The numeric value that failed the constraint check.
    pub value: F,
    /// A static string describing the constraint that was violated.
    pub message: &'static str,
}

/// A wrapper type that contains a floating point value and a PhantomData marker that implements the [Constrained] trait.
#[derive(Debug, Copy, Clone)]
pub struct Floco<F, C>(pub F, pub PhantomData<C>);

impl<F, C> Floco<F, C>
where
    F: Float + Display + Debug,
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

    /// Updates a floco's inner value without checking for validity. Use with caution.
    pub fn mutate_unchecked(&mut self, new_val: F) {
        self.0 = new_val;
    }

    /// Fallible constructor. Equivalent to the try_new in the marker type's impl.
    pub fn try_new(value: F) -> Result<Self, C::Error> {
        C::try_new(value)
    }
}

impl<F, C> Deref for Floco<F, C> {
    type Target = F;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

macro_rules! impl_ops {
    ($Trait:ident, $func:ident) => {
        impl<F, C> $Trait for Floco<F, C>
        where
            F: Float + Display + Debug + $Trait<Output = F>,
            C: Constrained<F>,
        {
            type Output = Result<Self, C::Error>;
            fn $func(self, other: Self) -> Self::Output {
                C::try_new(self.0.$func(other.0))
            }
        }
    };
}

impl_ops!(Add, add);
impl_ops!(Sub, sub);
impl_ops!(Mul, mul);
impl_ops!(Div, div);

impl<F, C> Serialize for Floco<F, C>
where
    F: Float + Display + Debug + Serialize,
    C: Constrained<F>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.get().serialize(serializer)
    }
}

impl<'de, F, C> Deserialize<'de> for Floco<F, C>
where
    F: Float + Display + Debug + Deserialize<'de>,
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
    F: Float + Display + Debug,
    C: Constrained<F>,
{
    fn default() -> Self {
        let default_val = C::get_default();
        C::try_new(default_val)
            .expect("The default value provided by the Constrained impl was invalid.")
    }
}

/// Defines valid conditions and errors for a Floco marker type.
pub trait Constrained<F>: Sized
where
    F: Float + Display + Debug,
{
    /// The error type returned on validation failure. Defaults to `ValidationError`.
    type Error: Display + Debug = ValidationError<F>;

    /// Function to determine whether a value is valid.
    fn is_valid(value: F) -> bool;

    /// Define the error behavior when values do not meet the constraint criteria.
    fn emit_error(value: F) -> Self::Error;

    /// Define a default value for a constraint type. Defaults to `F::zero()`.
    fn get_default() -> F {
        F::zero()
    }

    /// Fallible constructor for a Floco of this constraint type.
    fn try_new(value: F) -> Result<Floco<F, Self>, Self::Error> {
        if Self::is_valid(value) {
            Ok(Floco(value, PhantomData))
        } else {
            Err(Self::emit_error(value))
        }
    }
}

/// A convenience macro to quickly define new constrained types.
#[macro_export]
macro_rules! constrained_type {
    ($(#[$outer:meta])* $vis:vis type $TypeName:ident($InnerType:ty) where |$val_id:ident| $validator:expr, $error_msg:expr) => {
        ::paste::paste! {
            #[doc = "A marker struct for the " $TypeName " constrained type."]
            #[derive(Debug, Copy, Clone)]
            $vis struct [<$TypeName Constraint>];

            impl $crate::Constrained<$InnerType> for [<$TypeName Constraint>] {
                fn is_valid($val_id: $InnerType) -> bool {
                    $validator
                }

                fn emit_error(value: $InnerType) -> Self::Error {
                    $crate::ValidationError {
                        value,
                        message: $error_msg,
                    }
                }
            }

            $(#[$outer])*
            $vis type $TypeName = $crate::Floco<$InnerType, [<$TypeName Constraint>]>;

            // THE NEW ADDITION:
            // Automatically create an error type alias, e.g., `TurgorError`.
            #[doc = "The associated error type for the `" $TypeName "` constrained type."]
            $vis type [<$TypeName Error>] = <[<$TypeName Constraint>] as $crate::Constrained<$InnerType>>::Error;
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{constrained_type, Constrained, Floco};

    constrained_type! {
        /// RWC must be between 0.0 and 1.0
        pub type RWC(f64) where |val| { val >= 0.0 && val <= 1.0 },
        "RWC must be a proportion [0.0, 1.0]"
    }

    #[test]
    fn macro_creates_valid_type() {
        let rwc = RWC::try_new(0.5).unwrap();
        assert_eq!(rwc.get(), 0.5);
    }

    #[test]
    fn macro_type_fails_with_structured_error() {
        let err = RWC::try_new(1.1).unwrap_err();
        assert_eq!(err.value, 1.1);
    }
}
