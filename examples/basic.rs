use cathdl::{Bit, Circuit, Dff, Counter, Light, TrafficLight, simulate_steps};

fn main() {
    println!("CathDL Example - HDL with comp-cat effects + Verilog generation\n");

    if let Err(e) = run_examples() {
        eprintln!("Error: {}", e);
    } else {
        println!("\nAll simulations and Verilog generation completed successfully!");
    }
}

fn run_examples() -> Result<(), String> {
    let dff = Dff;
    let dff_states = simulate_steps(dff.clone(), Bit::Low, Bit::High, 4)
        .map_err(|e| e.to_string())?;
    println!("DFF states: {:?}", dff_states);
    println!("\n--- DFF Verilog ---");
    println!("{}", dff.to_verilog());

    let counter = Counter;
    let counts = simulate_steps(counter.clone(), 0u32, Bit::High, 5)
        .map_err(|e| e.to_string())?;
    println!("Counter: {:?}", counts);
    println!("\n--- Counter Verilog ---");
    println!("{}", counter.to_verilog());

    let light = TrafficLight;
    let lights = simulate_steps(light.clone(), Light::Red, true, 6)
        .map_err(|e| e.to_string())?;
    println!("Traffic lights: {:?}", lights);
    println!("\n--- TrafficLight Verilog ---");
    println!("{}", light.to_verilog());

    Ok(())
}
