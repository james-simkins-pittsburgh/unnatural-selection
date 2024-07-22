use bevy::prelude::*;
use deterministic_trigonometry;

pub fn step_simulation(
    mut deterministic_trig: Res<crate::utility_functions::DeterministicTrig>
) {

    let sine_a = deterministic_trig.d_trig.sine((500,1000));

}