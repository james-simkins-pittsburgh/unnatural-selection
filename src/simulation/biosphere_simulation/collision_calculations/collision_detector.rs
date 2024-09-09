use collidee_circle_list_maker::make_collidee_circle_list;
use collider_circle_list_maker::make_collider_circle_list;
use translational_collision_detector::check_two_circles_translational;

use crate::{
    settings::GameSettings,
    simulation::{
        biosphere_simulation::blob_mover::CollisionCheckResult,
        AllSpatialBiosphereInformation,
        CircleEntityType,
    },
    utility_functions::deterministic_trigonometry::DeterministicTrig,
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
    let collider_circles: Vec<ColliderCircleInfo> = make_collider_circle_list(
        all_spatial_biosphere_information,
        blob_number
    );

    // This makes a vec of vecs that will hold the potential collidee circles for each collider circle.
    let mut potential_collidee_circles: Vec<Vec<CollideeCircleInfo>> = Vec::new();

    // This populates the vec of vecs with the potential collider circles for each collidee circle.
    for index in 0..collider_circles.len() {
        potential_collidee_circles[index] = make_collidee_circle_list(
            &collider_circles[index],
            blob_number,
            &game_settings,
            &deterministic_trig,
            original_x_move,
            original_y_move,
            original_r_move,
            all_spatial_biosphere_information
        );
    }

    // This finds any collisions from translational movements only.
    for index in 0..collider_circles.len() {
        for collidee_circle in potential_collidee_circles[index].iter() {
            check_two_circles_translational(
                &mut x_move,
                &mut y_move,
                original_x_move,
                original_y_move,
                &mut involved_blobs,
                &mut mineral_involved,
                blob_number,
                &collider_circles[index],
                &collidee_circle
            );
        }
    }

    // If no collision has happened yet, then also check angular movement.
    if involved_blobs.len() <= 1 && !mineral_involved {
        for index in 0..collider_circles.len() {
            
            // LEFT OFF HERE!!!!!!!!!
            let collider_circle_radius = 0;
            let collider_distance_center_of_mass = 0;
            let center_of_mass_x_after_xymove = 0;
            let center_of_mass_y_after_xymove = 0;
            let collider_x_after_xymove = 0;
            let collider_y_after_xymove = 0;
            let full_collider_x = 0;
            let full_collider_y = 0;

            for collidee_circle in potential_collidee_circles[index].iter() {
                check_two_circles_angular(
                    &mut r_move,
                    original_r_move,
                    &mut involved_blobs,
                    &mut mineral_involved,
                    blob_number,
                    &collidee_circle,
                    collider_circle_radius,
                    collider_distance_center_of_mass,
                    center_of_mass_x_after_xymove,
                    center_of_mass_y_after_xymove,
                    collider_x_after_xymove,
                    collider_y_after_xymove,
                    full_collider_x,
                    full_collider_y,
                    &deterministic_trig
                );
            }
        }
    }

    // This is just placeholder code.
    return CollisionCheckResult {
        collision,
        x_move,
        y_move,
        r_move,
        involved_blobs,
        mineral_involved,
    };
}
