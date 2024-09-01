use bevy::scene::ron::de;

use crate::{
    settings::GameSettings,
    simulation::{
        biosphere_simulation::blob_mover::CollisionCheckResult,
        AllSpatialBiosphereInformation,
        CircleEntityType,
        CirclePositionRecord,
    },
    utility_functions::{
        deterministic_trigonometry::DeterministicTrig,
        integer_math::square_root_64,
        quadratic_solver,
        two_circles_intersection_solver::solve_two_circle_intersection,
    },
};

use super::GRID_SIZE;

// This stores the circle information for the colliders.
struct ColliderCircleInfo {
    x: i32,
    y: i32,
    radius: i32,
    distance_to_center_of_mass: i32,
    angle_to_center_of_mass: i32,
}

pub fn detect_collision(
    all_spatial_biosphere_information: &AllSpatialBiosphereInformation,
    blob_number: usize,
    game_settings: &GameSettings,
    deterministic_trig: &DeterministicTrig
) -> CollisionCheckResult {
    let mut collider_circles: Vec<ColliderCircleInfo> = Vec::new();

    // This makes a vec of all the circles of the collider blob.
    for organism_number in all_spatial_biosphere_information.blob_vec[blob_number].blob_members.iter() {
        collider_circles.push(ColliderCircleInfo {
            x: all_spatial_biosphere_information.organism_information_vec[*organism_number].x_location,
            y: all_spatial_biosphere_information.organism_information_vec[*organism_number].y_location,
            radius: all_spatial_biosphere_information.organism_information_vec[*organism_number].radius,
            distance_to_center_of_mass: all_spatial_biosphere_information.organism_information_vec
                [*organism_number].distance_from_center_of_mass,
            angle_to_center_of_mass: all_spatial_biosphere_information.organism_information_vec
                [*organism_number].angle_to_center_of_mass,
        });

        if all_spatial_biosphere_information.organism_information_vec[*organism_number].oblong {
            for circle in all_spatial_biosphere_information.organism_information_vec[
                *organism_number
            ].other_circle_positions.iter() {
                collider_circles.push(ColliderCircleInfo {
                    x: circle.x,
                    y: circle.y,
                    radius: circle.radius,
                    distance_to_center_of_mass: all_spatial_biosphere_information.organism_information_vec
                        [*organism_number].distance_from_center_of_mass,
                    angle_to_center_of_mass: all_spatial_biosphere_information.organism_information_vec
                        [*organism_number].angle_to_center_of_mass,
                });
            }
        }
    }

    // Send all the circles to the helper function to work out what, if anything, collides first
    // Then send the results back to the blob mover.
    return check_circles(
        collider_circles,
        &all_spatial_biosphere_information,
        &game_settings,
        blob_number,
        deterministic_trig
    );
}

