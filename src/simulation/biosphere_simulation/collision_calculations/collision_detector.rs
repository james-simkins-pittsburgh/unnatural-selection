use crate::{
    settings::GameSettings,
    simulation::{ biosphere_simulation::blob_mover::CollisionCheckResult, AllBiosphereInformation, CircleEntityType },
};

struct CircleCheckResults {
    collision: bool,
    collision_pairs_vec: Vec<(CircleInfo, CircleInfo)>
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

    for organism_number in all_biosphere_information.blob_vec[blob_number].blob_members.iter() {
        if all_biosphere_information.organism_information_vec[*organism_number].oblong {
          
        
        } else {
          
        }
    }

    // This is just a placeholder return that will not detect any collisions.
    return CollisionCheckResult {
        collision: false,
        x_move: all_biosphere_information.blob_vec[blob_number].blob_x_velocity,
        y_move: all_biosphere_information.blob_vec[blob_number].blob_y_velocity,
        rotation_in_thousandth_radians: all_biosphere_information.blob_vec
            [blob_number].angular_velocity,
        involved_blobs: vec![blob_number],
        involved_minerals: vec![],
    };
}

// This helper function consults the detection grid to determine if any collisions will occur with the movement.
fn check_circles(
    input_circle_vec: Vec <CircleInfo>,
    all_biosphere_information: &AllBiosphereInformation,
    game_settings: &GameSettings
) -> CircleCheckResult {


}
