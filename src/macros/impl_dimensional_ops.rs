// src/macros/impl_dimensional_ops.rs

/// A macro to implement arithmetic operations between different `Floco` types,
/// enabling dimensionally-aware calculations with libraries like `uom`.
macro_rules! impl_dimensional_ops {
    ($Lhs:ty, $Trait:ident, $func:ident, $Rhs:ty => $Result:ty) => {
        impl $Trait<$Rhs> for $Lhs
        where
            <$Lhs as ::core::ops::Deref>::Target: $Trait<<$Rhs as ::core::ops::Deref>::Target>,
            <$Result as ::core::ops::Deref>::Target: From<
                <<($Lhs) as ::core::ops::Deref>::Target as $Trait<
                    <$Rhs as ::core::ops::Deref>::Target,
                >>::Output,
            >,
        {
            type Output = Result<$Result, <$Result as $crate::types::HasError>::Error>;
            fn $func(self, other: $Rhs) -> Self::Output {
                let result_inner = self.0.$func(other.0);
                <$Result>::try_new(result_inner.into())
            }
        }
    };
}