// This helper function consults the detection grid to determine if any collisions will occur with the movement of the blob.
fn check_circles(
    collider_circles: Vec<ColliderCircleInfo>,
    all_spatial_biosphere_information: &AllSpatialBiosphereInformation,
    game_settings: &GameSettings,
    blob_number: usize,
    deterministic_trig: &DeterministicTrig
) -> CollisionCheckResult {
    // These store the maximum movement before a collision (if any) occurs.
    let mut x_move = all_spatial_biosphere_information.blob_vec[blob_number].blob_x_velocity;
    let mut y_move = all_spatial_biosphere_information.blob_vec[blob_number].blob_y_velocity;
    let mut r_move = all_spatial_biosphere_information.blob_vec[blob_number].angular_velocity;

    // This stores the original moves so it can be references later.
    let original_x_move = x_move;
    let original_y_move = y_move;

    // This is a list of all involved blobs so calculations can be done to get new velocities after a collision.
    let mut involved_blobs = vec![blob_number];

    // This keeps track if a mineral is involved. If one is, then the entire collision will result in 0 velocities.
    let mut involved_minerals = false;
    // Iterates over every collider circle.
    for collider_circle in collider_circles.iter() {
        // If the only movement is rotational, this can all be skipped.
        if x_move != 0 && y_move != 0 {
            // Iterates over every collidee circle in the detection grid.
            for collidee_circle in all_spatial_biosphere_information.collision_detection_grid[
                ((collider_circle.x + game_settings.map_length / 2) / GRID_SIZE) as usize
            ][((collider_circle.y + game_settings.map_height / 2) / GRID_SIZE) as usize].iter() {
                // This function checks if the two circle collide and determined how much x and y movement occurs before that.
                check_two_circles_translational(
                    &mut x_move,
                    &mut y_move,
                    original_x_move,
                    original_y_move,
                    &mut involved_blobs,
                    &mut involved_minerals,
                    blob_number,
                    collider_circle,
                    collidee_circle
                );
            }

            /* I THINK THERE IS A LOGIC ERROR HERE. NEEDS TO CONSIDER POTENTIAL COLLIDEE SIZE AS WELL AS X_MOVE AND Y_MOVE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! */

            // This checks to see if the grid to the right needs to be checked.

            if
                (collider_circle.x + collider_circle.radius + game_settings.map_length / 2) /
                    GRID_SIZE != (collider_circle.x + game_settings.map_length / 2) / GRID_SIZE &&
                (collider_circle.x + collider_circle.radius + game_settings.map_length / 2) /
                    GRID_SIZE < (all_spatial_biosphere_information.collision_detection_grid.len() as i32)
            {
                for collidee_circle in all_spatial_biosphere_information.collision_detection_grid[
                    ((collider_circle.x + collider_circle.radius + game_settings.map_length / 2) /
                        GRID_SIZE) as usize
                ][
                    ((collider_circle.y + game_settings.map_height / 2) / GRID_SIZE) as usize
                ].iter() {
                    // This function checks if the two circle collide and determined how much x and y movement occurs before that.
                    check_two_circles_translational(
                        &mut x_move,
                        &mut y_move,
                        original_x_move,
                        original_y_move,
                        &mut involved_blobs,
                        &mut involved_minerals,
                        blob_number,
                        collider_circle,
                        collidee_circle
                    );
                }
            }

            // This checks to see if the grid to the left to be checked.

            if
                (collider_circle.x - collider_circle.radius + game_settings.map_length / 2) /
                    GRID_SIZE != (collider_circle.x + game_settings.map_length / 2) / GRID_SIZE &&
                (collider_circle.x - collider_circle.radius + game_settings.map_length / 2) /
                    GRID_SIZE >= 0
            {
                for collidee_circle in all_spatial_biosphere_information.collision_detection_grid[
                    ((collider_circle.x - collider_circle.radius + game_settings.map_length / 2) /
                        GRID_SIZE) as usize
                ][
                    ((collider_circle.y + game_settings.map_height / 2) / GRID_SIZE) as usize
                ].iter() {
                    // This function checks if the two circle collide and determined how much x and y movement occurs before that.
                    check_two_circles_translational(
                        &mut x_move,
                        &mut y_move,
                        original_x_move,
                        original_y_move,
                        &mut involved_blobs,
                        &mut involved_minerals,
                        blob_number,
                        collider_circle,
                        collidee_circle
                    );
                }
            }

            // This checks to see if the grid above needs to be checked.

            if
                (collider_circle.y + collider_circle.radius + game_settings.map_height / 2) /
                    GRID_SIZE != (collider_circle.y + game_settings.map_height / 2) / GRID_SIZE &&
                (collider_circle.y + collider_circle.radius + game_settings.map_height / 2) /
                    GRID_SIZE < (all_spatial_biosphere_information.collision_detection_grid[0].len() as i32)
            {
                for collidee_circle in all_spatial_biosphere_information.collision_detection_grid[
                    ((collider_circle.x + game_settings.map_length / 2) / GRID_SIZE) as usize
                ][
                    ((collider_circle.y + collider_circle.radius + game_settings.map_height / 2) /
                        GRID_SIZE) as usize
                ].iter() {
                    // This function checks if the two circle collide and determined how much x and y movement occurs before that.
                    check_two_circles_translational(
                        &mut x_move,
                        &mut y_move,
                        original_x_move,
                        original_y_move,
                        &mut involved_blobs,
                        &mut involved_minerals,
                        blob_number,
                        collider_circle,
                        collidee_circle
                    );
                }

                // This checks to see if the grid above and to the right needs to be checked.

                if
                    (collider_circle.x + collider_circle.radius + game_settings.map_length / 2) /
                        GRID_SIZE !=
                        (collider_circle.x + game_settings.map_length / 2) / GRID_SIZE &&
                    (collider_circle.x + collider_circle.radius + game_settings.map_length / 2) /
                        GRID_SIZE <
                        (all_spatial_biosphere_information.collision_detection_grid.len() as i32)
                {
                    for collidee_circle in all_spatial_biosphere_information.collision_detection_grid[
                        ((collider_circle.x +
                            collider_circle.radius +
                            game_settings.map_length / 2) /
                            GRID_SIZE) as usize
                    ][
                        ((collider_circle.y +
                            collider_circle.radius +
                            game_settings.map_height / 2) /
                            GRID_SIZE) as usize
                    ].iter() {
                        // This function checks if the two circle collide and determined how much x and y movement occurs before that.
                        check_two_circles_translational(
                            &mut x_move,
                            &mut y_move,
                            original_x_move,
                            original_y_move,
                            &mut involved_blobs,
                            &mut involved_minerals,
                            blob_number,
                            collider_circle,
                            collidee_circle
                        );
                    }
                }

                // This checks to see if the grid above and to the left needs to be checked.

                if
                    (collider_circle.x - collider_circle.radius + game_settings.map_length / 2) /
                        GRID_SIZE !=
                        (collider_circle.x + game_settings.map_length / 2) / GRID_SIZE &&
                    (collider_circle.x - collider_circle.radius + game_settings.map_length / 2) /
                        GRID_SIZE >= 0
                {
                    for collidee_circle in all_spatial_biosphere_information.collision_detection_grid[
                        ((collider_circle.x -
                            collider_circle.radius +
                            game_settings.map_length / 2) /
                            GRID_SIZE) as usize
                    ][
                        ((collider_circle.y +
                            collider_circle.radius +
                            game_settings.map_height / 2) /
                            GRID_SIZE) as usize
                    ].iter() {
                        // This function checks if the two circle collide and determined how much x and y movement occurs before that.
                        check_two_circles_translational(
                            &mut x_move,
                            &mut y_move,
                            original_x_move,
                            original_y_move,
                            &mut involved_blobs,
                            &mut involved_minerals,
                            blob_number,
                            collider_circle,
                            collidee_circle
                        );
                    }
                }
            }

            // This checks to see if the grid below needs to be checked.

            if
                (collider_circle.y - collider_circle.radius + game_settings.map_height / 2) /
                    GRID_SIZE != (collider_circle.y + game_settings.map_height / 2) / GRID_SIZE &&
                (collider_circle.y + collider_circle.radius + game_settings.map_height / 2) /
                    GRID_SIZE >= 0
            {
                for collidee_circle in all_spatial_biosphere_information.collision_detection_grid[
                    ((collider_circle.x + game_settings.map_length / 2) / GRID_SIZE) as usize
                ][
                    ((collider_circle.y - collider_circle.radius + game_settings.map_height / 2) /
                        GRID_SIZE) as usize
                ].iter() {
                    // This function checks if the two circle collide and determined how much x and y movement occurs before that.
                    check_two_circles_translational(
                        &mut x_move,
                        &mut y_move,
                        original_x_move,
                        original_y_move,
                        &mut involved_blobs,
                        &mut involved_minerals,
                        blob_number,
                        collider_circle,
                        collidee_circle
                    );
                }

                // This checks to see if the grid below and to the right needs to be checked.

                if
                    (collider_circle.x + collider_circle.radius + game_settings.map_length / 2) /
                        GRID_SIZE !=
                        (collider_circle.x + game_settings.map_length / 2) / GRID_SIZE &&
                    (collider_circle.x + collider_circle.radius + game_settings.map_length / 2) /
                        GRID_SIZE <
                        (all_spatial_biosphere_information.collision_detection_grid.len() as i32)
                {
                    for collidee_circle in all_spatial_biosphere_information.collision_detection_grid[
                        ((collider_circle.x +
                            collider_circle.radius +
                            game_settings.map_length / 2) /
                            GRID_SIZE) as usize
                    ][
                        ((collider_circle.y -
                            collider_circle.radius +
                            game_settings.map_height / 2) /
                            GRID_SIZE) as usize
                    ].iter() {
                        // This function checks if the two circle collide and determined how much x and y movement occurs before that.
                        check_two_circles_translational(
                            &mut x_move,
                            &mut y_move,
                            original_x_move,
                            original_y_move,
                            &mut involved_blobs,
                            &mut involved_minerals,
                            blob_number,
                            collider_circle,
                            collidee_circle
                        );
                    }
                }

                // This checks to see if the grid below and to the left needs to be checked.

                if
                    (collider_circle.x - collider_circle.radius + game_settings.map_length / 2) /
                        GRID_SIZE !=
                        (collider_circle.x + game_settings.map_length / 2) / GRID_SIZE &&
                    (collider_circle.x - collider_circle.radius + game_settings.map_length / 2) /
                        GRID_SIZE >= 0
                {
                    for collidee_circle in all_spatial_biosphere_information.collision_detection_grid[
                        ((collider_circle.x -
                            collider_circle.radius +
                            game_settings.map_length / 2) /
                            GRID_SIZE) as usize
                    ][
                        ((collider_circle.y -
                            collider_circle.radius +
                            game_settings.map_height / 2) /
                            GRID_SIZE) as usize
                    ].iter() {
                        // This function checks if the two circle collide and determined how much x and y movement occurs before that.
                        check_two_circles_translational(
                            &mut x_move,
                            &mut y_move,
                            original_x_move,
                            original_y_move,
                            &mut involved_blobs,
                            &mut involved_minerals,
                            blob_number,
                            collider_circle,
                            collidee_circle
                        );
                    }
                }
            }
        }

        // Rotational movement only happens if the full translational movement is completed.

        if r_move != 0 && x_move == original_y_move && y_move == original_y_move {
            // Store the new collider x and y after full rotation.
            let full_collider_x =
                all_spatial_biosphere_information.blob_vec[blob_number].center_of_mass_x +
                x_move +
                (collider_circle.distance_to_center_of_mass *
                    deterministic_trig.d_trig.cosine((r_move, 1000)).0) /
                    1000;
            let full_collider_y =
                all_spatial_biosphere_information.blob_vec[blob_number].center_of_mass_y +
                y_move +
                (collider_circle.distance_to_center_of_mass *
                    deterministic_trig.d_trig.sine((r_move, 1000)).0) /
                    1000;

            // LEFT OFF HERE!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        }
    }

    let collision: bool;

    // If there are involved blobs other than the blob, then a collision must have happened.
    if involved_blobs.len() > 1 || involved_minerals == true {
        collision = true;
    } else {
        collision = false;
    }

    return CollisionCheckResult {
        collision,
        x_move,
        y_move,
        r_move,
        involved_blobs,
        involved_minerals,
    };
}

fn check_two_circles_translational(
    x_move: &mut i32,
    y_move: &mut i32,
    original_x_move: i32,
    original_y_move: i32,
    involved_blobs: &mut Vec<usize>,
    involved_minerals: &mut bool,
    blob_number: usize,
    collider_circle: &ColliderCircleInfo,
    collidee_circle: &CirclePositionRecord
) {
    // If the circles are not part of the same blob.
    if
        collidee_circle.blob_number != blob_number ||
        collidee_circle.circle_entity_type == CircleEntityType::Mineral
    {
        // If they're going to collide with less than or equal to the current x and y moves.
        if
            ((collidee_circle.center_x - (collider_circle.x + *x_move)) as i64) *
                ((collidee_circle.center_x - (collider_circle.x + *x_move)) as i64) +
                ((collidee_circle.center_y - (collider_circle.y + *y_move)) as i64) *
                    ((collidee_circle.center_y - (collider_circle.y + *y_move)) as i64) <=
            ((collidee_circle.radius + collider_circle.radius) as i64) *
                ((collidee_circle.radius + collider_circle.radius) as i64)
        {
            // Write these to memory to simplify code and reduce repetitive calculations
            let current_x_distance_squared =
                ((collidee_circle.center_x - collider_circle.x) as i64) *
                ((collidee_circle.center_x - collider_circle.x) as i64);

            let current_y_distance_squared =
                ((collidee_circle.center_y - collider_circle.y) as i64) *
                ((collidee_circle.center_y - collider_circle.y) as i64);

            let combined_radii_squared =
                ((collidee_circle.radius + collider_circle.radius) as i64) *
                ((collidee_circle.radius + collider_circle.radius) as i64);

            // Check for the case in which the circles were already overlapping because of an error.
            if current_x_distance_squared + current_y_distance_squared < combined_radii_squared {
                // If the x_move and y_move were already 0.
                if *x_move == 0 && *y_move == 0 {
                    // The collidee entity number just needs to be added.
                    if collidee_circle.circle_entity_type == CircleEntityType::Organism {
                        involved_blobs.push(collidee_circle.blob_number);
                        // Or if it is a mineral then the boolean needs to be marked true.
                    } else {
                        *involved_minerals = true;
                    }

                    // If they weren't already 0
                } else {
                    // Zero out the movement.
                    *x_move = 0;
                    *y_move = 0;

                    // Clear the collision list except for those blobs.
                    if collidee_circle.circle_entity_type == CircleEntityType::Organism {
                        *involved_blobs = vec![blob_number, collidee_circle.blob_number];
                        *involved_minerals = false;
                    } else {
                        *involved_blobs = vec![blob_number];
                        *involved_minerals = true;
                    }
                }
                // Since the moves are 0, the function can be over.
                return;
            }

            // Write these to memory to simplify code and reduce repetitive calculations

            let future_x_distance_squared =
                ((collidee_circle.center_x - (collider_circle.x + *x_move)) as i64) *
                ((collidee_circle.center_x - (collider_circle.x + *x_move)) as i64);

            let future_y_distance_squared =
                ((collidee_circle.center_y - (collider_circle.y + *y_move)) as i64) *
                ((collidee_circle.center_y - (collider_circle.y + *y_move)) as i64);

            // If they're going to collide with less than the current x and y moves.
            if future_x_distance_squared + future_y_distance_squared < combined_radii_squared {
                // Then reset the collision list because collisions with x_move and y_move aren't happening.
                if collidee_circle.circle_entity_type == CircleEntityType::Organism {
                    *involved_blobs = vec![blob_number, collidee_circle.blob_number];
                    *involved_minerals = false;
                } else {
                    *involved_blobs = vec![blob_number];
                    *involved_minerals = true;
                }

                // Make sure x_move isn't 0.
                if *x_move != 0 {
                    let slope_x_1000 =
                        ((original_y_move as i128) * 1000) / (original_x_move as i128);
                    let x1 = collider_circle.x as i128;
                    let y1 = collider_circle.y as i128;
                    let x2 = collidee_circle.center_x as i128;
                    let y2 = collidee_circle.center_y as i128;

                    let e_x_1000 = 1000 * y1 - slope_x_1000 * x1;

                    let a_x_1000 = 1000 + (slope_x_1000 * slope_x_1000) / 1000;

                    let b_x_1000 =
                        (2 * (slope_x_1000 * e_x_1000 - slope_x_1000 * y2 * 1000 - 1000000 * x2)) /
                        1000;

                    let c_x_1000 =
                        (y2 * y2 * 1000000 -
                            (combined_radii_squared as i128) * 1000000 +
                            1000000 * x2 * x2 -
                            1000 * 2 * e_x_1000 * y2 +
                            e_x_1000 * e_x_1000) /
                        1000;

                    // This tuple holds the solutions to the quadratic

                    let quadratic_solutions = quadratic_solver::solve_quadratic(
                        a_x_1000,
                        b_x_1000,
                        c_x_1000
                    );

                    // If is x collision 1 is a closer than x collision 2 then set x move to it. Otherwise, set x move to collision 2.
                    if
                        (quadratic_solutions.0 - (x1 as i64)).abs() <
                        (quadratic_solutions.1 - (x1 as i64)).abs()
                    {
                        *x_move = (quadratic_solutions.0 - (x1 as i64)) as i32;
                    } else {
                        *x_move = (quadratic_solutions.1 - (x1 as i64)) as i32;
                    }

                    // Make sure x_move has not become 0.
                    if *x_move != 0 {
                        // Set y_move based on the fact that movement will be proportional to the full movement before collision.
                        *y_move = (*x_move * original_y_move) / original_x_move;

                        // Check to make sure rounding errors didn't move this past the collision point. Fix it if it did.
                        while
                            x_move.abs() > 0 &&
                            (x2 - (x1 + (*x_move as i128))) * (x2 - (x1 + (*x_move as i128))) +
                                (y2 - (y1 + (*y_move as i128))) * (y2 - (y1 + (*y_move as i128))) <
                                (combined_radii_squared as i128)
                        {
                            // Slowly back it off if it is overlapping by intervals of 1 on the axis of greater velocity
                            if x_move.abs() >= y_move.abs() {
                                // Back off x_move by 1
                                *x_move = *x_move - x_move.signum();

                                // Adjust y_move proportionally
                                if x_move.abs() > 0 {
                                    *y_move = (*x_move * original_y_move) / original_x_move;
                                } else {
                                    *y_move = 0;
                                }
                            } else {
                                // Back off y_move by 1
                                *y_move = *y_move - y_move.signum();

                                // Adjust y_move proportionally
                                if y_move.abs() > 0 {
                                    *x_move = (*y_move * original_x_move) / original_y_move;
                                } else {
                                    *x_move = 0;
                                }
                            }
                        }
                    } else {
                        // In the case where x_move has become 0, y_move should be set to 0 too.
                        *y_move = 0;
                    }

                    // In the case where x_move is 0 from the beginning of the function.
                } else {
                    // If x_move is 0 because x_move was originally 0 for the blob.
                    if original_x_move == 0 && original_y_move != 0 {
                        // Then y of the collider at the collision points can be calculated as follows.
                        let y_of_collider_at_collision_one =
                            collidee_circle.center_y +
                            (
                                square_root_64(
                                    combined_radii_squared - current_x_distance_squared
                                ) as i32
                            );

                        // This is the other solution to the quadratic.
                        let y_of_collider_at_collision_two =
                            collidee_circle.center_y -
                            (
                                square_root_64(
                                    combined_radii_squared - current_x_distance_squared
                                ) as i32
                            );

                        // Set y_move to the closer of the two.
                        if
                            y_of_collider_at_collision_one.abs() <
                            y_of_collider_at_collision_two.abs()
                        {
                            *y_move = y_of_collider_at_collision_one;
                        } else {
                            *y_move = y_of_collider_at_collision_two;
                        }

                        // Check to make sure rounding errors didn't move this past the collision point. Fix it if it did.
                        while
                            y_move.abs() > 0 &&
                            current_x_distance_squared +
                                (
                                    ((collidee_circle.center_y - (collider_circle.y + *y_move)) *
                                        (collidee_circle.center_y -
                                            (collider_circle.y + *y_move))) as i64
                                ) < combined_radii_squared
                        {
                            // Back off y_move by 1
                            *y_move = *y_move - y_move.signum();
                        }

                        // If x_move wasn't originally 0, make sure y_move is 0 too.
                    } else {
                        *y_move = 0;
                    }
                }

                // If it is exactly on the circle, just add the collider. The x_move and y_move can stay the same.
            } else if x_move.abs() < original_x_move.abs() || y_move.abs() < original_y_move.abs() {
                // The collidee entity number just needs to be added.
                if collidee_circle.circle_entity_type == CircleEntityType::Organism {
                    involved_blobs.push(collidee_circle.blob_number)
                    // Or if it is a mineral then the boolean needs to be marked true.
                } else {
                    *involved_minerals = true;
                }
            }
        }
    }
}

fn check_two_circles_angular(
    r_move: &mut i32,
    original_r_move: i32,
    involved_blobs: &mut Vec<usize>,
    involved_minerals: &mut bool,
    blob_number: usize,
    collidee_circle: &CirclePositionRecord,
    collider_circle_radius: i32,
    collider_distance_center_of_mass: i32,
    // This is the center of mass after translation.
    center_of_mass_x_after_xymove: i32,
    center_of_mass_y_after_xymove: i32,
    // These are the x and y after translation only.
    collider_x_after_xymove: i32,
    collider_y_after_xymove: i32,
    // These are the x and y if it fully translates and rotates
    full_collider_x: i32,
    full_collider_y: i32,
    deterministic_trig: &DeterministicTrig
) {
    // If the circles are not part of the same blob.

    if
        collider_distance_center_of_mass != 0 &&
        (collidee_circle.blob_number != blob_number ||
            collidee_circle.circle_entity_type == CircleEntityType::Mineral)
    {
        // Check to see if a collision happens.
        if
            (collider_circle_radius + collidee_circle.radius) *
                (collider_circle_radius + collidee_circle.radius) <=
            (collidee_circle.center_x - full_collider_x) *
                (collidee_circle.center_x - full_collider_x) +
                (collidee_circle.center_y - full_collider_y) *
                    (collidee_circle.center_y - full_collider_y)
        {
            let combined_radius_squared =
                (collider_circle_radius + collidee_circle.radius) *
                (collider_circle_radius + collidee_circle.radius);

            // Check to see if the collision happens before the full rotation completed
            if
                combined_radius_squared <
                (collidee_circle.center_x - full_collider_x) *
                    (collidee_circle.center_x - full_collider_x) +
                    (collidee_circle.center_y - full_collider_y) *
                        (collidee_circle.center_y - full_collider_y)
            {
                // Then reset the collision list because collisions with the current r_move aren't happening.
                if collidee_circle.circle_entity_type == CircleEntityType::Organism {
                    *involved_blobs = vec![blob_number, collidee_circle.blob_number];
                    *involved_minerals = false;
                } else {
                    *involved_blobs = vec![blob_number];
                    *involved_minerals = true;
                }

                // This calculates the two points the collider could collider with the collidee.
                let points_of_collisions = solve_two_circle_intersection(
                    center_of_mass_x_after_xymove,
                    center_of_mass_y_after_xymove,
                    collider_distance_center_of_mass,
                    collidee_circle.center_x,
                    collidee_circle.center_y,
                    collidee_circle.radius
                );

                // This determines which one is closer, which is the one that actually happens.
                let initial_angle =
                    deterministic_trig.d_trig.arccosine((
                        ((collider_x_after_xymove - center_of_mass_x_after_xymove) * 1000) /
                            collider_distance_center_of_mass,
                        1000,
                    )).0 * (collider_y_after_xymove - center_of_mass_y_after_xymove).signum();
                let final_angle_1 =
                    deterministic_trig.d_trig.arccosine((
                        ((points_of_collisions.0.0 - center_of_mass_x_after_xymove) * 1000) /
                            collider_distance_center_of_mass,
                        1000,
                    )).0 * (points_of_collisions.0.1 - center_of_mass_y_after_xymove).signum();
                let final_angle_2 =
                    deterministic_trig.d_trig.arccosine((
                        ((points_of_collisions.1.0 - center_of_mass_x_after_xymove) * 1000) /
                            collider_distance_center_of_mass,
                        1000,
                    )).0 * (points_of_collisions.1.1 - center_of_mass_y_after_xymove).signum();

                if (final_angle_1 - initial_angle).abs() < final_angle_2 - final_angle_2 {
                    *r_move = final_angle_1;
                } else {
                    *r_move = final_angle_2;
                }

                // addresses the possibility a rounding error made it so that there is now overlap.
                let mut partial_collider_x =
                center_of_mass_x_after_xymove +
                    (collider_distance_center_of_mass *
                        deterministic_trig.d_trig.cosine((*r_move, 1000)).0) /
                        1000;
                let mut partial_collider_y =
                center_of_mass_y_after_xymove +
                    (collider_distance_center_of_mass *
                        deterministic_trig.d_trig.sine((*r_move, 1000)).0) /
                        1000;

                while
                    (collider_circle_radius + collidee_circle.radius) *
                        (collider_circle_radius + collidee_circle.radius) <
                        (collidee_circle.center_x - partial_collider_x) *
                            (collidee_circle.center_x - partial_collider_x) +
                            (collidee_circle.center_y - partial_collider_y) *
                                (collidee_circle.center_y - partial_collider_y) &&
                    *r_move > 0
                {
                    *r_move = *r_move - r_move.signum();

                    partial_collider_x =
                    center_of_mass_x_after_xymove +
                        (collider_distance_center_of_mass *
                            deterministic_trig.d_trig.cosine((*r_move, 1000)).0) /
                            1000;
                    partial_collider_y =
                    center_of_mass_y_after_xymove +
                        (collider_distance_center_of_mass *
                            deterministic_trig.d_trig.sine((*r_move, 1000)).0) /
                            1000;
                }

                // This covers the case in which another collision occurs exactly at the r_move
            } else {
                // If is has already collided with something else.
                if *r_move != original_r_move {
                    // The collidee entity number just needs to be added.
                    if collidee_circle.circle_entity_type == CircleEntityType::Organism {
                        involved_blobs.push(collidee_circle.blob_number)
                        // Or if it is a mineral then the boolean needs to be marked true.
                    } else {
                        *involved_minerals = true;
                    }
                }
            }
        }
    }
}
