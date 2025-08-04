// tests/integrations.rs

// Conditionally apply experimental features ONLY when not in runtime mode.
#![cfg_attr(not(feature = "runtime-defaults"), feature(const_trait_impl))]
#![cfg_attr(not(feature = "runtime-defaults"), feature(const_default))]
#![feature(associated_type_defaults)]

// Import the necessary items from our library.
use floco::constrained_type;

// Define types using the robust newtype-generating macro.
constrained_type! { pub type PositiveF64(f64) where |val| val > 0.0, "Value must be positive.", default: 1.0 }
constrained_type! { pub type NonNegativeI32(i32) where |val| val >= 0, "Value must be non-negative." }
constrained_type! { pub type Percentage(f64) where |val| val >= 0.0 && val <= 100.0, "Value must be between 0.0 and 100.0.", default: 0.0 }

#[test]
fn test_creation_and_validation() {
    assert!(Percentage::try_new(50.0).is_ok());
    assert!(Percentage::try_new(101.0).is_err());
}

#[test]
fn test_deref_and_get() {
    let p = Percentage::try_new(42.0).unwrap();
    assert_eq!(*p, 42.0);
    assert_eq!(p.get(), 42.0);
}

#[test]
fn test_default() {
    assert_eq!(*Percentage::default(), 0.0);
    assert_eq!(*PositiveF64::default(), 1.0);
    assert_eq!(*NonNegativeI32::default(), 0);
}

#[test]
fn test_serde_integration() {
    let p = Percentage::try_new(55.5).unwrap();
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, "55.5");
    let deserialized: Percentage = serde_json::from_str(&json).unwrap();
    assert_eq!(*deserialized, 55.5);
    let invalid_json = "101.0";
    let err = serde_json::from_str::<Percentage>(invalid_json);
    assert!(err.is_err());
}

// ---- Feature-Gated Tests for `runtime-defaults` ----

#[cfg(feature = "runtime-defaults")]
mod uom_tests {
    use super::*;
    // --- THE FINAL FIX ---
    // The `#[macro_export]` attribute hoists macros to the crate root.
    // The correct way to import it is directly from the crate.
    use floco::impl_dimensional_ops;
    use uom::si::f64::{Length, Time, Velocity};
    use uom::si::length::meter;
    use uom::si::time::second;
    use uom::si::velocity::meter_per_second; // Import the specific unit for the assertion.
    use uom::ConstZero;

    constrained_type! {
        pub type PositiveLength(Length) where |l| l >= Length::ZERO,
        "Length must be non-negative.", default: Length::new::<meter>(1.0)
    }
    constrained_type! {
        pub type PositiveTime(Time) where |t| t > Time::ZERO,
        "Time must be strictly positive.", default: Time::new::<second>(1.0)
    }
    constrained_type! {
        pub type SafeVelocity(Velocity) where |v| v >= Velocity::ZERO,
        "Velocity must be non-negative.", default: Velocity::ZERO
    }

    // This will now be found and expanded correctly.
    impl_dimensional_ops!(PositiveLength, Div, div, PositiveTime => SafeVelocity);

    #[test]
    fn test_uom_default() {
        let default_len = PositiveLength::default();
        assert_eq!(*default_len, Length::new::<meter>(1.0));
    }

    #[test]
    fn test_uom_dimensional_math() {
        let length = PositiveLength::try_new(Length::new::<meter>(10.0)).unwrap();
        let time = PositiveTime::try_new(Time::new::<second>(2.0)).unwrap();

        // The `Div` impl now exists, so this will compile.
        let velocity_result = length / time;
        assert!(velocity_result.is_ok());
        let velocity = velocity_result.unwrap();
        assert_eq!(velocity.get(), Velocity::new::<meter_per_second>(5.0));
    }
}
