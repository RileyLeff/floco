# floco 🛟 &emsp; ![Build Status] [![Codecov Badge]][Codecov Info] [![Apache Badge]][Apache Link] [![MIT Badge]][MIT Link]

[Build Status]: https://github.com/rileyleff/floco/actions/workflows/rust.yml/badge.svg
[Codecov Badge]: https://codecov.io/gh/RileyLeff/floco/graph/badge.svg?token=CEAG74DDK9
[Codecov Info]: https://codecov.io/gh/RileyLeff/floco
[MIT Badge]: https://img.shields.io/badge/License-MIT-yellow.svg
[MIT Link]: https://opensource.org/licenses/MIT
[Apache Badge]: https://img.shields.io/badge/License-Apache_2.0-blue.svg
[Apache Link]: https://opensource.org/licenses/Apache-2.0

Floco validates ***flo***ats against user-defined ***co***nstraints.

## Quick Start

```rust
use floco::{Floco, Constrained};

// We want to represent a value as f64, but we don't want to allow:
//      - values below 5.0f64
//      - values above 7.2f64

// We define an empty struct.
// This won't contain data, but will contain validation criteria in its impl.
struct Foo;

// The Constrained trait defines the above constraints, an error type, and a default value.
impl Constrained<f64> for Foo {

    type Error = &'static str;

    fn is_valid(value: f64) -> bool {
        value >= 5.0f64 && value <= 7.2f64
    }

    fn emit_error(_value: f64) -> Self::Error {
        "yikes this is a bad foo"
    }

    // Optionally, you can set a custom default.
    // Floco::<F, YourType>::Default will respect the default value impl for YourType.
    fn get_default() -> f64 {
        5.2f64
     }
}

// Now we can use Foo to constrain a Floco
let this_will_be_ok = Floco::<f64, Foo>::try_new(6.8);
let this_will_be_err = Floco::<f64, Foo>::try_new(4.2);

```

## Overview

This crate provides a struct that wraps a floating-point number alongside a PhantomData marker
type. The marker type defines arbitrary validation conditions for the inner float.
These validation conditions are invoked during construction, conversion, and deserialization.

The marker type also provides an Error type and an optional default value (defaults to zero).

Floco is no_std compatible by default, but has optional support for the standard library behind
a feature flag. This doesn't add any functionality, just changes math ops from libm to std and
changes the errors from thiserror-core to thiserror. Floco should compile on stable if std is
enabled, but will require the [error_in_core][`eiclink`] feature for no_std builds.

Floco is compatible with any type that implements the [float][`ntFloatlink`] trait from
the num_traits crate. TryFrom conversions are implemented from f32 and f64 for
convenience.

## Roadmap
- At some point I intend to implement the ops traits on the Floco struct.
- At some point I intend to add a macro to reduce the newtype boilerplate.
- I want to create a similar struct that also contains generic [uom][`uomlink`] dimensions, but might just put that in a separate crate.
- Not sure what to do with the Copy trait. Need to think that through.

## Alternative / Related Crates
- [prae][`PraeLink`] uses a macro to create distinct types rather than making a single type generic across arbitrary marker impls. Prae is incompatible with no_std.
- [tightness][`TightnessLink`] is a predecessor to prae.
- [typed_floats][`TypedFloatLink`] provides 12 useful pre-made restricted float types. See the useful "Similar crates" section at the end of the TypedFloats readme.

## Inspired By
- [Tightness Driven Development in Rust][`TightnessPost`]
- [this stackoverflow comment][`SOComment`]
- [this reddit comment][`RedditComment`]

[`PraeLink`]: https://github.com/teenjuna/prae
[`TightnessLink`]: https://github.com/PabloMansanet/tightness
[`TypedFloatLink`]: https://github.com/tdelmas/typed_floats
[`TightnessPost`]: https://www.ecorax.net/tightness/
[`RedditComment`]: https://www.reddit.com/r/rust/comments/abmilm/bounded_numeric_types/ed1fs0f/
[`SOComment`]: https://stackoverflow.com/questions/57440412/implementing-constructor-function-in-rust-trait#comment101360200_57440412
[`eiclink`]:  https://github.com/rust-lang/rust/issues/103765
[`ntFloatlink`]: https://docs.rs/num-traits/latest/num_traits/float/trait.Float.html
[`uomlink`]: https://github.com/iliekturtles/uom

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
