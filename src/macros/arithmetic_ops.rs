/// Implements standard arithmetic operations for a Floco newtype where the
/// output of the operation is the same as the input type.
///
/// This is the idiomatic way to handle arithmetic for a single constrained type,
/// ensuring that the result of any operation is re-validated against the
/// type's own constraints. This is the correct tool for "same-type" math.
///
/// # Example
///
/// ```rust,ignore
/// // Implements `+`, `-`, `*`, and `/` for the `MyType` newtype.
/// impl_arithmetic_ops!(MyType, Add, add, Sub, sub, Mul, mul, Div, div);
/// ```
#[macro_export]
macro_rules! impl_arithmetic_ops {
    // The matcher uses `+` to require at least one pair of (Trait, function).
    ($TypeName:ty, $($Trait:ident, $func:ident),+) => {
        // The `$()*` block repeats the implementation for each pair provided.
        $(
            impl ::core::ops::$Trait for $TypeName {
                // The output is a Result containing the type itself, or a validation error.
                type Output = Result<$TypeName, $crate::ValidationError<<$TypeName as ::core::ops::Deref>::Target>>;

                fn $func(self, other: Self) -> Self::Output {
                    // 1. Perform the operation on the inner values.
                    let result_inner = self.get().$func(other.get());

                    // 2. Re-validate the result against the type's OWN constraints.
                    // This is the core safety guarantee.
                    <$TypeName>::try_new(result_inner)
                }
            }
        )+
    };
}
