Here is a comprehensive summary of the floco library in its final, architecturally sound state.
Floco: A Summary
What is Floco?
Floco is a highly ergonomic Rust library for creating new, validated types from existing ones. At its core, it provides a macro, constrained_type!, that acts as a "type factory," allowing you to define custom constraints for any value. This lets you elevate simple data like f64 or i32 into rich, domain-specific types like Percentage, PositiveF64, or SpeedLimit that are guaranteed to be valid at the type level.
The library is designed to prevent invalid data from ever being created, catching errors at the earliest possible moment and using Rust's powerful type system to enforce business logic and physical constraints throughout your program.
The Core Philosophy: Safety First, Flexibility by Choice
The entire architecture of floco is built on a single, guiding principle: provide the strongest possible safety guarantees by default, while offering a clear, opt-in path for maximum flexibility when needed. This philosophy manifests in a hybrid validation model controlled by a feature flag:
Default Mode (Maximum Safety): By default, floco uses unstable const features from the nightly Rust compiler. This allows it to perform validation checks on default values at compile time. If you define a type with an invalid default, your program will not build. This is the ultimate guarantee of correctness for simple types.
Runtime Mode (Maximum Flexibility): When the runtime-defaults feature flag is enabled, the library removes all const dependencies. It validates default values at runtime, the first time a default is requested. If the default is invalid, the program panics immediately. This mode is essential for working with complex third-party types (like uom) whose functions are not const-compatible, allowing for powerful, dimensionally-aware constraints.
This dual-mode design puts the developer in control, allowing them to choose the right trade-off for their specific needs.
Key Architectural Pillars
The library's power stems from two fundamental design patterns that were discovered and refined to solve core Rust challenges:
The "Newtype" Pattern: The constrained_type! macro does not create simple type aliases. It generates a true, distinct struct that wraps the core Floco type (e.g., pub struct Percentage(Floco<f64, ...>);). This was the critical solution to Rust's "orphan rule," and it unlocks the library's full power. Because each new type is a local, distinct struct, users are free to implement any traits for them, such as custom arithmetic or serialization formats. For ergonomics, the macro automatically implements Deref (so *p works), transparent serde support, and standard arithmetic.
The Hybrid Trait System: The core Constrained trait is itself feature-gated. In the default mode, it is a #[const_trait], enabling compile-time checks. In runtime-defaults mode, it is a regular trait. The macros and impl blocks throughout the library respect this conditional compilation, ensuring that const logic is completely removed from the build when it is not supported.
Important Features in Practice
The constrained_type! Macro: This is the heart of the library. With a single, declarative macro call, you can define a new, validated type, specify its inner representation, provide its validation logic as a closure, and set its default value.
Generated rust
// A simple type, validated at compile-time by default.
constrained_type! {
    pub type Percentage(f64) where |p| p >= 0.0 && p <= 100.0,
    "Value must be a valid percentage.",
    default: 0.0
}
Use code with caution.
Rust
Dimensionally-Aware, Unit-Specific Constraints: The library fully supports defining constraints with physical units by leveraging uom. The validation logic is just Rust code, so you can provide complete uom::Quantity values as your constraints.
Generated rust
// This requires the `runtime-defaults` feature because `Velocity::new` is not const.
constrained_type! {
    pub type RoadSpeed(uom::si::f64::Velocity)
    where |v| *v >= Velocity::ZERO && *v <= Velocity::new::<kilometer_per_hour>(130.0),
    "Speed must be between 0 and 130 km/h."
}
Use code with caution.
Rust
uom's internal logic automatically handles unit conversions, so a RoadSpeed created with meters-per-second will be correctly validated against the kilometer-per-hour limit.
Extensible Dimensional Arithmetic: The impl_dimensional_ops! macro allows users to define safe arithmetic between their newtypes, enabling powerful dimensional analysis. Because the newtype pattern solves the orphan rule, users can freely implement these operations in their own code.
Generated rust
// User-defined relationship: Force = Mass * Acceleration
impl_dimensional_ops!(ConstrainedMass, Mul, mul, ConstrainedAcceleration => ConstrainedForce);
Use code with caution.
Rust
Seamless and Safe serde Integration: The generated newtypes derive Serialize and Deserialize transparently. Crucially, deserialization is intrinsically linked to validation. If you attempt to deserialize data that violates a type's constraints, Serde will return an error, providing a powerful safety net at your program's boundaries.