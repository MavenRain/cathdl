use crate::circuit::Circuit;
use crate::types::{Bit, CircuitError};
use comp_cat_rs::effect::io::Io;

#[derive(Clone, Default)]
pub struct Counter;

impl Circuit for Counter {
    type Input = Bit;  // enable
    type State = u32;

    fn update(&self, enable: Bit, current: u32) -> Io<CircuitError, u32> {
        let next = if enable.to_bool() {
            current.wrapping_add(1)
        } else {
            current
        };
        Io::pure(next)
    }
}
