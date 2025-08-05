// tests/arithmetic_ops.rs

#![cfg_attr(not(feature = "runtime-defaults"), feature(const_trait_impl))]
// CORRECTED: Add the missing feature flag
#![cfg_attr(not(feature = "runtime-defaults"), feature(const_default))]
#![feature(associated_type_defaults)]

use floco::{constrained_type, impl_arithmetic_ops};

constrained_type! {
    pub type LimitedInt(i32) where |i| i >= -100 && i <= 100,
    "Value must be between -100 and 100."
}

impl_arithmetic_ops!(LimitedInt, Add, add, Sub, sub);

// ... (the rest of the file is correct and does not need to change)
#[test]
fn test_same_type_arithmetic() {
    let val_50 = LimitedInt::try_new(50).unwrap();
    let val_20 = LimitedInt::try_new(20).unwrap();

    // --- Test successful addition ---
    let sum = (val_50 + val_20).unwrap();
    assert_eq!(sum.get(), 70);

    // --- Test successful subtraction ---
    let diff = (val_50 - val_20).unwrap();
    assert_eq!(diff.get(), 30);

    // --- Test FAILING addition (violates constraint) ---
    let val_80 = LimitedInt::try_new(80).unwrap();
    let failed_sum_result = val_50 + val_80; // 50 + 80 = 130, which is > 100
    assert!(failed_sum_result.is_err());
    assert_eq!(failed_sum_result.unwrap_err().value, 130);

    // --- Test FAILING subtraction (violates constraint) ---
    let val_neg_80 = LimitedInt::try_new(-80).unwrap();
    let failed_diff_result = val_neg_80 - val_50; // -80 - 50 = -130, which is < -100
    assert!(failed_diff_result.is_err());
    assert_eq!(failed_diff_result.unwrap_err().value, -130);
}
