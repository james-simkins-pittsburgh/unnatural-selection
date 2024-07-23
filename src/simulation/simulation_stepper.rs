use bevy::prelude::*;
use deterministic_trigonometry;

pub fn step_simulation(
    deterministic_trig: Res<crate::utility_functions::DeterministicTrig>,
    mut gameworld: Query<(&mut crate::simulation::AllBiosphereInformation, &mut crate::simulation::AllCurrentInformation, &mut crate::simulation::AllMapInformation, &mut crate::simulation::AllSpeciesInformation)>

) {

}