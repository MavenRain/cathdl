use crate::circuit::Circuit;
use crate::types::{Bit, CircuitError};
use comp_cat_rs::effect::io::Io;

#[derive(Clone, Default)]
pub struct Dff;

impl Circuit for Dff {
    type Input = Bit;
    type State = Bit;

    fn update(&self, input: Bit, _current: Bit) -> Io<CircuitError, Bit> {
        Io::pure(input)
    }

    fn to_verilog(&self) -> String {
        r#"module Dff (
    input wire clk,
    input wire d,
    output reg q
);
    always @(posedge clk) begin
        q <= d;
    end
endmodule
"#.to_string()
    }
}
