use bevy::prelude::*;
use crate::simulation::current_simulation::simulate_currents;
use super::biosphere_simulation::simulate_biosphere;

// This code runs one step of the simulation.
pub fn step_simulation(
    deterministic_trig: Res<crate::utility_functions::deterministic_trigonometry::DeterministicTrig>,
    game_settings: Res<crate::settings::GameSettings>,
    mut gameworld: Query<
        (
            &mut crate::simulation::AllBiosphereInformation,
            &mut crate::simulation::AllCurrentInformation,
            &mut crate::simulation::CheapRandomGameworld,
            &crate::simulation::AllSpeciesInformation,
            &mut crate::simulation::AdministrativeInformation,
        )
    >
) {
    for (
        mut biosphere,
        mut current,
        mut cheap_random,
        species_info,
        mut admin_info,
    ) in &mut gameworld {
        // This simulates all current movements for the step of the simulation.
        simulate_currents(
            &mut current,
            &admin_info,
            &mut cheap_random,
            &game_settings,
        );
        // This simulates all biosphere activity for the step of the simulation.
        simulate_biosphere(
            &mut biosphere,
            &species_info,
            &current,
            &admin_info,
            &mut cheap_random,
            &deterministic_trig,
            &game_settings
        );
        // This increases the simulation tick counter by 1.
        admin_info.tick_counter = admin_info.tick_counter + 1;
    }
}
