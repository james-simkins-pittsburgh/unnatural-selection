use crate::settings::GameSettings;
use crate::simulation::AllBiosphereInformation;
use crate::utility_functions::deterministic_trigonometry::DeterministicTrig;
use crate::simulation::biosphere_simulation::collision_calculations::collision_detector::detect_collision;
use crate::simulation::biosphere_simulation::collision_calculations::organism_collisions::apply_collision;
use crate::simulation::biosphere_simulation::collision_calculations::detection_grid_updater;

pub struct CollisionCheckResult {
    pub collision: bool,
    pub x_move: i32,
    pub y_move: i32,
    pub rotation_in_thousandth_radians: i32,
    pub involved_blobs: Vec<usize>,
    pub involved_minerals: Vec<usize>,
}

pub fn move_blob(
    all_biosphere_information: &mut AllBiosphereInformation,
    deterministic_trig: &DeterministicTrig,
    blob_number: usize,
    game_settings: &GameSettings
) {
    if
        all_biosphere_information.blob_vec[blob_number].blob_x_velocity != 0 ||
        all_biosphere_information.blob_vec[blob_number].blob_y_velocity != 0 ||
        all_biosphere_information.blob_vec[blob_number].angular_velocity != 0
    {
        let detection_result = detect_collision(all_biosphere_information, blob_number);

        for member_number in 0..all_biosphere_information.blob_vec[blob_number].blob_members.len() {
            if detection_result.x_move != 0 || detection_result.y_move != 0 {
                let organism_number =
                    all_biosphere_information.blob_vec[blob_number].blob_members[member_number];
                // Store previous position so the old record can be deleted from the collision detector.
                let previous_x = all_biosphere_information.organism_information_vec
                    [organism_number].x_location;
                let previous_y = all_biosphere_information.organism_information_vec
                    [organism_number].y_location;
                let previous_circle_positions =
                    all_biosphere_information.organism_information_vec[
                        organism_number
                    ].other_circle_positions.clone();

                // This is the only place in the code allowed to move organisms.
                all_biosphere_information.organism_information_vec[organism_number].x_location +=
                    detection_result.x_move;
                all_biosphere_information.organism_information_vec[organism_number].y_location +=
                    detection_result.y_move;
                // TO DO: ADD ROTATION HERE

                if all_biosphere_information.organism_information_vec[organism_number].oblong {
                    for circle_num in 0..all_biosphere_information.organism_information_vec[
                        organism_number
                    ].other_circle_positions.len() {
                        all_biosphere_information.organism_information_vec[
                            organism_number
                        ].other_circle_positions[circle_num].x += detection_result.x_move;
                        all_biosphere_information.organism_information_vec[
                            organism_number
                        ].other_circle_positions[circle_num].y += detection_result.y_move;
                    }
                // TO DO: ADD ROTATION HERE
                }

                // This updates the collision detector
                detection_grid_updater::update_for_movement(
                    all_biosphere_information,
                    previous_x,
                    previous_y,
                    &previous_circle_positions,
                    organism_number,
                    &game_settings
                );
            }
        }

        if detection_result.collision {
            apply_collision(all_biosphere_information, &detection_result.involved_blobs);
        }
    }
}
