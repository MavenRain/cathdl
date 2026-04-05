use crate::circuit::Circuit;
use crate::types::CircuitError;

pub fn simulate_steps<C>(
    circuit: C,
    initial_state: C::State,
    input: C::Input,
    steps: usize,
) -> Result<Vec<C::State>, CircuitError>
where
    C: Circuit,
    C::State: Clone,
    C::Input: Clone,
{
    let mut history = vec![initial_state.clone()];
    let mut state = initial_state;

    for _ in 0..steps {
        let next_state = circuit.update(input.clone(), state)
            .run()
            .map_err(|e| format!("Simulation error: {}", e))?;
        state = next_state;
        history.push(state.clone());
    }

    Ok(history)
}
