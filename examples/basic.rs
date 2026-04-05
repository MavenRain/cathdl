use cathdl::{Bit, Dff, Counter, Light, TrafficLight, simulate_steps};

fn main() {
    println!("CathDL Example - HDL with comp-cat effects");

    // DFF test
    let dff = Dff;
    let dff_states = simulate_steps(dff, Bit::Low, Bit::High, 4).unwrap();
    println!("DFF states: {:?}", dff_states);

    // Counter test
    let counter = Counter;
    let counts = simulate_steps(counter, 0u32, Bit::High, 5).unwrap();
    println!("Counter: {:?}", counts);

    // FSM with enum
    let light = TrafficLight;
    let lights = simulate_steps(light, Light::Red, true, 6).unwrap();
    println!("Traffic lights: {:?}", lights);

    println!("\nAll simulations completed successfully!");
    println!("This shows RHDL-style hardware in Rust using comp-cat Io monad.");
}
