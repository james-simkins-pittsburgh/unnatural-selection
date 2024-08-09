use crate::simulation::{biosphere_simulation::blob_mover::CollisionCheckResult, AllBiosphereInformation};

pub fn detect_collision(
    all_biosphere_information: &mut AllBiosphereInformation,
    blob_number: usize,
) -> CollisionCheckResult {

    // This is just a placeholder return that will not detect any collisions.
    return CollisionCheckResult {
        collision: false,
        x_move: all_biosphere_information.blob_vec[blob_number].blob_x_velocity,
        y_move: all_biosphere_information.blob_vec[blob_number].blob_y_velocity,
        rotation_in_thousandth_radians: all_biosphere_information.blob_vec[blob_number].angular_velocity,
        involved_blobs: vec![blob_number],
        involved_minerals: vec![],
    }
}
