#![cfg(feature = "const-validation")]
#![feature(const_trait_impl)]

use floco::constrained_type;

// Use the new `for` syntax
constrained_type! {
    pub type CompileTimeChecked for i32 where |i| *i > 0, "Value must be positive.", default: 1
}

#[test]
fn test_const_validation_success() {
    let checked = CompileTimeChecked::default();
    assert_eq!(*checked, 1);
}