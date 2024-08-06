use crate::settings::GameSettings;
use crate::simulation::AllBiosphereInformation;
use crate::simulation::AllSpeciesInformation;
use crate::simulation::AllCurrentInformation;
use crate::simulation::AdministrativeInformation;
use crate::simulation::CheapRandomGameworld;
use crate::utility_functions::deterministic_trigonometry::DeterministicTrig;

// This module detects currents and applies the motion to the blob.
pub mod current_detector;

// This module moves the blobs.
pub mod blob_mover;

// This detects collisions of organisms.
pub mod collision_detector;

pub fn simulate_biosphere(
    mut all_biosphere_information: &mut AllBiosphereInformation,
    _all_species_information: &AllSpeciesInformation,
    all_current_information: &AllCurrentInformation,
    _admin_information: &AdministrativeInformation,
    mut _cheap_random: &mut CheapRandomGameworld,
    d_trig: &DeterministicTrig,
    game_settings: &GameSettings
) {

    for organism_number in 1..all_biosphere_information.organism_information_vec.len() {

        current_detector::detect_current(
            &mut all_biosphere_information,
            d_trig,
            &all_current_information,
            organism_number,
        )

    }

    for blob_number in 1..all_biosphere_information.blob_vec.len() {
        blob_mover::move_blob(
            &mut all_biosphere_information,
            d_trig,
            blob_number,
            &game_settings,
        );
    }
}
