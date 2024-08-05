use crate::settings::GameSettings;
use crate::simulation::AllBiosphereInformation;
use crate::simulation::AllSpeciesInformation;
use crate::simulation::AllMapInformation;
use crate::simulation::AllCurrentInformation;
use crate::utility_functions::deterministic_trigonometry::DeterministicTrig;

pub fn move_organism(
    all_biosphere_information: &mut AllBiosphereInformation,
    _all_species_information: &AllSpeciesInformation,
    _all_map_information: &AllMapInformation,
    all_current_information: &AllCurrentInformation,
    deterministic_trig: &DeterministicTrig,
    organism_number: usize,
    game_settings: &GameSettings
) {
    if all_biosphere_information.organism_information_vec[organism_number].in_use {
        
        // Calculates acceleration from the currents.
        for current_number in 0..all_current_information.current_information_vec.len() {

            // Check distance from current centers.
            if
                (i64::from (all_biosphere_information.organism_information_vec[organism_number].x_location) -
                    i64::from (all_current_information.current_information_vec[current_number].center_x)) *
                    (i64::from(all_biosphere_information.organism_information_vec
                        [organism_number].x_location) -
                        i64::from(all_current_information.current_information_vec[current_number].center_x)) +
                    (i64::from(all_biosphere_information.organism_information_vec
                        [organism_number].y_location) -
                        i64::from (all_current_information.current_information_vec[current_number].center_y)) *
                        (i64::from (all_biosphere_information.organism_information_vec
                            [organism_number].y_location) -
                            i64::from(all_current_information.current_information_vec
                                [current_number].center_y)) <=
                i64::from (all_current_information.current_information_vec[current_number].radius) *
                    i64::from (all_current_information.current_information_vec[current_number].radius)
            {

                // Applies acceleration from current.
                all_biosphere_information.organism_information_vec[organism_number].x_velocity =
                    all_biosphere_information.organism_information_vec[organism_number].x_velocity +
                    (game_settings.current_intensity *
                        all_current_information.current_information_vec[current_number].intensity *
                        deterministic_trig.d_trig.cosine((
                            all_current_information.current_information_vec
                                [current_number].angle_in_radians_times_1000,
                            1000,
                        )).0) /
                        1000;

                all_biosphere_information.organism_information_vec[organism_number].y_velocity =
                    all_biosphere_information.organism_information_vec[organism_number].y_velocity +
                    (game_settings.current_intensity *
                        all_current_information.current_information_vec[current_number].intensity *
                        deterministic_trig.d_trig.sine((
                            all_current_information.current_information_vec
                                [current_number].angle_in_radians_times_1000,
                            1000,
                        )).0) /
                        1000;
            }
        }

        // Moves the organism unless this would move the organism outside of the bounds
        if
            all_biosphere_information.organism_information_vec[organism_number].x_location +
                all_biosphere_information.organism_information_vec[organism_number].x_velocity <=
                game_settings.map_length / 2 &&
            all_biosphere_information.organism_information_vec[organism_number].x_location +
                all_biosphere_information.organism_information_vec[organism_number].x_velocity >=
                -game_settings.map_length / 2
        {
            all_biosphere_information.organism_information_vec[organism_number].x_location =
                all_biosphere_information.organism_information_vec[organism_number].x_location +
                all_biosphere_information.organism_information_vec[organism_number].x_velocity;
        } else if
            all_biosphere_information.organism_information_vec[organism_number].x_location +
                all_biosphere_information.organism_information_vec[organism_number].x_velocity >
            game_settings.map_length / 2
        {
            all_biosphere_information.organism_information_vec[organism_number].x_location =
                game_settings.map_length / 2;
            all_biosphere_information.organism_information_vec[organism_number].x_velocity = -all_biosphere_information.organism_information_vec[organism_number].x_velocity;
        } else {
            all_biosphere_information.organism_information_vec[organism_number].x_location =
                -game_settings.map_length / 2;
            all_biosphere_information.organism_information_vec[organism_number].x_velocity = -all_biosphere_information.organism_information_vec[organism_number].x_velocity;
        }

        if
            all_biosphere_information.organism_information_vec[organism_number].y_location +
                all_biosphere_information.organism_information_vec[organism_number].y_velocity <=
                game_settings.map_height / 2 &&
            all_biosphere_information.organism_information_vec[organism_number].y_location +
                all_biosphere_information.organism_information_vec[organism_number].y_velocity >=
                -game_settings.map_height / 2
        {
            all_biosphere_information.organism_information_vec[organism_number].y_location =
                all_biosphere_information.organism_information_vec[organism_number].y_location +
                all_biosphere_information.organism_information_vec[organism_number].y_velocity;
        } else if
            all_biosphere_information.organism_information_vec[organism_number].y_location +
                all_biosphere_information.organism_information_vec[organism_number].y_velocity >
            game_settings.map_height / 2
        {
            all_biosphere_information.organism_information_vec[organism_number].y_location =
                game_settings.map_height / 2;
            all_biosphere_information.organism_information_vec[organism_number].y_velocity = -all_biosphere_information.organism_information_vec[organism_number].y_velocity;
        } else {
            all_biosphere_information.organism_information_vec[organism_number].y_location =
                -game_settings.map_height / 2;
            all_biosphere_information.organism_information_vec[organism_number].y_velocity = -all_biosphere_information.organism_information_vec[organism_number].y_velocity;
        }
    }
}
