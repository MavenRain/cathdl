use crate::circuit::Circuit;
use crate::types::{Bit, CircuitError};
use comp_cat_rs::effect::io::Io;

#[derive(Clone, Default)]
pub struct Dff;

impl Circuit for Dff {
    type Input = Bit;
    type State = Bit;

    fn update(&self, input: Bit, _current: Bit) -> Io<CircuitError, Bit> {
        // On clock, output becomes the input (D flip flop)
        Io::pure(input)
    }
}
