use crate::{
    settings::GameSettings,
    simulation::{
        biosphere_simulation::blob_mover::CollisionCheckResult,
        AllBiosphereInformation,
        CircleEntityType,
        CirclePositionRecord,
    },
    utility_functions::{ deterministic_trigonometry::DeterministicTrig, integer_math::square_root },
};

// This stores the circle information for the colliders and collidees.
struct CircleInfo {
    x: i32,
    y: i32,
    radius: i32,
}

pub fn detect_collision(
    all_biosphere_information: &AllBiosphereInformation,
    blob_number: usize,
    game_settings: &GameSettings,
    deterministic_trig: &DeterministicTrig
) -> CollisionCheckResult {
    let mut collider_circles: Vec<CircleInfo> = Vec::new();

    // This makes a vec of all the circles of the collider blob.
    for organism_number in all_biosphere_information.blob_vec[blob_number].blob_members.iter() {
        collider_circles.push(CircleInfo {
            x: all_biosphere_information.organism_information_vec[*organism_number].x_location,
            y: all_biosphere_information.organism_information_vec[*organism_number].y_location,
            radius: all_biosphere_information.organism_information_vec[*organism_number].radius,
        });

        if all_biosphere_information.organism_information_vec[*organism_number].oblong {
            for circle in all_biosphere_information.organism_information_vec[
                *organism_number
            ].other_circle_positions.iter() {
                collider_circles.push(CircleInfo {
                    x: circle.x,
                    y: circle.y,
                    radius: circle.radius,
                });
            }
        }
    }

    // Send all the circles to the helper function to work out what, if anything, collides first
    // Then send the results back to the blob mover.
    return check_circles(
        collider_circles,
        &all_biosphere_information,
        &game_settings,
        blob_number,
        deterministic_trig
    );
}

