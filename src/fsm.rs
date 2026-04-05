use crate::circuit::Circuit;
use crate::types::CircuitError;
use comp_cat_rs::effect::io::Io;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Light {
    Red,
    Yellow,
    Green,
}

#[derive(Clone, Default)]
pub struct TrafficLight;

impl Circuit for TrafficLight {
    type Input = bool;  // enable/tick
    type State = Light;

    fn update(&self, tick: bool, current: Light) -> Io<CircuitError, Light> {
        let next = if !tick {
            current
        } else {
            match current {
                Light::Red => Light::Green,
                Light::Green => Light::Yellow,
                Light::Yellow => Light::Red,
            }
        };
        Io::pure(next)
    }
}
