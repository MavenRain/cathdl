//! Widget library ported from RustHDL / RHDL
//! Contains common hardware components (FIFO, UART, debouncer, etc.)

use crate::Circuit;

/// A simple LED blinker widget
#[derive(Clone, Default)]
pub struct Blinker {
    pub period: u32,
}

impl Circuit for Blinker {
    type Input = ();
    type State = u32;  // counter

    fn update(&self, _input: (), current: u32) -> comp_cat_rs::effect::io::Io<crate::types::CircuitError, u32> {
        let next = if current >= self.period {
            0
        } else {
            current + 1
        };
        comp_cat_rs::effect::io::Io::pure(next)
    }

    fn to_verilog(&self) -> String {
        format!(r#"module Blinker (
    input wire clk,
    output reg led
);
    reg [31:0] counter;
    always @(posedge clk) begin
        if (counter >= {}) begin
            counter <= 0;
            led <= ~led;
        end else begin
            counter <= counter + 1;
        end
    end
endmodule"#, self.period)
    }
}
