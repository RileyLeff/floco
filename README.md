# floco 🛟 &emsp; ![Build Status] [![Codecov Badge]][Codecov Info] [![Apache Badge]][Apache Link] [![MIT Badge]][MIT Link]

[Build Status]: https://github.com/rileyleff/floco/actions/workflows/rust.yml/badge.svg
[Codecov Badge]: https://codecov.io/gh/RileyLeff/floco/graph/badge.svg?token=CEAG74DDK9
[Codecov Info]: https://codecov.io/gh/RileyLeff/floco
[MIT Badge]: https://img.shields.io/badge/License-MIT-yellow.svg
[MIT Link]: https://opensource.org/licenses/MIT
[Apache Badge]: https://img.shields.io/badge/License-Apache_2.0-blue.svg
[Apache Link]: https://opensource.org/licenses/Apache-2.0

Floco validates ***flo***ats against user-defined ***co***nstraints.

## Quick Start with the Macro

```rust
// These feature flags are required for the doctest to compile,
// as it's treated as a separate crate.
#![feature(associated_type_defaults)]
#![feature(const_trait_impl)]
#![feature(const_default)]

use floco::{constrained_type, Floco, Constrained};

// The macro generates the marker struct, the trait impl, and a type alias for you.
constrained_type! {
    /// A value representing a percentage, must be between 0.0 and 100.0.
    pub type Percentage(f64) where |val| val >= 0.0 && val <= 100.0,
    "Value must be a valid percentage [0.0, 100.0]"
}

// Now we can use our new type `Percentage`
let ok = Percentage::try_new(99.5);
assert!(ok.is_ok());

let err = Percentage::try_new(101.0);
assert!(err.is_err());
// The default error is rich and informative!
println!("{}", err.unwrap_err());
```

## Overview

This crate provides a struct that wraps a floating-point number alongside a PhantomData marker
type. The marker type defines arbitrary validation conditions for the inner float.
These validation conditions are invoked during construction, conversion, and deserialization.

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
