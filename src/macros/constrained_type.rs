// src/macros/constrained_type.rs

// This helper macro must be public for the `const-validation` feature path to work in tests.
#[macro_export]
#[doc(hidden)]
macro_rules! __floco_const_impl {
    ($($body:tt)*) => {
        impl const $($body)*
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __floco_internal_newtype_impl {
    (
        $(#[$outer:meta])* ,
        $vis:vis,
        $TypeName:ident,
        $(< $($gen:tt)+ >)? ,
        $InnerType:ty,
        |$val_id:ident| $validator:expr,
        $error_msg:expr
    ) => {
        ::paste::paste! {
            #[doc = "A marker struct for the constrained type."]
            #[derive(Debug, Clone, Default)]
            $vis struct [<$TypeName Constraint>]$(< $($gen)+ >)? {
                $( _phantom: ::core::marker::PhantomData<($($gen)+)>,)?
            }

            #[cfg(feature = "const-validation")]
            $crate::__floco_const_impl! {
                $crate::Constrained<$InnerType> for [<$TypeName Constraint>] {
                    fn is_valid($val_id: &$InnerType) -> bool { $validator }
                    fn emit_error(value: $InnerType) -> Self::Error { $crate::ValidationError { value, message: $error_msg } }
                }
            }

            #[cfg(not(feature = "const-validation"))]
            impl$(< $($gen)+ >)? $crate::Constrained<$InnerType> for [<$TypeName Constraint>]$(< $($gen)+ >)?
            where
                $InnerType: Debug + Clone,
            {
                fn is_valid($val_id: &$InnerType) -> bool { $validator }
                fn emit_error(value: $InnerType) -> Self::Error { $crate::ValidationError { value, message: $error_msg } }
            }

            $(#[$outer])*
            #[derive(Debug, Clone)]
            $vis struct $TypeName$(< $($gen)+ >)?(pub $crate::Floco<$InnerType, [<$TypeName Constraint>]$(< $($gen)+ >)?>);

            impl$(< $($gen)+ >)? serde::Serialize for $TypeName$(< $($gen)+ >)?
            where
                $InnerType: serde::Serialize + Debug + Clone,
            {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    self.0.serialize(serializer)
                }
            }

            impl<'de, $( $($gen)+ )?> serde::Deserialize<'de> for $TypeName$(< $($gen)+ >)?
            where
                $InnerType: serde::Deserialize<'de> + Debug + Clone,
            {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    let value = <$InnerType>::deserialize(deserializer)?;
                    Self::try_new(value).map_err(serde::de::Error::custom)
                }
            }

            impl$(< $($gen)+ >)? $TypeName$(< $($gen)+ >)? {
                pub fn try_new(value: $InnerType) -> Result<Self, $crate::ValidationError<$InnerType>> {
                    Ok(Self($crate::Floco::<_, [<$TypeName Constraint>]$(< $($gen)+ >)?>::try_new(value)?))
                }
                pub fn get(&self) -> &$InnerType { &(self.0).0 }
                pub fn into_inner(self) -> $InnerType { (self.0).0 }
            }

            impl$(< $($gen)+ >)? ::core::ops::Deref for $TypeName$(< $($gen)+ >)? {
                type Target = $InnerType;
                fn deref(&self) -> &Self::Target { &(self.0).0 }
            }
        }
    };
}

/// Creates a new, validated type from an existing one. See the crate-level documentation for examples.
#[macro_export]
macro_rules! constrained_type {
    // ARM 1: Generic, with default
    ($(#[$outer:meta])* $vis:vis type $TypeName:ident < $($gen:tt)+ > for $InnerType:ty where |$val_id:ident| $validator:expr, $error_msg:expr, default: $default_val:expr) => {
        $crate::__floco_internal_newtype_impl!{
            $(#[$outer])*, $vis, $TypeName, <$($gen)+>, $InnerType, |$val_id| $validator, $error_msg
        }
        ::paste::paste! {
            impl< $($gen)+ > Default for $TypeName<$($gen)+> {
                fn default() -> Self {
                    Self::try_new($default_val).expect("The provided default value is invalid.")
                }
            }
        }
    };

    // ARM 2: Generic, no default
    ($(#[$outer:meta])* $vis:vis type $TypeName:ident < $($gen:tt)+ > for $InnerType:ty where |$val_id:ident| $validator:expr, $error_msg:expr) => {
        $crate::__floco_internal_newtype_impl!{
            $(#[$outer])*, $vis, $TypeName, <$($gen)+>, $InnerType, |$val_id| $validator, $error_msg
        }
        ::paste::paste! {
            impl< $($gen)+ > Default for $TypeName<$($gen)+> where $InnerType: Default {
                fn default() -> Self {
                    Self::try_new(<$InnerType as Default>::default()).expect("The default value of the inner type is invalid.")
                }
            }
        }
    };

    // ARM 3: Non-generic, with default
    ($(#[$outer:meta])* $vis:vis type $TypeName:ident for $InnerType:ty where |$val_id:ident| $validator:expr, $error_msg:expr, default: $default_val:expr) => {
        $crate::__floco_internal_newtype_impl!{
            $(#[$outer])*, $vis, $TypeName, , $InnerType, |$val_id| $validator, $error_msg
        }
        impl Default for $TypeName {
            fn default() -> Self {
                Self::try_new($default_val).expect("The provided default value is invalid.")
            }
        }
    };

    // ARM 4: Non-generic, no default
    ($(#[$outer:meta])* $vis:vis type $TypeName:ident for $InnerType:ty where |$val_id:ident| $validator:expr, $error_msg:expr) => {
        $crate::__floco_internal_newtype_impl!{
            $(#[$outer])*, $vis, $TypeName, , $InnerType, |$val_id| $validator, $error_msg
        }
        impl Default for $TypeName where $InnerType: Default {
            fn default() -> Self {
                Self::try_new(<$InnerType as Default>::default()).expect("The default value of the inner type is invalid.")
            }
        }
    };
}