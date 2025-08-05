# Floco Showcase

This document provides a showcase of `floco`'s features with detailed examples and code snippets.

## Basic Usage: Creating a Constrained Type

The core of `floco` is the `constrained_type!` macro. Here's a simple example of creating a `Percentage` type:

```rust
use floco::constrained_type;

constrained_type! {
    pub type Percentage(f64) where |p| *p >= 0.0 && *p <= 100.0,
    "Value must be a valid percentage.",
    default: 0.0
}

fn main() {
    let half = Percentage::try_new(50.0).unwrap();
    assert_eq!(*half, 50.0);

    let invalid = Percentage::try_new(150.0);
    assert!(invalid.is_err());

    let zero = Percentage::default();
    assert_eq!(*zero, 0.0);
}
```

## `serde` Integration

`floco` types automatically derive `Serialize` and `Deserialize`, making them easy to use with `serde`.

```rust
use floco::constrained_type;

constrained_type! {
    pub type Percentage(f64) where |p| *p >= 0.0 && *p <= 100.0,
    "Value must be a valid percentage.",
    default: 0.0
}

fn main() {
    let p = Percentage::try_new(55.5).unwrap();
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, "55.5");

    let deserialized: Percentage = serde_json::from_str(&json).unwrap();
    assert_eq!(*deserialized, 55.5);

    let invalid_json = "101.0";
    let err = serde_json::from_str::<Percentage>(invalid_json);
    assert!(err.is_err());
}
```

## Working with `String` and Other `Clone` Types

`floco` now supports non-`Copy` types like `String` out of the box. This allows you to create constrained types for a wide variety of data structures.

### Non-Empty String

Here's how to create a `NonEmptyString` type that is guaranteed to not be empty:

```rust
use floco::constrained_type;

constrained_type! {
    pub type NonEmptyString(String) where |s| !s.is_empty(), "String must not be empty."
}

fn main() {
    let valid_str = NonEmptyString::try_new("hello".to_string()).unwrap();
    assert_eq!(valid_str.get(), "hello");

    let invalid_str = NonEmptyString::try_new("".to_string());
    assert!(invalid_str.is_err());
}
```

### Constrained `Vec`

You can also create constrained types for `Vec` and other collections:

```rust
use floco::constrained_type;

constrained_type! {
    pub type NonEmptyVec<T>(Vec<T>) where |v| !v.is_empty(), "Vector must not be empty."
}

fn main() {
    let valid_vec = NonEmptyVec::try_new(vec![1, 2, 3]).unwrap();
    assert_eq!(valid_vec.get(), &vec![1, 2, 3]);

    let invalid_vec = NonEmptyVec::try_new(Vec::<i32>::new());
    assert!(invalid_vec.is_err());
}
```

## Arithmetic Operations

`floco` provides macros to implement arithmetic operations for your constrained types.

### Same-Type Arithmetic

The `impl_arithmetic_ops!` macro implements standard arithmetic operations where the output is the same as the input type.

```rust
use floco::{constrained_type, impl_arithmetic_ops};

constrained_type! {
    pub type LimitedInt(i32) where |i| *i >= -100 && *i <= 100,
    "Value must be between -100 and 100."
}

impl_arithmetic_ops!(LimitedInt, Add, add, Sub, sub);

fn main() {
    let val_50 = LimitedInt::try_new(50).unwrap();
    let val_20 = LimitedInt::try_new(20).unwrap();

    let sum = (val_50.clone() + val_20.clone()).unwrap();
    assert_eq!(sum.get(), &70);

    let diff = (val_50 - val_20).unwrap();
    assert_eq!(diff.get(), &30);
}
```

### Dimensional Analysis with `uom`

The `impl_dimensional_ops!` macro allows you to implement operations between different constrained types, which is particularly useful for dimensional analysis with the `uom` crate.

```rust
# #![cfg(not(feature = "const-validation"))]
use floco::{constrained_type, impl_dimensional_ops};
use uom::si::f64::{Length, Time, Velocity};
use uom::si::length::meter;
use uom::si::time::second;
use uom::si::velocity::meter_per_second;
use uom::ConstZero;

constrained_type! {
    pub type PositiveLength(Length) where |l| *l >= Length::ZERO,
    "Length must be non-negative.", default: Length::new::<meter>(1.0)
}

constrained_type! {
    pub type PositiveTime(Time) where |t| *t > Time::ZERO,
    "Time must be strictly positive.", default: Time::new::<second>(1.0)
}

constrained_type! {
    pub type SafeVelocity(Velocity) where |v| *v >= Velocity::ZERO,
    "Velocity must be non-negative.", default: Velocity::ZERO
}

impl_dimensional_ops!(PositiveLength, Div, div, PositiveTime => SafeVelocity);

fn main() {
    let length = PositiveLength::try_new(Length::new::<meter>(10.0)).unwrap();
    let time = PositiveTime::try_new(Time::new::<second>(2.0)).unwrap();

    let velocity_result = length / time;
    assert!(velocity_result.is_ok());

    let velocity = velocity_result.unwrap();
    assert_eq!(velocity.get(), &Velocity::new::<meter_per_second>(5.0));
}
```
