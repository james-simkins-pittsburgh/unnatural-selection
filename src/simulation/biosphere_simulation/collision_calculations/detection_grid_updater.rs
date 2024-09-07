use crate::{
    settings::GameSettings,
    simulation::{
        AllSpatialBiosphereInformation,
        CircleEntityType,
        CirclePositionRecord,
        OtherCirclePosition,
    },
};

use super::{ SMALL_GRID_SIZE, SMALL_GRID_CIRCLE_MAX_RADIUS, LARGE_GRID_SIZE };

pub fn update_for_movement(
    all_spatial_biosphere_information: &mut AllSpatialBiosphereInformation,
    previous_x: i32,
    previous_y: i32,
    previous_other_circles: &Vec<OtherCirclePosition>,
    organism_number: usize,
    game_settings: &GameSettings
) {
    // If its radius is smaller than the maximum for the small grid.
    if
        all_spatial_biosphere_information.organism_information_vec[organism_number].radius <=
        SMALL_GRID_CIRCLE_MAX_RADIUS
    {
        // If it changed spots on the grid.
        if
            (previous_x + game_settings.map_width / 2) / SMALL_GRID_SIZE !=
                (all_spatial_biosphere_information.organism_information_vec
                    [organism_number].x_location +
                    game_settings.map_width / 2) /
                    SMALL_GRID_SIZE ||
            (previous_y + game_settings.map_height / 2) / SMALL_GRID_SIZE !=
                (all_spatial_biosphere_information.organism_information_vec
                    [organism_number].y_location +
                    game_settings.map_height / 2) /
                    SMALL_GRID_SIZE
        {
            // Erase the old record for the main circle
            all_spatial_biosphere_information.collision_detection_grid_small[
                ((previous_x + game_settings.map_width / 2) / SMALL_GRID_SIZE) as usize
            ][((previous_y + game_settings.map_height / 2) / SMALL_GRID_SIZE) as usize].retain(
                |&index|
                    index.identity_number != organism_number ||
                    index.circle_entity_type != CircleEntityType::Organism ||
                    index.circle_number != 0
            );

            // Write the new record for the main circle.
            all_spatial_biosphere_information.collision_detection_grid_small[
                ((all_spatial_biosphere_information.organism_information_vec
                    [organism_number].x_location +
                    game_settings.map_width / 2) /
                    SMALL_GRID_SIZE) as usize
            ][
                ((all_spatial_biosphere_information.organism_information_vec
                    [organism_number].y_location +
                    game_settings.map_height / 2) /
                    SMALL_GRID_SIZE) as usize
            ].push(CirclePositionRecord {
                circle_entity_type: CircleEntityType::Organism,
                identity_number: organism_number,
                circle_number: 0,
            });
        }
    } else {
        // If it changed spots on the grid.
        if
            (previous_x + game_settings.map_width / 2) / LARGE_GRID_SIZE !=
                (all_spatial_biosphere_information.organism_information_vec
                    [organism_number].x_location +
                    game_settings.map_width / 2) /
                    LARGE_GRID_SIZE ||
            (previous_y + game_settings.map_height / 2) / LARGE_GRID_SIZE !=
                (all_spatial_biosphere_information.organism_information_vec
                    [organism_number].y_location +
                    game_settings.map_height / 2) /
                    LARGE_GRID_SIZE
        {
            // Erase the old record for the main circle
            all_spatial_biosphere_information.collision_detection_grid_large[
                ((previous_x + game_settings.map_width / 2) / LARGE_GRID_SIZE) as usize
            ][((previous_y + game_settings.map_height / 2) / LARGE_GRID_SIZE) as usize].retain(
                |&circle|
                    circle.identity_number != organism_number ||
                    circle.circle_entity_type != CircleEntityType::Organism ||
                    circle.circle_number != 0
            );

            // Write the new record for the main circle.
            all_spatial_biosphere_information.collision_detection_grid_large[
                ((all_spatial_biosphere_information.organism_information_vec
                    [organism_number].x_location +
                    game_settings.map_width / 2) /
                    LARGE_GRID_SIZE) as usize
            ][
                ((all_spatial_biosphere_information.organism_information_vec
                    [organism_number].y_location +
                    game_settings.map_height / 2) /
                    LARGE_GRID_SIZE) as usize
            ].push(CirclePositionRecord {
                circle_entity_type: CircleEntityType::Organism,
                identity_number: organism_number,
                circle_number: 0,
            });
        }
    }
    // If it is an oblong organism then the other circles need to be updated too.

    if all_spatial_biosphere_information.organism_information_vec[organism_number].oblong {
        // These lists allow what new records need to be written and which old records need to be erased.

        // If its radius is smaller or equal to the maximum for the small grid.
        if
            all_spatial_biosphere_information.organism_information_vec[organism_number].radius <=
            SMALL_GRID_CIRCLE_MAX_RADIUS
        {
            // For all of the other circles
            for index in 0..all_spatial_biosphere_information.organism_information_vec[
                organism_number
            ].other_circle_positions.len() {
                // If it changed spots on the grid.
                if
                    (previous_other_circles[index].x + game_settings.map_width / 2) /
                        SMALL_GRID_SIZE !=
                        (all_spatial_biosphere_information.organism_information_vec
                            [organism_number].other_circle_positions[index].x +
                            game_settings.map_width / 2) /
                            SMALL_GRID_SIZE ||
                    (previous_other_circles[index].y + game_settings.map_height / 2) /
                        SMALL_GRID_SIZE !=
                        (all_spatial_biosphere_information.organism_information_vec
                            [organism_number].other_circle_positions[index].y +
                            game_settings.map_height / 2) /
                            SMALL_GRID_SIZE
                {
                    // Erase the old record for the other circle
                    all_spatial_biosphere_information.collision_detection_grid_small[
                        ((previous_other_circles[index].x + game_settings.map_width / 2) /
                            SMALL_GRID_SIZE) as usize
                    ][
                        ((previous_other_circles[index].y + game_settings.map_height / 2) /
                            SMALL_GRID_SIZE) as usize
                    ].retain(
                        |&other_circle|
                            other_circle.identity_number != organism_number ||
                            other_circle.circle_entity_type != CircleEntityType::Organism ||
                            other_circle.circle_number != index + 1
                    );

                    // Write the new record for the other circle.
                    all_spatial_biosphere_information.collision_detection_grid_small[
                        ((all_spatial_biosphere_information.organism_information_vec
                            [organism_number].other_circle_positions[index].x +
                            game_settings.map_width / 2) /
                            SMALL_GRID_SIZE) as usize
                    ][
                        ((all_spatial_biosphere_information.organism_information_vec
                            [organism_number].other_circle_positions[index].y +
                            game_settings.map_height / 2) /
                            SMALL_GRID_SIZE) as usize
                    ].push(CirclePositionRecord {
                        circle_entity_type: CircleEntityType::Organism,
                        identity_number: organism_number,
                        circle_number: index + 1,
                    });
                }
            }

        // In the case of a large circle
        } else {
            // For all of the other circles
            for index in 0..all_spatial_biosphere_information.organism_information_vec[
                organism_number
            ].other_circle_positions.len() {
                // If it changed spots on the grid.
                if
                    (previous_other_circles[index].x + game_settings.map_width / 2) /
                        LARGE_GRID_SIZE !=
                        (all_spatial_biosphere_information.organism_information_vec
                            [organism_number].other_circle_positions[index].x +
                            game_settings.map_width / 2) /
                            LARGE_GRID_SIZE ||
                    (previous_other_circles[index].y + game_settings.map_height / 2) /
                        LARGE_GRID_SIZE !=
                        (all_spatial_biosphere_information.organism_information_vec
                            [organism_number].other_circle_positions[index].y +
                            game_settings.map_height / 2) /
                            LARGE_GRID_SIZE
                {
                    // Erase the old record for the other circle
                    all_spatial_biosphere_information.collision_detection_grid_large[
                        ((previous_other_circles[index].x + game_settings.map_width / 2) /
                            LARGE_GRID_SIZE) as usize
                    ][
                        ((previous_other_circles[index].y + game_settings.map_height / 2) /
                            LARGE_GRID_SIZE) as usize
                    ].retain(
                        |&other_circle|
                            other_circle.identity_number != organism_number ||
                            other_circle.circle_entity_type != CircleEntityType::Organism ||
                            other_circle.circle_number != index + 1
                    );

                    // Write the new record for the other circle.
                    all_spatial_biosphere_information.collision_detection_grid_large[
                        ((all_spatial_biosphere_information.organism_information_vec
                            [organism_number].other_circle_positions[index].x +
                            game_settings.map_width / 2) /
                            LARGE_GRID_SIZE) as usize
                    ][
                        ((all_spatial_biosphere_information.organism_information_vec
                            [organism_number].other_circle_positions[index].y +
                            game_settings.map_height / 2) /
                            LARGE_GRID_SIZE) as usize
                    ].push(CirclePositionRecord {
                        circle_entity_type: CircleEntityType::Organism,
                        identity_number: organism_number,
                        circle_number: index + 1,
                    });
                }
            }
        }
    }
}
