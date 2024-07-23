use bevy::prelude::*;

pub fn step_simulation(
    _deterministic_trig: Res<crate::utility_functions::DeterministicTrig>,
    mut _gameworld: Query<(&mut crate::simulation::AllBiosphereInformation, &mut crate::simulation::AllCurrentInformation, &mut crate::simulation::AllMapInformation, &mut crate::simulation::AllSpeciesInformation)>

) {

}