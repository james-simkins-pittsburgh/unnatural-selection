use crate::{
    settings::GameSettings,
    simulation::{
        AllBiosphereInformation,
        CircleEntityType,
        CirclePositionRecord,
        OtherCirclePosition,
    },
};

use super::GRID_SIZE;

pub fn update_for_movement(
    all_biosphere_information: &mut AllBiosphereInformation,
    previous_x: i32,
    previous_y: i32,
    previous_other_circles: &Vec<OtherCirclePosition>,
    organism_number: usize,
    game_settings: &GameSettings
) {
    // Erase the old record for the main circle
    all_biosphere_information.collision_detection_grid[
        ((previous_x + game_settings.map_length / 2) / GRID_SIZE) as usize
    ][((previous_y + game_settings.map_height / 2) / GRID_SIZE) as usize].retain(
        |&index|
            index.identity_number != organism_number ||
            index.circle_entity_type != CircleEntityType::Organism
    );

    // Write the new record for the main circle.
    all_biosphere_information.collision_detection_grid[
        ((all_biosphere_information.organism_information_vec[organism_number].x_location +
            game_settings.map_length / 2) /
            GRID_SIZE) as usize
    ][
        ((all_biosphere_information.organism_information_vec[organism_number].y_location +
            game_settings.map_height / 2) /
            GRID_SIZE) as usize
    ].push(CirclePositionRecord {
        center_x: all_biosphere_information.organism_information_vec[organism_number].x_location,
        center_y: all_biosphere_information.organism_information_vec[organism_number].y_location,
        radius: all_biosphere_information.organism_information_vec[organism_number].radius,
        background: all_biosphere_information.organism_information_vec[organism_number].background,
        circle_entity_type: CircleEntityType::Organism,
        identity_number: organism_number,
        blob_number: all_biosphere_information.organism_information_vec
            [organism_number].blob_number,
    });

    // For oblong organisms, then write record for the other circle positions.
    if all_biosphere_information.organism_information_vec[organism_number].oblong {
        // For every previous circle
        for previous_other_circle in previous_other_circles.iter() {
            // check to see if it is in a different grid then the main circle
            if
                (previous_other_circle.x + game_settings.map_length / 2) / GRID_SIZE !=
                    (previous_x + game_settings.map_length / 2) / GRID_SIZE ||
                (previous_other_circle.y + game_settings.map_height / 2) / GRID_SIZE !=
                    (previous_y + game_settings.map_height / 2) / GRID_SIZE
            {
                // If it is, then remove the record.
                all_biosphere_information.collision_detection_grid[
                    ((previous_other_circle.x + game_settings.map_length / 2) / GRID_SIZE) as usize
                ][
                    ((previous_other_circle.y + game_settings.map_height / 2) / GRID_SIZE) as usize
                ].retain(
                    |&index|
                        index.identity_number != organism_number ||
                        index.circle_entity_type != CircleEntityType::Organism
                );
            }
        }

        // The write for every other circle
        for circle in all_biosphere_information.organism_information_vec[
            organism_number
        ].other_circle_positions.iter() {
            all_biosphere_information.collision_detection_grid[
                ((circle.x + game_settings.map_length / 2) / GRID_SIZE) as usize
            ][((circle.y + game_settings.map_height / 2) / GRID_SIZE) as usize].push(
                CirclePositionRecord {
                    center_x: circle.x,
                    center_y: circle.y,
                    radius: all_biosphere_information.organism_information_vec
                        [organism_number].radius,
                    background: all_biosphere_information.organism_information_vec
                        [organism_number].background,
                    circle_entity_type: CircleEntityType::Organism,
                    identity_number: organism_number,
                    blob_number: all_biosphere_information.organism_information_vec
                        [organism_number].blob_number,
                }
            );
        }
    }
}
