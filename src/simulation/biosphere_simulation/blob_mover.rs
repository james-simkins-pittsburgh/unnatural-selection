use crate::simulation::AllBiosphereInformation;
use crate::utility_functions::deterministic_trigonometry::DeterministicTrig;
use crate::simulation::biosphere_simulation::collision_calculations::collision_detector::detect_collision;
use crate::simulation::biosphere_simulation::collision_calculations::organism_collisions::apply_collision;

pub struct CollisionCheckResult {
    pub collision: bool,
    pub x_move: i32,
    pub y_move: i32,
    pub involved_blobs: Vec<usize>,
    pub involved_minerals: Vec<usize>,
}

pub fn move_blob(
    all_biosphere_information: &mut AllBiosphereInformation,
    deterministic_trig: &DeterministicTrig,
    blob_number: usize
) {
    let detection_result = detect_collision(all_biosphere_information, blob_number);

    for member_number in 0..all_biosphere_information.blob_vec[blob_number].blob_members.len() {
        // This is the only place in the code allowed to move organism.
        all_biosphere_information.organism_information_vec[all_biosphere_information.blob_vec[blob_number].blob_members[member_number]].x_location +=
            detection_result.x_move;
        all_biosphere_information.organism_information_vec[all_biosphere_information.blob_vec[blob_number].blob_members[member_number]].x_location +=
            detection_result.y_move;
    }

    if detection_result.collision {
        apply_collision(all_biosphere_information, &detection_result.involved_blobs);
    }
}
