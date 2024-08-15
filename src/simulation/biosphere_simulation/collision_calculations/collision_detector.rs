use crate::{
    settings::GameSettings,
    simulation::{
        biosphere_simulation::blob_mover::CollisionCheckResult,
        AllBiosphereInformation,
        CircleEntityType,
        CirclePositionRecord,
    },
};

struct CircleInfo {
    x: i32,
    y: i32,
    radius: i32,
}

pub fn detect_collision(
    all_biosphere_information: &AllBiosphereInformation,
    blob_number: usize,
    game_settings: &GameSettings
) -> CollisionCheckResult {
    let mut collider_circles: Vec<CircleInfo> = Vec::new();

    // This makes a vec of all the circles of the collider.

    for organism_number in all_biosphere_information.blob_vec[blob_number].blob_members.iter() {
        collider_circles.push(CircleInfo {
            x: all_biosphere_information.organism_information_vec[*organism_number].x_location,
            y: all_biosphere_information.organism_information_vec[*organism_number].y_location,
            radius: all_biosphere_information.organism_information_vec[*organism_number].y_location,
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
    return check_circles(collider_circles, &all_biosphere_information, &game_settings, blob_number);
}

// This helper function consults the detection grid to determine if any collisions will occur with the movement.
fn check_circles(
    collider_circles: Vec<CircleInfo>,
    all_biosphere_information: &AllBiosphereInformation,
    game_settings: &GameSettings,
    blob_number: usize
) -> CollisionCheckResult {
    let mut x_move = all_biosphere_information.blob_vec[blob_number].blob_x_velocity;
    let mut y_move = all_biosphere_information.blob_vec[blob_number].blob_y_velocity;
    let mut r_move = all_biosphere_information.blob_vec[blob_number].angular_velocity;

    let original_x_move = x_move;
    let original_y_move = y_move;

    let mut involved_blobs = vec![blob_number];
    let mut involved_minerals = false;

    // Iterate over every collider circle.
    for collider_circle in collider_circles.iter() {
        // Iterate over every collidee circle in the detection grid.
        for collidee_circle in all_biosphere_information.collision_detection_grid[
            ((collider_circle.x - game_settings.map_length / 2) / 10000) as usize
        ][((collider_circle.x - game_settings.map_length / 2) / 10000) as usize].iter() {
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
    collider_circle: &CircleInfo,
    collidee_circle: &CirclePositionRecord
) {
    // If the circles are not part of the same blob.
    if
        collidee_circle.identity_number != blob_number ||
        collidee_circle.circle_entity_type != CircleEntityType::Organism
    {
        // If they're going to collide with the current x and y moves.
        if
            (collidee_circle.center_x - (collider_circle.x + *x_move)) *
                (collidee_circle.center_x - (collider_circle.x + *x_move)) +
                (collidee_circle.center_y - (collider_circle.y + *y_move)) *
                    (collidee_circle.center_y - (collider_circle.y + *y_move)) <
            (collidee_circle.radius + collider_circle.radius) *
                (collidee_circle.radius + collider_circle.radius)
        {
            /* If x is moving toward the direction of a collision and it is either closer to 
                a collision in proportion to the x movement amount than y is in proportion to y movement 
                or y is moving in the wrong direction for a collision. */
            if
                (collidee_circle.center_x - collider_circle.x).signum() == x_move.signum() &&
                (((collidee_circle.center_x - collider_circle.x).abs() -
                    (collidee_circle.radius + collider_circle.radius) * 1000) /
                    x_move.abs() <
                    ((collidee_circle.center_y - collider_circle.y).abs() -
                        (collidee_circle.radius + collider_circle.radius) * 1000) /
                        y_move.abs() || collider_circle.y.signum() != y_move.signum())
            {
                if
                    // If the amount it moves before it hits is exactly the current x move.
                    (collidee_circle.center_x - collider_circle.x).abs() -
                        (collidee_circle.radius + collider_circle.radius) == *x_move
                {
                    // If the collidee is a mineral
                    if collidee_circle.circle_entity_type == CircleEntityType::Mineral {
                        // Then mark minerals involved as true
                        *involved_minerals = true;

                        // If it is an organism
                    } else if
                        collidee_circle.circle_entity_type == CircleEntityType::Organism &&
                        !involved_blobs.contains(&collidee_circle.identity_number)
                    {
                        // Then add that blob to the list.
                        involved_blobs.push(collidee_circle.identity_number);
                    }
                } else if
                    // If the amount is moved before it hits is less than the current x move.
                    (collidee_circle.center_x - collider_circle.x).abs() -
                        (collidee_circle.radius + collider_circle.radius) < *x_move
                {
                    // That becomes the new x move.
                    *x_move =
                        ((collidee_circle.center_x - collider_circle.x).abs() -
                            (collidee_circle.radius + collider_circle.radius)) *
                        original_y_move.signum();

                    // The list of involved blobs and minerals is cleared.
                    *involved_blobs = vec![blob_number];
                    *involved_minerals = false;

                    // If it is a mineral
                    if collidee_circle.circle_entity_type == CircleEntityType::Mineral {
                        // Then mark involved minerals true.
                        *involved_minerals = true;

                        // Otherwise, the organism number is added to the vec
                    } else if
                        collidee_circle.circle_entity_type == CircleEntityType::Organism &&
                        !involved_blobs.contains(&collidee_circle.identity_number)
                    {
                        involved_blobs.push(collidee_circle.identity_number);
                    }

                    // The y move is set to be proportional to the x move based on the original moves.
                    *y_move = (*x_move * original_y_move) / original_x_move;
                }
                // Since we know it hits and it doesn't hit first on x, it must be the y that hits first.
            } else {
                if
                    // If the amount it moves before it hits is exactly the current y move.
                    (collidee_circle.center_y - collider_circle.y).abs() -
                        (collidee_circle.radius + collider_circle.radius) == *y_move
                {
                    // If the collidee is a mineral
                    if collidee_circle.circle_entity_type == CircleEntityType::Mineral {
                        // Then mark minerals involved as true
                        *involved_minerals = true;

                        // If it is an organism
                    } else if
                        collidee_circle.circle_entity_type == CircleEntityType::Organism &&
                        !involved_blobs.contains(&collidee_circle.identity_number)
                    {
                        // Then add that blob to the list.
                        involved_blobs.push(collidee_circle.identity_number);
                    }
                } else if
                    // If the amount is moved before it hits is less than the current y move.
                    (collidee_circle.center_y - collider_circle.y).abs() -
                        (collidee_circle.radius + collider_circle.radius) < *y_move
                {
                    // That becomes the new y move.
                    *y_move =
                        ((collidee_circle.center_y - collider_circle.y).abs() -
                            (collidee_circle.radius + collider_circle.radius)) *
                        original_y_move.signum();

                    // The list of involved blobs and minerals is cleared.
                    *involved_blobs = vec![blob_number];
                    *involved_minerals = false;

                    // If it is a mineral
                    if collidee_circle.circle_entity_type == CircleEntityType::Mineral {
                        // Then mark involved minerals true.
                        *involved_minerals = true;

                        // Otherwise, the organism number is added to the vec
                    } else if
                        collidee_circle.circle_entity_type == CircleEntityType::Organism &&
                        !involved_blobs.contains(&collidee_circle.identity_number)
                    {
                        involved_blobs.push(collidee_circle.identity_number);
                    }

                    // The y move is set to be proportional to the x move based on the original moves.
                    *x_move = (*y_move * original_x_move) / original_y_move;
                }
            }
        }
    }
}
