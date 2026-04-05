//! CathDL - Hardware Description Library using comp-cat categorical effects.
//!
//! This library allows describing digital circuits in Rust using the `Io` monad
//! from comp-cat-rs for functional, testable hardware models.

pub mod types;
pub mod circuit;
pub mod register;
pub mod counter;
pub mod fsm;
pub mod sim;

pub use types::{Bit, CircuitError, Signal};
pub use circuit::Circuit;
pub use register::Dff;
pub use counter::Counter;
pub use fsm::{Light, TrafficLight};
pub use sim::simulate_steps;
