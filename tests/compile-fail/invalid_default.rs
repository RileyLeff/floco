#![cfg(all(feature = "const-validation", not(feature = "stable")))]
#![feature(const_trait_impl)]

use floco::constrained_type;

// This test ensures that the build fails when a default value is invalid.
constrained_type! {
    pub type InvalidDefault(i32) where |i| *i > 0, "Value must be positive.", default: -1
}
