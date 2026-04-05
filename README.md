# CathDL

A hardware description library built on comp-cat-rs 0.5 using categorical effects (Io monad).

## Features
- Circuits defined via the Circuit trait with Io-based updates
- Functional style inspired by RHDL
- Simulation via simulate_steps
- Full Rustdoc support

## Building Documentation

To generate the doc webpage:

```bash
cargo doc --open
```

This opens a local webpage with all module and function documentation.

For CI/CD, we have a GitHub workflow that builds the docs.

## Example

```rust
use cathdl::{Bit, Dff, Counter, simulate_steps};

fn main() {
    if let Err(e) = run_simulation() {
        eprintln!("Simulation error: {}", e);
        return;
    }
    println!("Simulation completed.");
}

fn run_simulation() -> Result<(), String> {
    let dff = Dff;
    let states = simulate_steps(dff, Bit::Low, Bit::High, 3)
        .map_err(|e| e.to_string())?;
    println!("{:?}", states);

    let counter = Counter;
    let counts = simulate_steps(counter, 0u32, Bit::High, 5)
        .map_err(|e| e.to_string())?;
    println!("{:?}", counts);

    Ok(())
}
```

## Next steps
- Waveform generation using Stream
- Synthesis backend
- Macros for nicer syntax
