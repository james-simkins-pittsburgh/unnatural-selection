use crate::simulation::{
    biosphere_simulation::blob_mover::CollisionCheckResult,
    AllBiosphereInformation,
};

struct CircleCheckResult {
    collision: bool,
    involved_blobs: Vec<usize>,
    involved_minerals: Vec<usize>,
}

pub fn detect_collision(
    all_biosphere_information: &AllBiosphereInformation,
    blob_number: usize
) -> CollisionCheckResult {
    let mut involved_blobs: Vec<(usize, i32)> = Vec::new();
    let mut involved_minerals: Vec<(usize, i32)> = Vec::new();
    let mut percent_translational_motion_before_collision: i32 = 100;

    for organism_number in all_biosphere_information.blob_vec[blob_number].blob_members.iter() {
        let mut circle_check_result = check_circle(
            all_biosphere_information.organism_information_vec[*organism_number].x_location,
            all_biosphere_information.organism_information_vec[*organism_number].y_location,
            all_biosphere_information.organism_information_vec[*organism_number].radius,
            blob_number,
            &all_biosphere_information
        );

        if circle_check_result.collision {
            // Need to add logic to find percent motion if there is a collision

        }

        if all_biosphere_information.organism_information_vec[*organism_number].oblong {
            for circle in all_biosphere_information.organism_information_vec
                [*organism_number].other_circle_positions.iter() {
                circle_check_result = check_circle(
                    circle.x,
                    circle.y,
                    circle.radius,
                    blob_number,
                    &all_biosphere_information
                );

                if circle_check_result.collision {
                    // Need to add logic to find percent motion if there is a collision
        
                }
            }
            
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

fn check_circle(
    x: i32,
    y: i32,
    radius: i32,
    blob_number: usize,
    all_biosphere_information: &AllBiosphereInformation
) -> CircleCheckResult {
    return CircleCheckResult {
        collision: false,
        involved_blobs: vec![],
        involved_minerals: vec![],
    };
}