// This helper function consults the detection grid to determine if any collisions will occur with the movement of the blob.
fn check_circles(
    collider_circles: Vec<CircleInfo>,
    all_biosphere_information: &AllBiosphereInformation,
    game_settings: &GameSettings,
    blob_number: usize,
    _deterministic_trig: &DeterministicTrig
) -> CollisionCheckResult {
    // These store the maximum movement before a collision (if any) occurs.
    let mut x_move = all_biosphere_information.blob_vec[blob_number].blob_x_velocity;
    let mut y_move = all_biosphere_information.blob_vec[blob_number].blob_y_velocity;
    let mut _r_move = all_biosphere_information.blob_vec[blob_number].angular_velocity;

    // This stores the original moves so it can be references later.
    let original_x_move = x_move;
    let original_y_move = y_move;

    // This is a list of all involved blobs so calculations can be done to get new velocities after a collision.
    let mut involved_blobs = vec![blob_number];

    // This keeps track if a mineral is involved. If one is, then the entire collision will result in 0 velocities.
    let mut involved_minerals = false;

    // Iterates over every collider circle.
    for collider_circle in collider_circles.iter() {
        // Iterates over every collidee circle in the detection grid.
        for collidee_circle in all_biosphere_information.collision_detection_grid[
            ((collider_circle.x + game_settings.map_length / 2) / 10000) as usize
        ][((collider_circle.y + game_settings.map_height / 2) / 10000) as usize].iter() {
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
        r_move: _r_move,
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
    collider_circle: &CircleInfo,
    collidee_circle: &CirclePositionRecord
) {
    // If the circles are not part of the same blob.
    if
        collidee_circle.blob_number != blob_number ||
        collidee_circle.circle_entity_type == CircleEntityType::Mineral
    {
        // If they're going to collide with less than or equal to the current x and y moves.
        if
            (collidee_circle.center_x - (collider_circle.x + *x_move)) *
                (collidee_circle.center_x - (collider_circle.x + *x_move)) +
                (collidee_circle.center_y - (collider_circle.y + *y_move)) *
                    (collidee_circle.center_y - (collider_circle.y + *y_move)) <=
            (collidee_circle.radius + collider_circle.radius) *
                (collidee_circle.radius + collider_circle.radius)
        {
            // If they're going to collide with less than the current x and y moves.
            if
                (collidee_circle.center_x - (collider_circle.x + *x_move)) *
                    (collidee_circle.center_x - (collider_circle.x + *x_move)) +
                    (collidee_circle.center_y - (collider_circle.y + *y_move)) *
                        (collidee_circle.center_y - (collider_circle.y + *y_move)) <
                (collidee_circle.radius + collider_circle.radius) *
                    (collidee_circle.radius + collider_circle.radius)
            {
                // Then reset the collision list because collisions with x_move and y_move aren't happening.
                if collidee_circle.circle_entity_type == CircleEntityType::Organism {
                    *involved_blobs = vec![blob_number, collidee_circle.blob_number];
                    *involved_minerals = false;
                } else {
                    *involved_blobs = vec![blob_number];
                    *involved_minerals = true;
                }

                // Check for the case in which the circles were already overlapping because of an error.
                if
                    (collidee_circle.center_x - collider_circle.x) *
                        (collidee_circle.center_x - collider_circle.x) +
                        (collidee_circle.center_y - collider_circle.y) *
                            (collidee_circle.center_y - collider_circle.y) <
                    (collidee_circle.radius + collider_circle.radius) *
                        (collidee_circle.radius + collider_circle.radius)
                {
                    // In the case of such an overlap, set x_move and y_move to 0.
                    *x_move = 0;
                    *y_move = 0;
                }

                // Make sure x_move isn't 0.
                if *x_move != 0 {
                    // This monstrosity of an equation is an application of the quadratic equation to find the x where the circles collide.
                    // This is the first solution to the quadratic.
                    let x_of_collider_at_collision_one = ((-(
                        2 *
                            (original_y_move as i64) *
                            (((-original_y_move as i64) * (collider_circle.x as i64)) /
                                ((original_x_move as i64) * (original_x_move as i64)) +
                                (collider_circle.y as i64) -
                                (collidee_circle.center_y as i64)) -
                        2 * (collidee_circle.center_x as i64)
                    ) +
                        square_root(
                            (2 *
                                (original_y_move as i64) *
                                (((-original_y_move as i64) * (collider_circle.x as i64)) /
                                    ((original_x_move as i64) * (original_x_move as i64)) +
                                    (collider_circle.y as i64) -
                                    (collidee_circle.center_y as i64)) -
                                2 * (collidee_circle.center_x as i64)) *
                                (2 *
                                    (original_y_move as i64) *
                                    (((-original_y_move as i64) * (collider_circle.x as i64)) /
                                        ((original_x_move as i64) * (original_x_move as i64)) +
                                        (collider_circle.y as i64) -
                                        (collidee_circle.center_y as i64)) -
                                    2 * (collidee_circle.center_x as i64)) -
                                4 *
                                    (1 +
                                        ((original_y_move as i64) * (original_y_move as i64)) /
                                            ((original_x_move as i64) * (original_x_move as i64))) *
                                    ((collidee_circle.center_x as i64) *
                                        (collidee_circle.center_x as i64) +
                                        (-(
                                            ((original_y_move as i64) *
                                                (collider_circle.x as i64)) /
                                            (original_x_move as i64)
                                        ) +
                                            (collider_circle.y as i64) -
                                            (collidee_circle.center_y as i64)) *
                                            (-(
                                                ((original_y_move as i64) *
                                                    (collider_circle.x as i64)) /
                                                (original_x_move as i64)
                                            ) +
                                                (collider_circle.y as i64) -
                                                (collidee_circle.center_y as i64)) -
                                        ((collider_circle.radius as i64) +
                                            (collidee_circle.radius as i64)) *
                                            ((collider_circle.radius as i64) +
                                                (collidee_circle.radius as i64)) *
                                            (((collider_circle.radius as i64) +
                                                (collidee_circle.radius as i64)) *
                                                ((collider_circle.radius as i64) +
                                                    (collidee_circle.radius as i64))))
                        )) /
                        (2 *
                            (1 +
                                ((original_y_move as i64) * (original_y_move as i64)) /
                                    ((original_x_move as i64) * (original_x_move as i64))))) as i32;

                    // This is the second solution to the quadratic.
                    let x_of_collider_at_collision_two = ((-(
                        2 *
                            (original_y_move as i64) *
                            (((-original_y_move as i64) * (collider_circle.x as i64)) /
                                ((original_x_move as i64) * (original_x_move as i64)) +
                                (collider_circle.y as i64) -
                                (collidee_circle.center_y as i64)) -
                        2 * (collidee_circle.center_x as i64)
                    ) -
                        square_root(
                            (2 *
                                (original_y_move as i64) *
                                (((-original_y_move as i64) * (collider_circle.x as i64)) /
                                    ((original_x_move as i64) * (original_x_move as i64)) +
                                    (collider_circle.y as i64) -
                                    (collidee_circle.center_y as i64)) -
                                2 * (collidee_circle.center_x as i64)) *
                                (2 *
                                    (original_y_move as i64) *
                                    (((-original_y_move as i64) * (collider_circle.x as i64)) /
                                        ((original_x_move as i64) * (original_x_move as i64)) +
                                        (collider_circle.y as i64) -
                                        (collidee_circle.center_y as i64)) -
                                    2 * (collidee_circle.center_x as i64)) -
                                4 *
                                    (1 +
                                        ((original_y_move as i64) * (original_y_move as i64)) /
                                            ((original_x_move as i64) * (original_x_move as i64))) *
                                    ((collidee_circle.center_x as i64) *
                                        (collidee_circle.center_x as i64) +
                                        (-(
                                            ((original_y_move as i64) *
                                                (collider_circle.x as i64)) /
                                            (original_x_move as i64)
                                        ) +
                                            (collider_circle.y as i64) -
                                            (collidee_circle.center_y as i64)) *
                                            (-(
                                                ((original_y_move as i64) *
                                                    (collider_circle.x as i64)) /
                                                (original_x_move as i64)
                                            ) +
                                                (collider_circle.y as i64) -
                                                (collidee_circle.center_y as i64)) -
                                        ((collider_circle.radius as i64) +
                                            (collidee_circle.radius as i64)) *
                                            ((collider_circle.radius as i64) +
                                                (collidee_circle.radius as i64)) *
                                            (((collider_circle.radius as i64) +
                                                (collidee_circle.radius as i64)) *
                                                ((collider_circle.radius as i64) +
                                                    (collidee_circle.radius as i64))))
                        )) /
                        (2 *
                            (1 +
                                ((original_y_move as i64) * (original_y_move as i64)) /
                                    ((original_x_move as i64) * (original_x_move as i64))))) as i32;

                    // If is x collision 1 is a closer than x collision 2 then set x move to it. Otherwise, set x move to collision 2.
                    if
                        (x_of_collider_at_collision_one - collider_circle.x).abs() <
                        (x_of_collider_at_collision_two - collider_circle.x).abs()
                    {
                        *x_move = x_of_collider_at_collision_one - collider_circle.x;
                    } else {
                        *x_move = x_of_collider_at_collision_two - collider_circle.x;
                    }

                    // Make sure x_move is not 0.
                    if *x_move != 0 {
                        // Set y_move based on the fact that movement will be proportional to the full movement before collision.
                        *y_move = (*x_move * original_y_move) / original_x_move;

                        // Check to make sure rounding errors didn't move this past the collision point. Fix it if it did.
                        while
                            x_move.abs() > 0 &&
                            (collidee_circle.center_x - (collider_circle.x + *x_move)) *
                                (collidee_circle.center_x - (collider_circle.x + *x_move)) +
                                (collidee_circle.center_y - (collider_circle.y + *y_move)) *
                                    (collidee_circle.center_y - (collider_circle.y + *y_move)) <
                                (collidee_circle.radius + collider_circle.radius) *
                                    (collidee_circle.radius + collider_circle.radius)
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
                        // In the case where x_move is 0, y_move should be set to 0 too.
                        *y_move = 0;
                    }

                    // In the case where x__move is 0 from the beginning of the function.
                } else {
                    // If x_move is 0 because x_move was originally 0 for the blob.
                    if original_x_move == 0 {
                        // Then y of the collider at the collision points can be calculated as follows.
                        let y_of_collider_at_collision_one =
                            (collidee_circle.center_y as i32) +
                            (
                                square_root(
                                    ((collidee_circle.radius + collider_circle.radius) *
                                        (collidee_circle.radius + collider_circle.radius) -
                                        (collidee_circle.center_x - collider_circle.x) *
                                            (collidee_circle.center_x - collider_circle.x)) as i64
                                ) as i32
                            );

                        // This is the other solution to the quadratic.
                        let y_of_collider_at_collision_two =
                            (collidee_circle.center_y as i32) -
                            (
                                square_root(
                                    ((collidee_circle.radius + collider_circle.radius) *
                                        (collidee_circle.radius + collider_circle.radius) -
                                        (collidee_circle.center_x - collider_circle.x) *
                                            (collidee_circle.center_x - collider_circle.x)) as i64
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
                            x_move.abs() > 0 &&
                            (collidee_circle.center_x - collider_circle.x) *
                                (collidee_circle.center_x - collider_circle.x) +
                                (collidee_circle.center_y - (collider_circle.y + *y_move)) *
                                    (collidee_circle.center_y - (collider_circle.y + *y_move)) <
                                (collidee_circle.radius + collider_circle.radius) *
                                    (collidee_circle.radius + collider_circle.radius)
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
            } else {
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
