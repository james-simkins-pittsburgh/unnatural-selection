use blob_splitter::split_blob;

use crate::settings::GameSettings;
use crate::simulation::AllSpatialBiosphereInformation;
use crate::simulation::AllCurrentInformation;
use crate::simulation::AdministrativeInformation;
use crate::utility_functions::deterministic_trigonometry::DeterministicTrig;

// This module splits blobs into organisms
pub mod blob_splitter;

// This module detects currents and applies the motion to the blob.
pub mod current_applicator;

// This module moves the blobs.
pub mod blob_mover;

// This detects collisions of organisms.
pub mod collision_calculations;

pub fn simulate_spatial_biosphere(
    mut all_spatial_biosphere_information: &mut AllSpatialBiosphereInformation,
    all_current_information: &AllCurrentInformation,
    admin_information: &AdministrativeInformation,
    deterministic_trig: &DeterministicTrig,
    game_settings: &GameSettings
) {
    // Splits up one out of thirty blobs every turn.
    for blob_number in 1..all_spatial_biosphere_information.blob_vec.len() {
        if all_spatial_biosphere_information.blob_vec[blob_number].in_use {
            if (blob_number as i64) % 30 == admin_information.tick_counter % 30 {
                split_blob(&mut all_spatial_biosphere_information, &deterministic_trig, blob_number);
            }
        }
    }

    // Applies current to each blob
    for blob_number in 1..all_spatial_biosphere_information.blob_vec.len() {
        current_applicator::apply_current(
            &mut all_spatial_biosphere_information,
            &deterministic_trig,
            &all_current_information,
            blob_number
        );
    }

    // Moves each blob
    for blob_number in 1..all_spatial_biosphere_information.blob_vec.len() {
        if all_spatial_biosphere_information.blob_vec[blob_number].in_use {
            blob_mover::move_blob(
                &mut all_spatial_biosphere_information,
                &deterministic_trig,
                blob_number,
                &game_settings
            );
        }
    }
}
