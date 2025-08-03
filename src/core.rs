//! Core implementation of the Floco library.

use core::{
    fmt::{Debug, Display},
    marker::PhantomData,
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

    /// Fallible constructor. This is the idiomatic way to create a Floco.
    pub fn try_new(value: F) -> Result<Self, C::Error> {
        if C::is_valid(value) {
            Ok(Floco(value, PhantomData))
        } else {
            Err(C::emit_error(value))
        }
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
                // Use the inherent try_new method for construction.
                Self::try_new(self.0.$func(other.0))
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
        Self::try_new(value).map_err(serde::de::Error::custom)
    }
}

impl<F, C> Default for Floco<F, C>
where
    F: Float + Display + Debug,
    C: Constrained<F>,
{
    fn default() -> Self {
        // We can call try_new here because it's validated at compile-time by the macro.
        // If the default is invalid, the code will not compile.
        Self::try_new(C::DEFAULT).unwrap()
    }
}

/// Defines valid conditions and errors for a Floco marker type.
// FIX: The trait itself is now marked as #[const_trait].
#[const_trait]
pub trait Constrained<F>: Sized
where
    F: Float + Display + Debug,
{
    /// The error type returned on validation failure. Defaults to `ValidationError`.
    type Error: Display + Debug = ValidationError<F>;

    /// The default value for this constrained type.
    const DEFAULT: F;

    // FIX: The `const` keyword is removed from the function signature inside the trait.
    /// A compile-time-compatible function to determine if a value is valid.
    fn is_valid(value: F) -> bool;

    /// Define the error behavior when values do not meet the constraint criteria.
    fn emit_error(value: F) -> Self::Error;
}

/// A convenience macro to quickly define new constrained types with compile-time default validation.
#[macro_export]
macro_rules! constrained_type {
    // ARM 1: User provides an explicit default value.
    ($(#[$outer:meta])* $vis:vis type $TypeName:ident($InnerType:ty) where |$val_id:ident| $validator:expr, $error_msg:expr, default: $default_val:expr) => {
        ::paste::paste! {
            #[doc = "A marker struct for the " $TypeName " constrained type."]
            #[derive(Debug, Copy, Clone)]
            $vis struct [<$TypeName Constraint>];

            // FIX: The `impl` block for a const trait must also be `const`.
            impl const $crate::Constrained<$InnerType> for [<$TypeName Constraint>] {
                const DEFAULT: $InnerType = $default_val;

                fn is_valid($val_id: $InnerType) -> bool {
                    $validator
                }

                // emit_error is not a const function.
                fn emit_error(value: $InnerType) -> Self::Error {
                    $crate::ValidationError { value, message: $error_msg }
                }
            }

            const _: () = {
                assert!([<$TypeName Constraint>]::is_valid([<$TypeName Constraint>]::DEFAULT), "The provided default value does not meet the constraint criteria.");
            };

            $(#[$outer])*
            $vis type $TypeName = $crate::Floco<$InnerType, [<$TypeName Constraint>]>;

            #[allow(dead_code)] // The alias is part of the public API, even if unused in some contexts.
            #[doc = "The associated error type for the `" $TypeName "` constrained type."]
            $vis type [<$TypeName Error>] = <[<$TypeName Constraint>] as $crate::Constrained<$InnerType>>::Error;
        }
    };
    // ARM 2: No default value is provided. Macro will try to inherit from the inner type.
    ($(#[$outer:meta])* $vis:vis type $TypeName:ident($InnerType:ty) where |$val_id:ident| $validator:expr, $error_msg:expr) => {
        $crate::constrained_type! {
            $(#[$outer])* $vis type $TypeName($InnerType)
            where |$val_id| $validator,
            $error_msg,
            // The `const_default` feature is required for this call to work in a const context.
            default: <$InnerType as Default>::default()
        }
    };
}

#[cfg(test)]
mod tests {
    // Import the trait to make its items available in the test scope.
    use crate::constrained_type;
    use crate::Constrained;

    constrained_type! {
        // Removed unnecessary braces around the validation expression.
        pub type PositiveF64(f64) where |val| val > 0.0,
        "Value must be positive.",
        default: 1.0
    }

    constrained_type! {
        pub type NonNegativeF64(f64) where |val| val >= 0.0,
        "Value must be non-negative."
    }

    #[test]
    fn default_value_is_correct() {
        assert_eq!(*PositiveF64::default(), 1.0);
        assert_eq!(*NonNegativeF64::default(), 0.0);
    }

    #[test]
    fn try_new_is_idiomatic() {
        let p = PositiveF64::try_new(10.5).unwrap();
        assert_eq!(*p, 10.5);

        let err_res = PositiveF64::try_new(-5.0);
        assert!(err_res.is_err());
    }

    // FIX: Add a test that uses the generated error alias to silence warnings.
    #[test]
    fn error_alias_is_usable() {
        // We explicitly type the error variable with the generated alias.
        // This marks the alias as "used".
        let err: PositiveF64Error = PositiveF64::try_new(-1.0).unwrap_err();
        assert_eq!(err.value, -1.0);
    }
}
