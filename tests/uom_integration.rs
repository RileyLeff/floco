// tests/uom_integration.rs

// This entire file will only be compiled and run if the feature is enabled.
#![cfg(feature = "runtime-defaults")]
#![feature(associated_type_defaults)]

use floco::{constrained_type, impl_dimensional_ops};
use uom::si::f64::{Length, Time, Velocity};
use uom::si::length::meter;
use uom::si::time::second;
use uom::si::velocity::meter_per_second;
use uom::ConstZero;

// CORRECTED: The `*` has been removed from the where clauses.
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

impl_dimensional_ops!(PositiveLength, Div, div, PositiveTime => SafeVelocity);

#[test]
fn test_uom_default() {
    let default_len = PositiveLength::default();
    assert_eq!(*default_len, Length::new::<meter>(1.0)); // The `*` is correct here, because you're dereferencing the newtype!
}

#[test]
fn test_uom_dimensional_math() {
    let length = PositiveLength::try_new(Length::new::<meter>(10.0)).unwrap();
    let time = PositiveTime::try_new(Time::new::<second>(2.0)).unwrap();

    let velocity_result = length / time;
    assert!(velocity_result.is_ok());
    let velocity = velocity_result.unwrap();
    assert_eq!(velocity.get(), Velocity::new::<meter_per_second>(5.0));
}
