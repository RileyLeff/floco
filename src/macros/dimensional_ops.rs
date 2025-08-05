// src/macros/dimensional_ops.rs

/// Implements dimensional or cross-type operations (e.g., A / B => C).
#[macro_export]
macro_rules! impl_dimensional_ops {
    ($Lhs:ty, $Trait:ident, $func:ident, $Rhs:ty => $Result:ty) => {
        impl ::core::ops::$Trait<$Rhs> for $Lhs {
            type Output =
                Result<$Result, $crate::ValidationError<<$Result as ::core::ops::Deref>::Target>>;
            fn $func(self, other: $Rhs) -> Self::Output {
                let result_inner = (*self).$func(*other);
                <$Result>::try_new(result_inner)
            }
        }
    };
}
