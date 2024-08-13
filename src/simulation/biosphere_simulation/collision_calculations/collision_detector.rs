use crate::{
    settings::GameSettings,
    simulation::{
        biosphere_simulation::blob_mover::CollisionCheckResult,
        AllBiosphereInformation,
        CircleEntityType,
        CirclePositionRecord,
    },
};

struct CollisionCheckResults {
    collision: bool,
    x_move: i32,
    y_move: i32,
    rotation_in_thousandth_radians: i32,
    involved_blobs: Vec<usize>,
    involved_minerals: Vec<usize>,
}
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
                &mut involved_blobs,
                &mut involved_minerals,
                blob_number,
                collider_circle,
                collidee_circle
            );
        }
    }

    return CollisionCheckResult {
        collision: todo!(),
        x_move: todo!(),
        y_move: todo!(),
        rotation_in_thousandth_radians: todo!(),
        involved_blobs: todo!(),
        involved_minerals: todo!(),
    };
}

fn check_two_circles_translational(
    mut x_move: &mut i32,
    mut y_move: &mut i32,
    mut involved_blobs: &mut Vec<usize>,
    mut involved_minerals: &mut bool,
    blob_number: usize,
    collider_circle: &CircleInfo,
    collidee_circle: &CirclePositionRecord
) {
    if
        collidee_circle.identity_number != blob_number ||
        collidee_circle.circle_entity_type != CircleEntityType::Organism
    {
        if
            (collidee_circle.center_x - (collider_circle.x + *x_move)) *
                (collidee_circle.center_x - (collider_circle.x + *x_move)) +
                (collidee_circle.center_y - (collider_circle.y + *y_move)) *
                    (collidee_circle.center_y - (collider_circle.y + *y_move)) <
            (collidee_circle.radius + collider_circle.radius) *
                (collidee_circle.radius + collider_circle.radius)
        {
            if
                (collidee_circle.center_x -
                    collider_circle.x -
                    (collidee_circle.radius + collider_circle.radius) * 1000) /
                    *x_move >
                (collidee_circle.center_y -
                    collider_circle.y -
                    (collidee_circle.radius + collider_circle.radius) * 1000) /
                    *y_move
            {
                // Left off here

                if
                    collidee_circle.center_x -
                        collider_circle.x -
                        (collidee_circle.radius + collider_circle.radius) == *x_move
                {
                    if
                        collidee_circle.circle_entity_type == CircleEntityType::Mineral
                    {
                        *involved_minerals = true;

                    } else if
                        collidee_circle.circle_entity_type == CircleEntityType::Organism &&
                        !involved_blobs.contains(&collidee_circle.identity_number)
                    {
                        involved_blobs.push(collidee_circle.identity_number);
                    }
                } else if
                    collidee_circle.center_x -
                        collider_circle.x -
                        (collidee_circle.radius + collider_circle.radius) < *x_move
                {
                    // Logic Here

                }
            } else {
            }
        }
    }
}
