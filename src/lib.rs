//! Floco validates ***flo***ats against user-defined ***co***nstraints.
//!
//! # Quick Start with the Macro
//!
//! ```
//! // These feature flags are required for the doctest to compile,
//! // as it's treated as a separate crate.
//! #![feature(associated_type_defaults)]
//! #![feature(const_trait_impl)]
//! #![feature(const_default)]
//!
//! use floco::{constrained_type, Floco, Constrained};
//!
//! // The macro generates the marker struct, the trait impl, and a type alias for you.
//! constrained_type! {
//!     /// A value representing a percentage, must be between 0.0 and 100.0.
//!     pub type Percentage(f64) where |val| val >= 0.0 && val <= 100.0,
//!     "Value must be a valid percentage [0.0, 100.0]"
//! }
//!
//! // Now we can use our new type `Percentage`
//! let ok = Percentage::try_new(99.5);
//! assert!(ok.is_ok());
//!
//! let err = Percentage::try_new(101.0);
//! assert!(err.is_err());
//! // The default error is rich and informative!
//! println!("{}", err.unwrap_err());
//! ```
//!
//! # Overview
//!
//! This crate provides a struct that wraps a floating-point number alongside a PhantomData marker
//! type. The marker type defines arbitrary validation conditions for the inner float.
//! These validation conditions are invoked during construction, conversion, and deserialization.

#![warn(missing_docs)]
#![no_std]
// Allows `type Error = ...` in the `Constrained` trait.
#![feature(associated_type_defaults)]
// Allows `const` items in traits, like `const DEFAULT: F` and `const fn is_valid`.
#![feature(const_trait_impl)]
#![feature(const_default)]

// Declare the `core` module, making its contents available within the crate.
pub mod core;

// Re-export the public API from the `core` module to the crate root.
// This allows users to `use floco::Floco` instead of `use floco::core::Floco`.
pub use crate::core::*;
