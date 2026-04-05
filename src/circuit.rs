use crate::types::CircuitError;
use comp_cat_rs::effect::io::Io;

pub trait Circuit {
    type Input;
    type State: Clone;

    fn update(&self, input: Self::Input, current: Self::State) -> Io<CircuitError, Self::State>;

    fn to_verilog(&self) -> String;
}
