use crate::{
    settings::GameSettings,
    simulation::{
        biosphere_simulation::blob_mover::CollisionCheckResult,
        AllBiosphereInformation,
        CircleEntityType,
    },
};

struct CircleCheckResults {
    collision: bool,
    collision_pairs_vec: Vec<(CircleInfo, CircleInfo)>,
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
    entity_type: CircleEntityType,
    id_number: usize,
}

pub fn detect_collision(
    all_biosphere_information: &AllBiosphereInformation,
    blob_number: usize,
    game_settings: &GameSettings
) -> CollisionCheckResult {
    let mut collider_circles: Vec<CircleInfo> = Vec::new();

    for organism_number in all_biosphere_information.blob_vec[blob_number].blob_members.iter() {
        collider_circles.push(CircleInfo {
            x: all_biosphere_information.organism_information_vec[*organism_number].x_location,
            y: all_biosphere_information.organism_information_vec[*organism_number].y_location,
            radius: all_biosphere_information.organism_information_vec[*organism_number].y_location,
            entity_type: CircleEntityType::Organism,
            id_number: *organism_number,
        });

        if all_biosphere_information.organism_information_vec[*organism_number].oblong {
            for circle in all_biosphere_information.organism_information_vec[
                *organism_number
            ].other_circle_positions.iter() {
                collider_circles.push(CircleInfo {
                    x: circle.x,
                    y: circle.y,
                    radius: circle.radius,
                    entity_type: CircleEntityType::Organism,
                    id_number: *organism_number,
                });
            }
        }
    }

    // Send all the circles to the helper function to work out what, if anything, collides first.
    let collision_check_result = check_circles(
        collider_circles,
        &all_biosphere_information,
        &game_settings
    );

    // Send the results back to the blob mover.
    return CollisionCheckResult {
        collision: collision_check_result.collision,
        x_move: collision_check_result.x_move,
        y_move: collision_check_result.y_move,
        rotation_in_thousandth_radians: collision_check_result.rotation_in_thousandth_radians,
        involved_blobs: collision_check_result.involved_blobs,
        involved_minerals: collision_check_result.involved_minerals,
    };
}
// This helper function consults the detection grid to determine if any collisions will occur with the movement.
fn check_circles(
    collider_circles: Vec<CircleInfo>,
    all_biosphere_information: &AllBiosphereInformation,
    game_settings: &GameSettings
) -> CollisionCheckResult {}
