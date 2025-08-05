// src/lib.rs

//! # Floco: Constrained Types for Safe, Ergonomic Rust
//!
//! Floco is a highly ergonomic Rust library for creating new, validated types from
//! existing ones. At its core, it provides a macro, `constrained_type!`, that
//! acts as a "type factory," allowing you to define custom constraints for any value.
//!
//! This lets you elevate simple data like `f64` or `i32` into rich, domain-specific
//! types like `Percentage`, `PositiveF64`, or `SpeedLimit` that are guaranteed
//! to be valid at the type level.
//!
//! The library is designed to prevent invalid data from ever being created, catching
//! errors at the earliest possible moment and using Rust's powerful type system to
//! enforce business logic and physical constraints throughout your program.
//!
//! ## Examples
//!
//! A simple type, with its default value validated at compile-time:
//!
//! ## Examples
//!
//! A simple type, with its default value validated at compile-time:
//!
//! ```rust,ignore
//! use floco::constrained_type;
//!
//! constrained_type! {
//!     pub type Percentage(f64) where |p| p >= 0.0 && p <= 100.0,
//!     "Value must be a valid percentage.",
//!     default: 0.0
//! }
//!
//! let half = Percentage::try_new(50.0).unwrap();
//! assert_eq!(*half, 50.0);
//!
//! let invalid = Percentage::try_new(150.0);
//! assert!(invalid.is_err());
//!
//! let zero = Percentage::default();
//! assert_eq!(*zero, 0.0);
//! ```

#![warn(missing_docs)]
#![no_std]
#![cfg_attr(not(feature = "runtime-defaults"), feature(const_trait_impl))]
#![cfg_attr(not(feature = "runtime-defaults"), feature(const_default))]
#![feature(associated_type_defaults)]

// This loads everything from `src/macros.rs` and makes it available.
#[macro_use]
mod macros;

/// Contains the core data structures and traits.
pub mod types;

// Re-export the public types for ergonomic access.
pub use crate::types::*;
