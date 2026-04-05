use crate::circuit::Circuit;
use crate::types::CircuitError;

pub fn simulate_steps<C>(
    circuit: C,
    mut state: C::State,
    input: C::Input,
    steps: usize,
) -> Result<Vec<C::State>, CircuitError>
where
    C: Circuit,
    C::State: Clone,
    C::Input: Clone,
{
    let mut history = vec![state.clone()];

    for _ in 0..steps {
        let next_state = circuit.update(input.clone(), state)
            .run()
            .map_err(|e| format!("Simulation error: {}", e))?;
        state = next_state;
        history.push(state.clone());
    }

    Ok(history)
}
