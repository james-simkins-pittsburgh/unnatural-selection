use crate::{
    simulation::CircleEntityType,
    utility_functions::{
        deterministic_trigonometry::DeterministicTrig,
        two_circles_intersection_solver::solve_two_circle_intersection,
    },
};

use super::CollideeCircleInfo;

pub fn check_two_circles_angular(
    r_move: &mut i32,
    original_r_move: i32,
    involved_blobs: &mut Vec<usize>,
    mineral_involved: &mut bool,
    blob_number: usize,
    collidee_circle: &CollideeCircleInfo,
    collider_circle_radius: i32,
    collider_distance_center_of_mass: i32,
    // This is the center of mass after translation.
    center_of_mass_x_after_xymove: i32,
    center_of_mass_y_after_xymove: i32,
    // These are the x and y after translation only.
    collider_x_after_xymove: i32,
    collider_y_after_xymove: i32,
    // These are the x and y if it fully translates and rotates
    full_collider_x: i32,
    full_collider_y: i32,
    deterministic_trig: &DeterministicTrig
) {
    // If the circle is not at the center of the blob.
    if collider_distance_center_of_mass != 0 {
        // Check to see if a collision happens.
        if
            (collider_circle_radius + collidee_circle.radius) *
                (collider_circle_radius + collidee_circle.radius) >=
            (collidee_circle.x - full_collider_x) * (collidee_circle.x - full_collider_x) +
                (collidee_circle.y - full_collider_y) * (collidee_circle.y - full_collider_y)
        {
            // Save the combined radius to avoid calculating it over and over again.
            let combined_radius_squared =
                (collider_circle_radius + collidee_circle.radius) *
                (collider_circle_radius + collidee_circle.radius);

            // Check to see if the collision happens before the full rotation completed
            if
                combined_radius_squared >
                (collidee_circle.x - full_collider_x) * (collidee_circle.x - full_collider_x) +
                    (collidee_circle.y - full_collider_y) * (collidee_circle.y - full_collider_y)
            {
                // If it did, then reset the collision list because collisions with the current r_move aren't happening.
                if collidee_circle.circle_entity_type == CircleEntityType::Organism {
                    *involved_blobs = vec![blob_number, collidee_circle.blob_number];
                    *mineral_involved = false;
                } else {
                    *involved_blobs = vec![blob_number];
                    *mineral_involved = true;
                }

                // This calculates the two points the collider could collide with the collidee.
                let points_of_collisions = solve_two_circle_intersection(
                    center_of_mass_x_after_xymove,
                    center_of_mass_y_after_xymove,
                    collider_distance_center_of_mass,
                    collidee_circle.x,
                    collidee_circle.y,
                    collidee_circle.radius + collider_circle_radius
                );

                // This determines which one is closer, which is the one that actually happens.
                let initial_angle = if
                    ((collider_x_after_xymove - center_of_mass_x_after_xymove) * 1000) /
                        collider_distance_center_of_mass > 1000
                {
                    0
                } else if
                    ((collider_x_after_xymove - center_of_mass_x_after_xymove) * 1000) /
                        collider_distance_center_of_mass < -1000
                {
                    3142
                } else {
                    deterministic_trig.d_trig.arccosine((
                        ((collider_x_after_xymove - center_of_mass_x_after_xymove) * 1000) /
                            collider_distance_center_of_mass,
                        1000,
                    )).0 * (collider_y_after_xymove - center_of_mass_y_after_xymove).signum()
                };

                let final_angle_1 = if
                    ((points_of_collisions.0.0 - center_of_mass_x_after_xymove) * 1000) /
                        collider_distance_center_of_mass > 1000
                {
                    0
                } else if
                    ((points_of_collisions.0.0 - center_of_mass_x_after_xymove) * 1000) /
                        collider_distance_center_of_mass < -1000
                {
                    3142
                } else {
                    deterministic_trig.d_trig.arccosine((
                        ((points_of_collisions.0.0 - center_of_mass_x_after_xymove) * 1000) /
                            collider_distance_center_of_mass,
                        1000,
                    )).0 * (points_of_collisions.0.1 - center_of_mass_y_after_xymove).signum()
                };

                let final_angle_2 = if
                    ((points_of_collisions.1.0 - center_of_mass_x_after_xymove) * 1000) /
                        collider_distance_center_of_mass > 1000
                {
                    0
                } else if
                    ((points_of_collisions.1.0 - center_of_mass_x_after_xymove) * 1000) /
                        collider_distance_center_of_mass < -1000
                {
                    3142
                } else {
                    deterministic_trig.d_trig.arccosine((
                        ((points_of_collisions.1.0 - center_of_mass_x_after_xymove) * 1000) /
                            collider_distance_center_of_mass,
                        1000,
                    )).0 * (points_of_collisions.1.1 - center_of_mass_y_after_xymove).signum()
                };

                let angle_1_change = if
                    initial_angle.signum() == final_angle_1.signum() ||
                    initial_angle.abs() < 1571
                {
                    final_angle_1 - initial_angle
                } else {
                    final_angle_1 + 3142 * initial_angle.signum() - initial_angle
                };

                let angle_2_change = if
                    initial_angle.signum() == final_angle_2.signum() ||
                    initial_angle.abs() < 1571
                {
                    final_angle_2 - initial_angle
                } else {
                    final_angle_2 + 3142 * initial_angle.signum() - initial_angle
                };

                // This sets the r_move to whichever is the smaller change in angle.

                if angle_1_change.abs() < angle_2_change.abs() {
                    *r_move = angle_1_change;
                } else {
                    *r_move = angle_2_change;
                }

                // Addresses the possibility a rounding error made it so that there is now overlap.
                let mut rounding_error_collider_x =
                    center_of_mass_x_after_xymove +
                    (collider_distance_center_of_mass *
                        deterministic_trig.d_trig.cosine((*r_move, 1000)).0) /
                        1000;
                let mut rounding_error_collider_y =
                    center_of_mass_y_after_xymove +
                    (collider_distance_center_of_mass *
                        deterministic_trig.d_trig.sine((*r_move, 1000)).0) /
                        1000;

                while
                    (collider_circle_radius + collidee_circle.radius) *
                        (collider_circle_radius + collidee_circle.radius) >
                        (collidee_circle.x - rounding_error_collider_x) *
                            (collidee_circle.x - rounding_error_collider_x) +
                            (collidee_circle.y - rounding_error_collider_y) *
                                (collidee_circle.y - rounding_error_collider_y) &&
                    r_move.abs() > 0
                {
                    *r_move = *r_move - r_move.signum();

                    rounding_error_collider_x =
                        center_of_mass_x_after_xymove +
                        (collider_distance_center_of_mass *
                            deterministic_trig.d_trig.cosine((*r_move, 1000)).0) /
                            1000;
                    rounding_error_collider_y =
                        center_of_mass_y_after_xymove +
                        (collider_distance_center_of_mass *
                            deterministic_trig.d_trig.sine((*r_move, 1000)).0) /
                            1000;
                }

                // Test code

                println!("center_of_mass_x_after_xymove: {}", center_of_mass_x_after_xymove);
                println!("center_of_mass_y_after_xymove: {}", center_of_mass_y_after_xymove);
                println!("collider_distance_center_of_mass: {}", collider_distance_center_of_mass);
                println!("collidee_circle.x: {}", collidee_circle.x);
                println!("collidee_circle.y: {}", collidee_circle.y);
                println!(
                    "collidee_circle.radius + collider_circle_radius: {}",
                    collidee_circle.radius + collider_circle_radius
                );

                print!("points_of_collisions x1: {}, y1: {} x2: {} y2: {}", points_of_collisions.0.0, points_of_collisions.0.1, points_of_collisions.1.0, points_of_collisions.1.1);

                // End test code

                // This covers the case in which the collision occurs exactly at the r_move.
            } else {
                // If is has already collided with something else.
                if *r_move != original_r_move {
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
}

#[test]
fn test_angular_collision_detector() {
    let collidee_circle = CollideeCircleInfo {
        x: -6871,
        y: -6482,
        radius: 2000,
        blob_number: 2,
        circle_entity_type: CircleEntityType::Organism,
    };

    let deterministic_trig = &DeterministicTrig::default();

    let mut r_move = -150;
    let original_r_move = -150;
    let mut involved_blobs = vec![];
    let mut mineral_involved = false;
    let blob_number = 1;
    let collider_circle_radius = 2000;
    let collider_distance_center_of_mass = 6000;
    let center_of_mass_x_after_xymove = -2000;
    let center_of_mass_y_after_xymove = -1000;
    let collider_x_after_xymove = -2500;
    let collider_y_after_xymove = -6982;
    let full_collider_x = -3387;
    let full_collider_y = -6837;

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
        deterministic_trig
    );

    assert_eq!(r_move, -66);
}
