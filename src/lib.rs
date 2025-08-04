// src/lib.rs

//! (Docs will go here)

#![warn(missing_docs)]
#![no_std]
// --- THE FIX ---
// Conditionally apply the experimental feature flags ONLY when we are NOT
// building with the `runtime-defaults` feature.
#![cfg_attr(not(feature = "runtime-defaults"), feature(const_trait_impl))]
#![cfg_attr(not(feature = "runtime-defaults"), feature(const_default))]
#![feature(associated_type_defaults)]

/// Contains all procedural macros for defining new types and operations.
pub mod macros;
/// Contains the core data structures and traits.
pub mod types;

// Re-export the public types for ergonomic access.
pub use crate::types::*;
