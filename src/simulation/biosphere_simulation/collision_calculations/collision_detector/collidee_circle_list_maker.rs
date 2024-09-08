use crate::{
    settings::GameSettings,
    simulation::{
        biosphere_simulation::collision_calculations::{
            SMALL_GRID_CIRCLE_MAX_RADIUS,
            SMALL_GRID_SIZE,
        },
        AllSpatialBiosphereInformation,
        CircleEntityType,
    },
    utility_functions::deterministic_trigonometry::DeterministicTrig,
};
use super::{ CollideeCircleInfo, ColliderCircleInfo };

pub fn make_collidee_circle_list(
    collider_circle: &ColliderCircleInfo,
    blob_number: usize,
    game_settings: &GameSettings,
    deterministic_trig: &DeterministicTrig,
    x_move: i32,
    y_move: i32,
    r_move: i32,
    all_spatial_biosphere_information: &AllSpatialBiosphereInformation
) -> Vec<CollideeCircleInfo> {
    let mut collidee_circles: Vec<CollideeCircleInfo> = Vec::new();

    let collider_radius = collider_circle.radius;

    // If it is a small circle.
    if collider_radius <= SMALL_GRID_CIRCLE_MAX_RADIUS {
        let x_index_main = ((collider_circle.x + game_settings.map_width / 2) /
            SMALL_GRID_SIZE) as usize;
        let y_index_main = ((collider_circle.y + game_settings.map_height / 2) /
            SMALL_GRID_SIZE) as usize;

        // Add the circles in the small grid square the collider is in.
        add_circles_in_small_grid(
            x_index_main,
            y_index_main,
            blob_number,
            &all_spatial_biosphere_information,
            &mut collidee_circles
        );

        // If the grid square above is within range.
        if
            y_index_main !=
            (
                ((collider_circle.y +
                    (if y_move > 0 { y_move } else { 0 }) +
                    collider_radius +
                    SMALL_GRID_CIRCLE_MAX_RADIUS +
                    game_settings.map_height / 2) /
                    SMALL_GRID_SIZE) as usize
            )
        {
            // Add the circles in the grid square above.
            add_circles_in_small_grid(
                x_index_main,
                y_index_main + 1,
                blob_number,
                &all_spatial_biosphere_information,
                &mut collidee_circles
            );
            // Check to see if the grid square above and to the right is in range.
            if
                x_index_main !=
                (
                    ((collider_circle.x +
                        (if x_move > 0 { x_move } else { 0 }) +
                        collider_radius +
                        SMALL_GRID_CIRCLE_MAX_RADIUS +
                        game_settings.map_width / 2) /
                        SMALL_GRID_SIZE) as usize
                )
            {
                // Add the circles in the grid square above and to the right.
                add_circles_in_small_grid(
                    x_index_main + 1,
                    y_index_main + 1,
                    blob_number,
                    &all_spatial_biosphere_information,
                    &mut collidee_circles
                );
            }
            // Check to see if the grid square above and to the left is in range.
            if
                x_index_main !=
                (
                    ((collider_circle.x +
                        (if x_move < 0 { x_move } else { 0 }) -
                        collider_radius -
                        SMALL_GRID_CIRCLE_MAX_RADIUS +
                        game_settings.map_width / 2) /
                        SMALL_GRID_SIZE) as usize
                )
            {
                // Add the circles in the grid square above and to the left.
                add_circles_in_small_grid(
                    x_index_main - 1,
                    y_index_main + 1,
                    blob_number,
                    &all_spatial_biosphere_information,
                    &mut collidee_circles
                );
            }

        }

        // If the grid square below is within range.
        if
            y_index_main !=
            (
                ((collider_circle.y +
                    (if y_move < 0 { y_move } else { 0 }) +
                    collider_radius -
                    SMALL_GRID_CIRCLE_MAX_RADIUS -
                    game_settings.map_height / 2) /
                    SMALL_GRID_SIZE) as usize
            )
        {
            // Add the circles in the grid square below.
            add_circles_in_small_grid(
                x_index_main,
                y_index_main - 1,
                blob_number,
                &all_spatial_biosphere_information,
                &mut collidee_circles
            );

            // Check to see if the grid square below and to the right is in range.
            if
                x_index_main !=
                (
                    ((collider_circle.x +
                        (if x_move > 0 { x_move } else { 0 }) +
                        collider_radius +
                        SMALL_GRID_CIRCLE_MAX_RADIUS +
                        game_settings.map_width / 2) /
                        SMALL_GRID_SIZE) as usize
                )
            {
                // Add the circles in the grid square below and to the right.
                add_circles_in_small_grid(
                    x_index_main + 1,
                    y_index_main - 1,
                    blob_number,
                    &all_spatial_biosphere_information,
                    &mut collidee_circles
                );
            }
            // Check to see if the grid square below and to the left is in range.
            if
                x_index_main !=
                (
                    ((collider_circle.x +
                        (if x_move < 0 { x_move } else { 0 }) -
                        collider_radius -
                        SMALL_GRID_CIRCLE_MAX_RADIUS +
                        game_settings.map_width / 2) /
                        SMALL_GRID_SIZE) as usize
                )
            {
                // Add the circles in the grid square below and to the left.
                add_circles_in_small_grid(
                    x_index_main - 1,
                    y_index_main - 1,
                    blob_number,
                    &all_spatial_biosphere_information,
                    &mut collidee_circles
                );
            }

        }

        // This covers the more complicated case of big circles
    } else {
    }

    return collidee_circles;
}

