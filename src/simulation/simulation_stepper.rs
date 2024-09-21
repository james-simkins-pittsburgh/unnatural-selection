use bevy::prelude::*;
use crate::simulation::current_simulation::simulate_currents;
use super::biosphere_simulation::simulate_spatial_biosphere;

// This code runs one step of the simulation for the spatial simulation
pub fn step_spatial_simulation(
    deterministic_trig: Res<crate::utility_functions::deterministic_trigonometry::DeterministicTrig>,
    game_settings: Res<crate::settings::GameSettings>,
    mut gameworld: Query<
        (
            &mut crate::simulation::AllSpatialBiosphereInformation,
            &mut crate::simulation::AllCurrentInformation,
            &mut crate::simulation::CheapRandomGameworld,
            &mut crate::simulation::AdministrativeInformation,
        )
    >
) {
    for (
        mut spatial_biosphere,
        mut current,
        mut cheap_random,
        mut admin_info,
    ) in &mut gameworld {
        // This simulates all current movements for the step of the simulation.
        simulate_currents(
            &mut current,
            &admin_info,
            &mut cheap_random,
            &game_settings,
        );
        // This simulates all spatial biosphere activity for the step of the simulation.
        simulate_spatial_biosphere(
            &mut spatial_biosphere,
            &current,
            &admin_info,
            &deterministic_trig,
            &game_settings
        );

        // This increases the simulation tick counter by 1.
        admin_info.tick_counter = admin_info.tick_counter + 1;
    }
}
