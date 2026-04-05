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
    type Input = bool;  // tick
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

    fn to_verilog(&self) -> String {
        r#"module TrafficLight (
    input wire clk,
    input wire tick,
    output reg [1:0] state
);
    localparam RED = 0, GREEN = 1, YELLOW = 2;

    always @(posedge clk) begin
        if (tick) begin
            case (state)
                RED: state <= GREEN;
                GREEN: state <= YELLOW;
                YELLOW: state <= RED;
            endcase
        end
    end
endmodule
"#.to_string()
    }
}
