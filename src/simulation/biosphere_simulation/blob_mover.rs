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

                /*
                
                            STILL NEED ANGULAR MOVEMENT HERE!!!!
                
                 */
                
                // This is the only place in the code allowed to move organisms.
                all_biosphere_information.organism_information_vec[organism_number].x_location +=
                    detection_result.x_move;
                all_biosphere_information.organism_information_vec[organism_number].y_location +=
                    detection_result.y_move;

                // This updates the collision detector
                detection_grid_updater::update_for_movement(
                    all_biosphere_information,
                    previous_x,
                    previous_y,
                    organism_number,
                    game_settings
                );
            }
        }

        if detection_result.collision {
            apply_collision(all_biosphere_information, &detection_result.involved_blobs);
        }
    }
}
