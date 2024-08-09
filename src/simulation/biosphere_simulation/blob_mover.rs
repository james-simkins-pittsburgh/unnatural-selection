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

        // Rule out immediate collision before doing expensive calculations.
        if
            detection_result.x_move != 0 ||
            detection_result.y_move != 0 ||
            detection_result.rotation_in_thousandth_radians != 0
        {
            // The blob center of mass needs to be moved.
            all_biosphere_information.blob_vec[blob_number].center_of_mass_x +=
                detection_result.x_move;
            all_biosphere_information.blob_vec[blob_number].center_of_mass_y +=
                detection_result.y_move;

            // Every organism in the blob needs to be moved.
            for member_number in 0..all_biosphere_information.blob_vec[
                blob_number
            ].blob_members.len() {
                /*  ========================================================================================
                                  Start the only place in the code allowed to move organisms.
                    ========================================================================================
                */

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

                // It is simple to move organisms not attached to anything.
                if
                    !all_biosphere_information.organism_information_vec
                        [organism_number].part_of_multi_org_blob
                {
                    // Move the organism itself.
                    all_biosphere_information.organism_information_vec[
                        organism_number
                    ].x_location += detection_result.x_move;
                    all_biosphere_information.organism_information_vec[
                        organism_number
                    ].y_location += detection_result.y_move;
                    all_biosphere_information.organism_information_vec[organism_number].rotation +=
                        detection_result.rotation_in_thousandth_radians;

                    // Move any extra circles for oblong blobs.
                    if all_biosphere_information.organism_information_vec[organism_number].oblong {
                        // This is easy if not rotating
                        if detection_result.rotation_in_thousandth_radians == 0 {
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
                        // This requires some trig if it is rotating.
                        } else {
                            // TO DO: ADD ROTATION LOGIC FOR HERE FOR OBLONG BLOBS
                        }
                    }
                // This is much more complicated more multi-organism blobs.
                } else {
                    // TO DO: ADD LOGIC FOR MULTI-ORGANISM BLOBS
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

                /*  ========================================================================================
                                  End the only place in the code allowed to move organisms.
                    ========================================================================================
                */
            }
        }

        if detection_result.collision {
            apply_collision(all_biosphere_information, &detection_result.involved_blobs);
        }
    }
}
