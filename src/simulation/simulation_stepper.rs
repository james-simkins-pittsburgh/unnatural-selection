use bevy::prelude::*;

// This code runs one step of the simulation.
pub fn step_simulation(
    deterministic_trig: Res<crate::utility_functions::deterministic_trigonometry::DeterministicTrig>,
    mut gameworld: Query<
        (
            &mut crate::simulation::AllBiosphereInformation,
            &mut crate::simulation::AllCurrentInformation,
            &crate::simulation::AllMapInformation,
            &crate::simulation::AllSpeciesInformation,
        )
    >
) {
    for (mut biosphere, mut current, map_info, species_info) in &mut gameworld {



        
    }
}
