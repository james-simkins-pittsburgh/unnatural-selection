use collider_circle_list_maker::make_collider_circle_list;

use crate::{
    settings::GameSettings,
    simulation::{
        biosphere_simulation::blob_mover::CollisionCheckResult,
        AllSpatialBiosphereInformation,
        CircleEntityType,
    },
    utility_functions::{
        deterministic_trigonometry::DeterministicTrig,
        integer_math::square_root_64,
        quadratic_solver,
        two_circles_intersection_solver::solve_two_circle_intersection,
    },
};

// This module makes a list of collider circles.
pub mod collider_circle_list_maker;

// This module makes a list of collidee circles.
pub mod collidee_circle_list_maker;

// This module tests for collisions from translational motion.
pub mod translational_collision_detector;

// This module tests for collisions from angular motion.
pub mod angular_collision_detector;

// This stores the circle information for the colliders.
pub struct ColliderCircleInfo {
    x: i32,
    y: i32,
    radius: i32,
    distance_to_center_of_mass: i32,
    angle_to_center_of_mass: i32,
}
pub struct CollideeCircleInfo {
    x: i32,
    y: i32,
    radius: i32,
    blob_number: usize,
    circle_entity_type: CircleEntityType,
}

pub fn detect_collision(
    all_spatial_biosphere_information: &AllSpatialBiosphereInformation,
    blob_number: usize,
    game_settings: &GameSettings,
    deterministic_trig: &DeterministicTrig
) -> CollisionCheckResult {

    // These store the maximum movement before a collision (if any) occurs.
    let mut x_move = all_spatial_biosphere_information.blob_vec[blob_number].blob_x_velocity;
    let mut y_move = all_spatial_biosphere_information.blob_vec[blob_number].blob_y_velocity;
    let mut r_move = all_spatial_biosphere_information.blob_vec[blob_number].angular_velocity;

    // This stores the original moves so it can be references later.
    let original_x_move = x_move;
    let original_y_move = y_move;
    let original_r_move = r_move;

    // This is a list of all involved blobs so calculations can be done to get new velocities after a collision.
    let mut involved_blobs = vec![blob_number];

    // This keeps track if a mineral is involved. If one is, then the entire collision will result in zero velocities.
    let mut mineral_involved = false;

    // This keeps track of whether or not a collision occurred.
    let mut collision = false;

    // This makes a vec of all the circles of the collider blob.
    let mut collider_circles: Vec<ColliderCircleInfo> = make_collider_circle_list(all_spatial_biosphere_information, blob_number);


   
// This is just placeholder code.
return CollisionCheckResult {
    collision,
    x_move,
    y_move,
    r_move,
    involved_blobs,
    mineral_involved,
}

}