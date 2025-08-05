// tests/arithmetic_ops.rs

#![cfg_attr(feature = "const-validation", feature(const_trait_impl))]
#![cfg_attr(feature = "const-validation", feature(const_default))]
#![feature(associated_type_defaults)]

use core::fmt::Debug;
use floco::{constrained_type, impl_arithmetic_ops};

constrained_type! {
    pub type LimitedInt for i32 where |i| *i >= -100 && *i <= 100,
    "Value must be between -100 and 100."
}

impl_arithmetic_ops!(LimitedInt, Add, add, Sub, sub, Mul, mul, Div, div);

#[test]
fn test_same_type_arithmetic() {
    let val_50 = LimitedInt::try_new(50).unwrap();
    let val_20 = LimitedInt::try_new(20).unwrap();
    let val_100 = LimitedInt::try_new(100).unwrap();
    let val_neg_100 = LimitedInt::try_new(-100).unwrap();

    // --- Test successful addition ---
    let sum = (val_50.clone() + val_20.clone()).unwrap();
    assert_eq!(sum.get(), &70);

    // --- Test successful subtraction ---
    let diff = (val_50.clone() - val_20.clone()).unwrap();
    assert_eq!(diff.get(), &30);

    // --- Test successful multiplication ---
    let product = (val_20.clone() * LimitedInt::try_new(5).unwrap()).unwrap();
    assert_eq!(product.get(), &100);

    // --- Test successful division ---
    let quotient = (val_100.clone() / LimitedInt::try_new(2).unwrap()).unwrap();
    assert_eq!(quotient.get(), &50);

    // --- Test FAILING addition (violates constraint) ---
    let failed_sum_result = val_50.clone() + val_100.clone();
    assert!(failed_sum_result.is_err());
    assert_eq!(failed_sum_result.unwrap_err().value, 150);

    // --- Test FAILING subtraction (violates constraint) ---
    let failed_diff_result = val_neg_100.clone() - val_50.clone();
    assert!(failed_diff_result.is_err());
    assert_eq!(failed_diff_result.unwrap_err().value, -150);

    // --- Test FAILING multiplication (violates constraint) ---
    let failed_product_result = val_20.clone() * LimitedInt::try_new(6).unwrap();
    assert!(failed_product_result.is_err());
    assert_eq!(failed_product_result.unwrap_err().value, 120);

    // --- Test FAILING division (by zero) ---
    let division_by_zero_result = val_100 / LimitedInt::try_new(0).unwrap();
    assert!(division_by_zero_result.is_err());
}
