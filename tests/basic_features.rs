// tests/basic_features.rs

#![cfg_attr(not(feature = "runtime-defaults"), feature(const_trait_impl))]
#![cfg_attr(not(feature = "runtime-defaults"), feature(const_default))]
#![feature(associated_type_defaults)]

use floco::constrained_type;

// CORRECTED: No `*` on `val` or `p` for the primitive f64 and i32 types.
constrained_type! { pub type PositiveF64(f64) where |val| val > 0.0, "Value must be positive.", default: 1.0 }
constrained_type! { pub type NonNegativeI32(i32) where |i| i >= 0, "Value must be non-negative." }
constrained_type! { pub type Percentage(f64) where |p| p >= 0.0 && p <= 100.0, "Value must be between 0.0 and 100.0.", default: 0.0 }

// ... (the rest of the file is correct and does not need to change)
#[test]
fn test_creation_and_validation() {
    assert!(Percentage::try_new(50.0).is_ok());
    assert!(Percentage::try_new(101.0).is_err());
}

#[test]
fn test_deref_and_get() {
    let p = Percentage::try_new(42.0).unwrap();
    assert_eq!(*p, 42.0);
    assert_eq!(p.get(), 42.0);
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
