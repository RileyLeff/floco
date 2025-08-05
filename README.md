# floco 🛟 &emsp; ![Build Status] [![Codecov Badge]][Codecov Info] [![Apache Badge]][Apache Link] [![MIT Badge]][MIT Link]

[Build Status]: https://github.com/rileyleff/floco/actions/workflows/rust.yml/badge.svg
[Codecov Badge]: https://codecov.io/gh/RileyLeff/floco/graph/badge.svg?token=CEAG74DDK9
[Codecov Info]: https://codecov.io/gh/RileyLeff/floco
[MIT Badge]: https://img.shields.io/badge/License-MIT-yellow.svg
[MIT Link]: https://opensource.org/licenses/MIT
[Apache Badge]: https://img.shields.io/badge/License-Apache_2.0-blue.svg
[Apache Link]: https://opensource.org/licenses/Apache-2.0

## Floco: Constrained Types for Safe, Ergonomic Rust

Floco is a highly ergonomic Rust library for creating new, validated types from
existing ones. At its core, it provides a macro, `constrained_type!`, that
acts as a "type factory," allowing you to define custom constraints for any value.

This lets you elevate simple data like `f64` or `i32` into rich, domain-specific
types like `Percentage`, `PositiveF64`, or `SpeedLimit` that are guaranteed
to be valid at the type level.

The library is designed to prevent invalid data from ever being created, catching
errors at the earliest possible moment and using Rust's powerful type system to
enforce business logic and physical constraints throughout your program.

#### Features

*   **Type Safety:** Create new types with custom validation rules, ensuring that invalid data is never created.
*   **Ergonomic API:** The `constrained_type!` macro is designed to be intuitive and easy to use.
*   **`serde` Support:** Automatically derive `Serialize` and `Deserialize` for your constrained types.
*   **Arithmetic Operations:** Implement standard arithmetic operations for your types with the `impl_arithmetic_ops!` macro.
*   **Dimensional Analysis:** Use the `impl_dimensional_ops!` macro to perform operations between different constrained types, perfect for working with libraries like `uom`.
*   **`const` Validation:** With the `const-validation` feature, you can validate default values at compile time.

#### Design Philosophy

The core idea behind `floco` is to make invalid states unrepresentable. By creating new types with specific constraints, you can leverage Rust's type system to ensure that your data is always valid. This approach helps to catch errors at compile time, rather than at runtime, leading to more robust and reliable code.

### Examples

A simple type, with its default value validated at compile-time:

```rust
use floco::constrained_type;

constrained_type! {
    pub type Percentage(f64) where |p| p >= 0.0 && p <= 100.0,
    "Value must be a valid percentage.",
    default: 0.0
}

let half = Percentage::try_new(50.0).unwrap();
assert_eq!(*half, 50.0);

let invalid = Percentage::try_new(150.0);
assert!(invalid.is_err());

let zero = Percentage::default();
assert_eq!(*zero, 0.0);
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
