use crate::settings::GameSettings;
use crate::simulation::AllSpatialBiosphereInformation;
use crate::utility_functions::deterministic_trigonometry::DeterministicTrig;
use crate::simulation::biosphere_simulation::collision_calculations::collision_detector::detect_collision;
use crate::simulation::biosphere_simulation::collision_calculations::organism_combination::apply_collision;
use crate::simulation::biosphere_simulation::collision_calculations::detection_grid_updater;

use super::collision_calculations::{ ROTATIONAL_SPEED_LIMIT, SPEED_LIMIT };

pub struct CollisionCheckResult {
    pub collision: bool,
    pub x_move: i32,
    pub y_move: i32,
    pub r_move: i32,
    pub involved_blobs: Vec<usize>,
    pub mineral_involved: bool,
}

pub fn move_blob(
    all_spatial_biosphere_information: &mut AllSpatialBiosphereInformation,
    deterministic_trig: &DeterministicTrig,
    blob_number: usize,
    game_settings: &GameSettings
) {
    // Don't do anything if translational velocity and angular velocity are zero.
    if
        all_spatial_biosphere_information.blob_vec[blob_number].blob_x_velocity != 0 ||
        all_spatial_biosphere_information.blob_vec[blob_number].blob_y_velocity != 0 ||
        all_spatial_biosphere_information.blob_vec[blob_number].angular_velocity != 0
    {
        // Enforce speed limits
        if
            all_spatial_biosphere_information.blob_vec[blob_number].blob_x_velocity.abs() >
            SPEED_LIMIT
        {
            all_spatial_biosphere_information.blob_vec[blob_number].blob_x_velocity =
                SPEED_LIMIT *
                all_spatial_biosphere_information.blob_vec[blob_number].blob_x_velocity.signum();
        }

        if
            all_spatial_biosphere_information.blob_vec[blob_number].blob_y_velocity.abs() >
            SPEED_LIMIT
        {
            all_spatial_biosphere_information.blob_vec[blob_number].blob_y_velocity =
                SPEED_LIMIT *
                all_spatial_biosphere_information.blob_vec[blob_number].blob_y_velocity.signum();
        }

        if
            all_spatial_biosphere_information.blob_vec[blob_number].angular_velocity.abs() >
            ROTATIONAL_SPEED_LIMIT
        {
            all_spatial_biosphere_information.blob_vec[blob_number].angular_velocity =
                ROTATIONAL_SPEED_LIMIT *
                all_spatial_biosphere_information.blob_vec[blob_number].angular_velocity.signum();
        }

        // Test code !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        if
            all_spatial_biosphere_information.blob_vec[blob_number].angular_velocity.abs() > 0 {

                println!("Angular velocity exits.");
            }
        
        // End Test code !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

        // Checks to see if current velocities result in a collision for currently moving blob.
        let mut detection_result = detect_collision(
            &all_spatial_biosphere_information,
            blob_number,
            &game_settings,
            &deterministic_trig
        );

        // Rules out immediate collision before doing expensive calculations to determine angular motion.
        if
            detection_result.x_move != 0 ||
            detection_result.y_move != 0 ||
            detection_result.r_move != 0
        {
            /* Start temporary code */

            let mut reverse_x = false;
            let mut reverse_y = false;

            for organism_number in all_spatial_biosphere_information.blob_vec[
                blob_number
            ].blob_members.iter() {
                if
                    (all_spatial_biosphere_information.organism_information_vec[
                        *organism_number
                    ].x_location + detection_result.x_move).abs() >= game_settings.map_width / 2
                {
                    reverse_x = true;
                }

                if
                    (all_spatial_biosphere_information.organism_information_vec[
                        *organism_number
                    ].y_location+ detection_result.y_move).abs() >= game_settings.map_height / 2
                {
                    reverse_y = true;
                }
            }

            if reverse_x == true {
                all_spatial_biosphere_information.blob_vec[blob_number].blob_x_velocity =
                    all_spatial_biosphere_information.blob_vec[blob_number].blob_x_velocity * -1;
                all_spatial_biosphere_information.blob_vec[blob_number].angular_velocity = 0;
                detection_result.x_move = 0;
                detection_result.y_move = 0;
                detection_result.r_move = 0;
            }

            if reverse_y == true {
                all_spatial_biosphere_information.blob_vec[blob_number].blob_y_velocity =
                    all_spatial_biosphere_information.blob_vec[blob_number].blob_y_velocity * -1;
                all_spatial_biosphere_information.blob_vec[blob_number].angular_velocity = 0;
                detection_result.x_move = 0;
                detection_result.y_move = 0;
                detection_result.r_move = 0;
            }

            /* End Temporary code!!! */

            // The blob center of mass needs to be moved.
            all_spatial_biosphere_information.blob_vec[blob_number].center_of_mass_x +=
                detection_result.x_move;
            all_spatial_biosphere_information.blob_vec[blob_number].center_of_mass_y +=
                detection_result.y_move;

            // Every organism in the blob needs to be moved.
            for member_number in 0..all_spatial_biosphere_information.blob_vec[
                blob_number
            ].blob_members.len() {
                /*  ========================================================================================
                                  Start the only place in the code allowed to move organisms.
                    ========================================================================================
                */

                let organism_number =
                    all_spatial_biosphere_information.blob_vec[blob_number].blob_members
                        [member_number];

                // Store previous position so the old record can be deleted from the collision detector.
                let previous_x = all_spatial_biosphere_information.organism_information_vec
                    [organism_number].x_location;
                let previous_y = all_spatial_biosphere_information.organism_information_vec
                    [organism_number].y_location;
                let previous_circle_positions =
                    all_spatial_biosphere_information.organism_information_vec[
                        organism_number
                    ].other_circle_positions.clone();

                // It is simple to move organisms not attached to anything.
                if
                    !all_spatial_biosphere_information.organism_information_vec
                        [organism_number].part_of_multi_org_blob
                {
                    // Move the organism itself.
                    all_spatial_biosphere_information.organism_information_vec[
                        organism_number
                    ].x_location += detection_result.x_move;
                    all_spatial_biosphere_information.organism_information_vec[
                        organism_number
                    ].y_location += detection_result.y_move;
                    all_spatial_biosphere_information.organism_information_vec[
                        organism_number
                    ].rotation += detection_result.r_move;

                    // Move any extra circles for oblong blobs.
                    if
                        all_spatial_biosphere_information.organism_information_vec
                            [organism_number].oblong
                    {
                        // This is easy if it is not rotating
                        if detection_result.r_move == 0 {
                            for circle in all_spatial_biosphere_information.organism_information_vec[
                                organism_number
                            ].other_circle_positions.iter_mut() {
                                circle.x += detection_result.x_move;
                                circle.y += detection_result.y_move;
                            }

                            // This requires some trig if it is rotating.
                        } else {
                            for circle_number in 0..all_spatial_biosphere_information.organism_information_vec[
                                organism_number
                            ].other_circle_positions.len() {
                                all_spatial_biosphere_information.organism_information_vec[
                                    organism_number
                                ].other_circle_positions[circle_number].angle_from_org_center +=
                                    detection_result.r_move;
                                all_spatial_biosphere_information.organism_information_vec[
                                    organism_number
                                ].other_circle_positions[circle_number].x =
                                    all_spatial_biosphere_information.organism_information_vec
                                        [organism_number].x_location +
                                    (all_spatial_biosphere_information.organism_information_vec
                                        [organism_number].other_circle_positions
                                        [circle_number].distance_from_org_center *
                                        deterministic_trig.d_trig.cosine((
                                            all_spatial_biosphere_information.organism_information_vec
                                                [organism_number].other_circle_positions
                                                [circle_number].angle_from_org_center,
                                            1000,
                                        )).0) /
                                        1000;
                                all_spatial_biosphere_information.organism_information_vec[
                                    organism_number
                                ].other_circle_positions[circle_number].y =
                                    all_spatial_biosphere_information.organism_information_vec
                                        [organism_number].y_location +
                                    (all_spatial_biosphere_information.organism_information_vec
                                        [organism_number].other_circle_positions
                                        [circle_number].distance_from_org_center *
                                        deterministic_trig.d_trig.sine((
                                            all_spatial_biosphere_information.organism_information_vec
                                                [organism_number].other_circle_positions
                                                [circle_number].angle_from_org_center,
                                            1000,
                                        )).0) /
                                        1000;
                            }
                        }
                    }
                } else {
                    // Check for the easy case in which no rotation is happening
                    if detection_result.r_move == 0 {
                        // Move the organism itself.
                        all_spatial_biosphere_information.organism_information_vec[
                            organism_number
                        ].x_location += detection_result.x_move;
                        all_spatial_biosphere_information.organism_information_vec[
                            organism_number
                        ].y_location += detection_result.y_move;

                        // Move other circles if needed.
                        if
                            all_spatial_biosphere_information.organism_information_vec
                                [organism_number].oblong
                        {
                            for circle in all_spatial_biosphere_information.organism_information_vec[
                                organism_number
                            ].other_circle_positions.iter_mut() {
                                circle.x += detection_result.x_move;
                                circle.y += detection_result.y_move;
                            }
                        }
                        // This is the most complex case of a rotating multi-blob organism.
                    } else {
                        // Move the organism itself
                        all_spatial_biosphere_information.organism_information_vec[
                            organism_number
                        ].rotation += detection_result.r_move;
                        all_spatial_biosphere_information.organism_information_vec[
                            organism_number
                        ].angle_to_center_of_mass += detection_result.r_move;
                        let distance = all_spatial_biosphere_information.organism_information_vec
                            [organism_number].distance_from_center_of_mass;
                        let angle = all_spatial_biosphere_information.organism_information_vec
                            [organism_number].angle_to_center_of_mass;
                        let center_x = all_spatial_biosphere_information.blob_vec
                            [blob_number].center_of_mass_x;
                        let center_y = all_spatial_biosphere_information.blob_vec
                            [blob_number].center_of_mass_y;
                        all_spatial_biosphere_information.organism_information_vec[
                            organism_number
                        ].x_location =
                            center_x +
                            (distance * deterministic_trig.d_trig.cosine((angle, 1000)).0) / 1000;
                        all_spatial_biosphere_information.organism_information_vec[
                            organism_number
                        ].y_location =
                            center_y +
                            (distance * deterministic_trig.d_trig.sine((angle, 1000)).0) / 1000;
                        if
                            all_spatial_biosphere_information.organism_information_vec
                                [organism_number].oblong
                        {
                            for circle_number in 0..all_spatial_biosphere_information.organism_information_vec[
                                organism_number
                            ].other_circle_positions.len() {
                                all_spatial_biosphere_information.organism_information_vec[
                                    organism_number
                                ].other_circle_positions[circle_number].angle_from_org_center +=
                                    detection_result.r_move;
                                all_spatial_biosphere_information.organism_information_vec[
                                    organism_number
                                ].other_circle_positions[circle_number].x =
                                    all_spatial_biosphere_information.organism_information_vec
                                        [organism_number].x_location +
                                    (all_spatial_biosphere_information.organism_information_vec
                                        [organism_number].other_circle_positions
                                        [circle_number].distance_from_org_center *
                                        deterministic_trig.d_trig.cosine((
                                            all_spatial_biosphere_information.organism_information_vec
                                                [organism_number].other_circle_positions
                                                [circle_number].angle_from_org_center,
                                            1000,
                                        )).0) /
                                        1000;
                                all_spatial_biosphere_information.organism_information_vec[
                                    organism_number
                                ].other_circle_positions[circle_number].y =
                                    all_spatial_biosphere_information.organism_information_vec
                                        [organism_number].y_location +
                                    (all_spatial_biosphere_information.organism_information_vec
                                        [organism_number].other_circle_positions
                                        [circle_number].distance_from_org_center *
                                        deterministic_trig.d_trig.sine((
                                            all_spatial_biosphere_information.organism_information_vec
                                                [organism_number].other_circle_positions
                                                [circle_number].angle_from_org_center,
                                            1000,
                                        )).0) /
                                        1000;
                            }
                        }
                    }
                }

                // This updates the collision detector
                detection_grid_updater::update_for_movement(
                    all_spatial_biosphere_information,
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
            apply_collision(
                all_spatial_biosphere_information,
                &detection_result.involved_blobs,
                detection_result.mineral_involved,
                &deterministic_trig
            );
        }
    }
}