fn add_circles_in_small_grid(
    x_index: usize,
    y_index: usize,
    blob_number: usize,
    all_spatial_biosphere_information: &AllSpatialBiosphereInformation,
    collidee_circles: &mut Vec<CollideeCircleInfo>
) {
    // For every circle that is in the square
    for circle in all_spatial_biosphere_information.collision_detection_grid_small[x_index][
        y_index
    ].iter() {
        // If the circle is either a mineral or it is from a different blob.
        if
            circle.circle_entity_type == CircleEntityType::Mineral ||
            all_spatial_biosphere_information.organism_information_vec
                [circle.identity_number].blob_number != blob_number
        {
            // Then add it to the collidee list.
            if circle.circle_entity_type == CircleEntityType::Mineral {
                collidee_circles.push(CollideeCircleInfo {
                    x: all_spatial_biosphere_information.mineral_information_vec
                        [circle.identity_number].x_location,
                    y: all_spatial_biosphere_information.mineral_information_vec
                        [circle.identity_number].y_location,
                    radius: all_spatial_biosphere_information.mineral_information_vec
                        [circle.identity_number].radius,
                    blob_number: 0,
                    circle_entity_type: CircleEntityType::Mineral,
                });
            } else {
                collidee_circles.push(CollideeCircleInfo {
                    x: if circle.circle_number == 0 {
                        all_spatial_biosphere_information.organism_information_vec
                            [circle.identity_number].x_location
                    } else {
                        all_spatial_biosphere_information.organism_information_vec
                            [circle.identity_number].other_circle_positions
                            [circle.circle_number - 1].x
                    },
                    y: if circle.circle_number == 0 {
                        all_spatial_biosphere_information.organism_information_vec
                            [circle.identity_number].y_location
                    } else {
                        all_spatial_biosphere_information.organism_information_vec
                            [circle.identity_number].other_circle_positions
                            [circle.circle_number - 1].y
                    },
                    radius: if circle.circle_number == 0 {
                        all_spatial_biosphere_information.organism_information_vec
                            [circle.identity_number].radius
                    } else {
                        all_spatial_biosphere_information.organism_information_vec
                            [circle.identity_number].other_circle_positions
                            [circle.circle_number - 1].radius
                    },
                    blob_number: all_spatial_biosphere_information.organism_information_vec
                        [circle.identity_number].blob_number,
                    circle_entity_type: CircleEntityType::Organism,
                });
            }
        }
    }
}
