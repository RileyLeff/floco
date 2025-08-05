### `plan.md`

# Floco Development Plan: The Ergonomics & Power Update

This document outlines the plan for evolving `floco` from a functional library into a highly ergonomic and powerful tool for creating safe, constrained types in Rust. The plan is based on discussions that identified key areas for improvement in developer experience and feature set.

The project is currently in a clean, working state. All tests for the core library and the `runtime-defaults` feature are passing. The following is a roadmap for new features.

## Tier 1: Core Ergonomic Improvements (High Priority)

These features address the most common use cases and will provide the largest immediate benefit to users.

### 1. Implement `impl_arithmetic_ops!` Macro

*   **Motivation:** The existing `impl_dimensional_ops!` is poorly suited for the common case of same-type arithmetic (e.g., `A + A`). Users need a simple, one-line way to implement standard math operators for their newtypes without the cognitive overhead of defining a separate output type.
*   **Purpose:** To provide a convenient macro for implementing arithmetic operations where the output type is the same as the input types (`+`, `-`, `*`, `/`). The result of each operation must be re-validated against the type's own constraints.
*   **Proposed Signature:** `impl_arithmetic_ops!(TypeName, Add, add, Sub, sub, ...);`
*   **Action:**
    1.  Create a new file: `src/macros/arithmetic_ops.rs`.
    2.  Define the `impl_arithmetic_ops!` macro inside this file.
    3.  Add `pub mod arithmetic_ops;` to `src/macros/mod.rs`.
    4.  Create a new integration test file, `tests/arithmetic_ops.rs`, to verify its functionality, including success and failure (constraint violation) cases.

### 2. Refactor to Support Non-`Copy` Types

*   **Motivation:** The library is currently constrained by a `T: Copy` bound on its core traits, limiting its use to simple numeric types. Removing this makes `floco` a truly general-purpose validation library for types like `String`, `Vec<T>`, `url::Url`, etc.
*   **Purpose:** To remove the `Copy` bound in favor of `Clone` throughout the library, making it compatible with non-`Copy`, heap-allocated types.
*   **Action:**
    1.  **In `src/types.rs`:**
        *   Modify the `where` clauses on `Floco`'s `impl` blocks and the `Constrained` trait definitions to require `T: Clone` instead of `T: Copy`.
        *   Change `#[derive(Debug, Copy, Clone)]` on `Floco` and `ValidationError` to `#[derive(Debug, Clone)]`.
        *   Add `PartialEq` and `Eq` to the `Floco` derive macro (`#[derive(Debug, Clone, PartialEq, Eq)]`) so that user newtypes can derive them.
    2.  **In `src/macros/constrained_type.rs`:**
        *   Change `#[derive(Debug, Copy, Clone, ...)]` on the generated newtype to `#[derive(Debug, Clone, ...)]`.
        *   Change `fn get(&self) -> InnerType` to `fn get(&self) -> &InnerType`.
        *   Add a new `fn into_inner(self) -> InnerType` method to consume the wrapper and return the value.
        *   Update the `impl From` to use `into_inner()`.
        *   Update the macro's handling of default values to use `.clone()` to support non-`Copy` defaults.
    3.  **Update All Existing Tests:**
        *   Fix all calls to `.get()` to dereference the result (e.g., `*my_val.get()`).
        *   Add `.clone()` to values used in arithmetic tests to make ownership semantics clear.
    4.  **Create a New Integration Test:**
        *   Add `tests/non_copy_types.rs` to explicitly test creating and using a `floco` type that wraps `String`.

## Tier 2: Documentation Overhaul (Medium Priority)

Once the core features are implemented, we will update the documentation to reflect the new capabilities and provide clear user guidance.

*   **Action: Create a "Guide to Operations" in the Docs:**
    *   Explain the difference between `impl_arithmetic_ops!` and `impl_dimensional_ops!`.
    *   Provide clear examples for same-type math, cross-type math, and direct implementation of unary functions (`.sqrt()`, etc.).
*   **Action: Create a "Working with `uom`" Guide:**
    *   Explain the concept of wrapping abstract **quantities** vs. specific units.
    *   Show a complete example with a complex unit.
*   **Action: Fix the `lib.rs` Doctest:**
    *   Ensure the main example in the library's documentation compiles and runs correctly with `cargo test`. This requires adding `#![feature(...)]` lines prefixed with `#` to the code block.