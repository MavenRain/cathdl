#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bit {
    Low,
    High,
}

impl Bit {
    pub fn to_bool(self) -> bool {
        matches!(self, Bit::High)
    }

    pub fn from_bool(b: bool) -> Self {
        if b {
            Bit::High
        } else {
            Bit::Low
        }
    }
}

pub type Signal = Bit;
pub type CircuitError = String;
