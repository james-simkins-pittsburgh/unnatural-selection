use crate::{simulation::CircleEntityType, utility_functions::{
        integer_math::square_root_64,
        quadratic_solver,
    }};

use super::{CollideeCircleInfo, ColliderCircleInfo};

pub fn check_two_circles_translational(
    x_move: &mut i32,
    y_move: &mut i32,
    original_x_move: i32,
    original_y_move: i32,
    involved_blobs: &mut Vec<usize>,
    mineral_involved: &mut bool,
    blob_number: usize,
    collider_circle: &ColliderCircleInfo,
    collidee_circle: &CollideeCircleInfo,
) {
    // If the circles are not part of the same blob.
    if
        collidee_circle.blob_number != blob_number ||
        collidee_circle.circle_entity_type == CircleEntityType::Mineral
    {
        // If they're going to collide with less than or equal to the current x and y moves.
        if
            ((collidee_circle.x - (collider_circle.x + *x_move)) as i64) *
                ((collidee_circle.x - (collider_circle.x + *x_move)) as i64) +
                ((collidee_circle.y - (collider_circle.y + *y_move)) as i64) *
                    ((collidee_circle.y - (collider_circle.y + *y_move)) as i64) <=
            ((collidee_circle.radius + collider_circle.radius) as i64) *
                ((collidee_circle.radius + collider_circle.radius) as i64)
        {
            // Write these to memory to simplify code and reduce repetitive calculations
            let current_x_distance_squared =
                ((collidee_circle.x - collider_circle.x) as i64) *
                ((collidee_circle.x - collider_circle.x) as i64);

            let current_y_distance_squared =
                ((collidee_circle.y - collider_circle.y) as i64) *
                ((collidee_circle.y - collider_circle.y) as i64);

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
                        *mineral_involved = true;
                    }

                    // If they weren't already 0
                } else {
                    // Zero out the movement.
                    *x_move = 0;
                    *y_move = 0;

                    // Clear the collision list except for those blobs.
                    if collidee_circle.circle_entity_type == CircleEntityType::Organism {
                        *involved_blobs = vec![blob_number, collidee_circle.blob_number];
                        *mineral_involved = false;
                    } else {
                        *involved_blobs = vec![blob_number];
                        *mineral_involved = true;
                    }
                }
                // Since the moves are 0, the function can be over.
                return;
            }

            // Write these to memory to simplify code and reduce repetitive calculations

            let future_x_distance_squared =
                ((collidee_circle.x - (collider_circle.x + *x_move)) as i64) *
                ((collidee_circle.x - (collider_circle.x + *x_move)) as i64);

            let future_y_distance_squared =
                ((collidee_circle.y - (collider_circle.y + *y_move)) as i64) *
                ((collidee_circle.y - (collider_circle.y + *y_move)) as i64);

            // If they're going to collide with less than the current x and y moves.
            if future_x_distance_squared + future_y_distance_squared < combined_radii_squared {
                // Then reset the collision list because collisions with x_move and y_move aren't happening.
                if collidee_circle.circle_entity_type == CircleEntityType::Organism {
                    *involved_blobs = vec![blob_number, collidee_circle.blob_number];
                    *mineral_involved = false;
                } else {
                    *involved_blobs = vec![blob_number];
                    *mineral_involved = true;
                }

                // Make sure x_move isn't 0.
                if *x_move != 0 {
                    let slope_x_1000 =
                        ((original_y_move as i128) * 1000) / (original_x_move as i128);
                    let x1 = collider_circle.x as i128;
                    let y1 = collider_circle.y as i128;
                    let x2 = collidee_circle.x as i128;
                    let y2 = collidee_circle.y as i128;

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
                            collidee_circle.y +
                            (
                                square_root_64(
                                    combined_radii_squared - current_x_distance_squared
                                ) as i32
                            );

                        // This is the other solution to the quadratic.
                        let y_of_collider_at_collision_two =
                            collidee_circle.y -
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
                                    ((collidee_circle.y - (collider_circle.y + *y_move)) *
                                        (collidee_circle.y -
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
                    *mineral_involved = true;
                }
            }
        }
    }
}