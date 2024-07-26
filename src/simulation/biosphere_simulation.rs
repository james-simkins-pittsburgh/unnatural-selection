use crate::settings::GameSettings;
use crate::simulation::AllBiosphereInformation;
use crate::simulation::AllSpeciesInformation;
use crate::simulation::AllMapInformation;
use crate::simulation::AllCurrentInformation;
use crate::simulation::AdministrativeInformation;
use crate::simulation::CheapRandomGameworld;
use crate::utility_functions::deterministic_trigonometry::DeterministicTrig;

// This module moves the organisms.
pub mod organism_mover;

pub fn simulate_biosphere(
    mut all_biosphere_information: &mut AllBiosphereInformation,
    all_species_information: &AllSpeciesInformation,
    all_map_information: &AllMapInformation,
    all_current_information: &AllCurrentInformation,
    _admin_information: &AdministrativeInformation,
    mut _cheap_random: &mut CheapRandomGameworld,
    d_trig: &DeterministicTrig,
    game_settings: &GameSettings
) {
    for organism_number in 0..all_biosphere_information.organism_information_vec.len() {
        organism_mover::move_organism(
            &mut all_biosphere_information,
            &all_species_information,
            &all_map_information,
            &all_current_information,
            d_trig,
            organism_number,
            &game_settings,

        );
    }
}
