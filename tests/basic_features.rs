// tests/basic_features.rs

#![cfg_attr(feature = "const-validation", feature(const_trait_impl))]
#![cfg_attr(feature = "const-validation", feature(const_default))]
#![feature(associated_type_defaults)]

use floco::constrained_type;

// These definitions now work on stable Rust by default.
// The `const-validation` feature will add compile-time checks for defaults.
constrained_type! { pub type PositiveF64(f64) where |val| *val > 0.0, "Value must be positive.", default: 1.0 }
constrained_type! { pub type NonNegativeI32(i32) where |i| *i >= 0, "Value must be non-negative." }
constrained_type! { pub type Percentage(f64) where |p| *p >= 0.0 && *p <= 100.0, "Value must be between 0.0 and 100.0.", default: 0.0 }

#[cfg(not(feature = "const-validation"))]
constrained_type! {
    pub type NonEmptyString(String) where |s| !s.is_empty(), "String must not be empty."
}

#[cfg(not(feature = "const-validation"))]
constrained_type! {
    pub type NonEmptyVec<T>(Vec<T>) where |v| !v.is_empty(), "Vector must not be empty."
}

#[test]
fn test_creation_and_validation() {
    assert!(Percentage::try_new(50.0).is_ok());
    assert!(Percentage::try_new(101.0).is_err());
}

#[test]
fn test_deref_get_and_into_inner() {
    let p = Percentage::try_new(42.0).unwrap();

    // Test deref
    assert_eq!(*p, 42.0);

    // Test get() -> &f64
    assert_eq!(p.get(), &42.0);

    // Test into_inner() -> f64
    assert_eq!(p.into_inner(), 42.0);
}

#[test]
fn test_default() {
    assert_eq!(*Percentage::default(), 0.0);
    assert_eq!(*PositiveF64::default(), 1.0);
    assert_eq!(*NonNegativeI32::default(), 0);
}

#[test]
fn test_serde_integration() {
    let p = Percentage::try_new(55.5).unwrap();
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, "55.5");

    let deserialized: Percentage = serde_json::from_str(&json).unwrap();
    assert_eq!(*deserialized, 55.5);

    let invalid_json = "101.0";
    let err = serde_json::from_str::<Percentage>(invalid_json);
    assert!(err.is_err());
}

#[test]
fn test_serde_deserialization_failure() {
    let invalid_json = "101.0";
    let err = serde_json::from_str::<Percentage>(invalid_json);
    assert!(err.is_err());
    let err_msg = err.unwrap_err().to_string();
    assert!(err_msg
        .contains("Validation failed for value '101.0': Value must be between 0.0 and 100.0."));
}

// Test with a Clone-only, non-Copy type
#[cfg(not(feature = "const-validation"))]
#[test]
fn test_string_type() {
    let valid_str = NonEmptyString::try_new("hello".to_string()).unwrap();
    assert_eq!(valid_str.get(), "hello");

    let invalid_str = NonEmptyString::try_new("".to_string());
    assert!(invalid_str.is_err());

    // Test clone
    let cloned_str = valid_str.clone();
    assert_eq!(cloned_str.get(), "hello");

    // Test into_inner
    let inner_str = cloned_str.into_inner();
    assert_eq!(inner_str, "hello");
}

#[cfg(not(feature = "const-validation"))]
#[test]
fn test_serde_string() {
    let s = NonEmptyString::try_new("hello".to_string()).unwrap();
    let json = serde_json::to_string(&s).unwrap();
    assert_eq!(json, "\"hello\"");

    let deserialized: NonEmptyString = serde_json::from_str(&json).unwrap();
    assert_eq!(*deserialized, "hello");

    let invalid_json = "\"\"";
    let err = serde_json::from_str::<NonEmptyString>(invalid_json);
    assert!(err.is_err());
}

#[cfg(not(feature = "const-validation"))]
#[test]
fn test_generic_vec() {
    let valid_vec = NonEmptyVec::try_new(vec![1, 2, 3]).unwrap();
    assert_eq!(valid_vec.get(), &vec![1, 2, 3]);

    let invalid_vec = NonEmptyVec::try_new(Vec::<i32>::new());
    assert!(invalid_vec.is_err());
}
