// src/macros/impl_dimensional_ops.rs

/// A macro to implement arithmetic operations between different `Floco` newtypes,
/// enabling dimensionally-aware calculations with libraries like `uom`.
///
/// # Example
///
/// ```rust,ignore
/// // Implements `PositiveLength / PositiveTime` -> `Result<SafeVelocity, ...>`
/// impl_dimensional_ops!(PositiveLength, Div, div, PositiveTime => SafeVelocity);
/// ```
#[macro_export]
macro_rules! impl_dimensional_ops {
    ($Lhs:ty, $Trait:ident, $func:ident, $Rhs:ty => $Result:ty) => {
        impl ::core::ops::$Trait<$Rhs> for $Lhs
        {
            // The output of this operation is the `Result` returned by the
            // `try_new` constructor of the output type (e.g., `Ok(SafeVelocity)` or `Err(...)`).
            type Output = Result<$Result, $crate::ValidationError<<$Result as ::core::ops::Deref>::Target>>;

            fn $func(self, other: $Rhs) -> Self::Output {
                // 1. Dereference `self` and `other` to get the inner values (e.g., `Length`, `Time`).
                // 2. Perform the arithmetic operation on the inner values.
                let result_inner = (*self).$func(*other);

                // 3. Call `try_new` on the result type to validate the outcome.
                <$Result>::try_new(result_inner)
            }
        }
    };
}