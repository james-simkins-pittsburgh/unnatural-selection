use crate::{
    settings::GameSettings,
    simulation::{
        AllSpatialBiosphereInformation,
        CircleEntityType,
        CirclePositionRecord,
        OtherCirclePosition,
    },
};

use super::{SMALL_GRID_SIZE, SMALL_GRID_CIRCLE_MAX_RADIUS};

pub fn update_for_movement(
    all_spatial_biosphere_information: &mut AllSpatialBiosphereInformation,
    previous_x: i32,
    previous_y: i32,
    previous_other_circles: &Vec<OtherCirclePosition>,
    organism_number: usize,
    game_settings: &GameSettings
) {
    if
        all_spatial_biosphere_information.organism_information_vec[organism_number].radius <=
        SMALL_GRID_CIRCLE_MAX_RADIUS
    {
        // Erase the old record for the main circle
        all_spatial_biosphere_information.collision_detection_grid_small[
            ((previous_x + game_settings.map_length / 2) / SMALL_GRID_SIZE) as usize
        ][((previous_y + game_settings.map_height / 2) / SMALL_GRID_SIZE) as usize].retain(
            |&index|
                index.identity_number != organism_number ||
                index.circle_entity_type != CircleEntityType::Organism
        );

        // Write the new record for the main circle.
        all_spatial_biosphere_information.collision_detection_grid_small[
            ((all_spatial_biosphere_information.organism_information_vec
                [organism_number].x_location +
                game_settings.map_length / 2) /
                SMALL_GRID_SIZE) as usize
        ][
            ((all_spatial_biosphere_information.organism_information_vec
                [organism_number].y_location +
                game_settings.map_height / 2) /
                SMALL_GRID_SIZE) as usize
        ].push(CirclePositionRecord {
            x: all_spatial_biosphere_information.organism_information_vec
                [organism_number].x_location,
            y: all_spatial_biosphere_information.organism_information_vec
                [organism_number].y_location,
            radius: all_spatial_biosphere_information.organism_information_vec
                [organism_number].radius,
            background: all_spatial_biosphere_information.organism_information_vec
                [organism_number].background,
            circle_entity_type: CircleEntityType::Organism,
            identity_number: organism_number,
            blob_number: all_spatial_biosphere_information.organism_information_vec
                [organism_number].blob_number,
        });

        // For oblong organisms, then write record for the other circle positions.
        if all_spatial_biosphere_information.organism_information_vec[organism_number].oblong {
            // For every previous circle
            for previous_other_circle in previous_other_circles.iter() {
                // check to see if it is in a different grid then the main circle
                if
                    (previous_other_circle.x + game_settings.map_length / 2) / SMALL_GRID_SIZE !=
                        (previous_x + game_settings.map_length / 2) / SMALL_GRID_SIZE ||
                    (previous_other_circle.y + game_settings.map_height / 2) / SMALL_GRID_SIZE !=
                        (previous_y + game_settings.map_height / 2) / SMALL_GRID_SIZE
                {
                    // If it is, then remove the record.
                    all_spatial_biosphere_information.collision_detection_grid_small[
                        ((previous_other_circle.x + game_settings.map_length / 2) /
                            SMALL_GRID_SIZE) as usize
                    ][
                        ((previous_other_circle.y + game_settings.map_height / 2) /
                            SMALL_GRID_SIZE) as usize
                    ].retain(
                        |&index|
                            index.identity_number != organism_number ||
                            index.circle_entity_type != CircleEntityType::Organism
                    );
                }
            }

            // The write for every other circle
            for circle in all_spatial_biosphere_information.organism_information_vec[
                organism_number
            ].other_circle_positions.iter() {
                all_spatial_biosphere_information.collision_detection_grid_small[
                    ((circle.x + game_settings.map_length / 2) / SMALL_GRID_SIZE) as usize
                ][((circle.y + game_settings.map_height / 2) / SMALL_GRID_SIZE) as usize].push(
                    CirclePositionRecord {
                        x: circle.x,
                        y: circle.y,
                        radius: all_spatial_biosphere_information.organism_information_vec
                            [organism_number].radius,
                        background: all_spatial_biosphere_information.organism_information_vec
                            [organism_number].background,
                        circle_entity_type: CircleEntityType::Organism,
                        identity_number: organism_number,
                        blob_number: all_spatial_biosphere_information.organism_information_vec
                            [organism_number].blob_number,
                    }
                );
            }
        }
    }
}
