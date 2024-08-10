use crate::{
    settings::GameSettings,
    simulation::{ AllBiosphereInformation, CircleEntityType, CirclePositionRecord, OtherCirclePosition },
};

pub fn update_for_movement(
    all_biosphere_information: &mut AllBiosphereInformation,
    previous_x: i32,
    previous_y: i32,
    previous_other_circle: &Vec<OtherCirclePosition>,
    organism_number: usize,
    game_settings: &GameSettings
) {
    // Checks to see if any of the not main circles of the organism are now in a different grid for oblong organisms.
    let mut other_circles_different_grid = false;

    if all_biosphere_information.organism_information_vec[organism_number].oblong {
        for index in 0..all_biosphere_information.organism_information_vec[
            organism_number
        ].other_circle_positions.len() {
            if
                (all_biosphere_information.organism_information_vec
                    [organism_number].other_circle_positions[index].x +
                    game_settings.map_length / 2) /
                    10000 !=
                    (previous_other_circle[index].x + game_settings.map_length / 2) / 10000 ||
                (all_biosphere_information.organism_information_vec
                    [organism_number].other_circle_positions[index].y +
                    game_settings.map_height / 2) /
                    10000 != (previous_other_circle[index].y + game_settings.map_height / 2) / 10000
            {
                other_circles_different_grid = true;
            }
        }
    }

    // If the main circle or any of the other circles are in a different grid
    if
        (all_biosphere_information.organism_information_vec[organism_number].x_location +
            game_settings.map_length / 2) /
            10000 != (previous_x + game_settings.map_length / 2) / 10000 ||
        (all_biosphere_information.organism_information_vec[organism_number].y_location +
            game_settings.map_height / 2) /
            10000 != (previous_y + game_settings.map_height / 2) / 10000 ||
        other_circles_different_grid
    {
        // Then erase the old record(s)
        let previous_grid_index_1 = ((previous_x + game_settings.map_length / 2) / 10000) as usize;
        let previous_grid_index_2 = ((previous_y + game_settings.map_height / 2) / 10000) as usize;

        all_biosphere_information.collision_detection_grid[previous_grid_index_1][
            previous_grid_index_2
        ].retain(
            |&index|
                index.identity_number != organism_number ||
                index.circle_entity_type != CircleEntityType::Organism
        );
        // If it is oblong, check to see if other circles are in different grid(s) and erase those record(s) too.
        if all_biosphere_information.organism_information_vec[organism_number].oblong {
            for circle in previous_other_circle.iter() {
                if
                    (((circle.x + game_settings.map_length / 2) / 10000) as usize) !=
                        previous_grid_index_1 ||
                    (((circle.y + game_settings.map_height / 2) / 10000) as usize) !=
                        previous_grid_index_2
                {
                    all_biosphere_information.collision_detection_grid[
                        ((circle.x + game_settings.map_length / 2) / 10000) as usize
                    ][((circle.y + game_settings.map_height / 2) / 10000) as usize].retain(
                        |&index|
                            index.identity_number != organism_number ||
                            index.circle_entity_type != CircleEntityType::Organism
                    );
                }
            }
        }

        // Write the new record for the main circle.
        all_biosphere_information.collision_detection_grid[
            ((all_biosphere_information.organism_information_vec[organism_number].x_location +
                game_settings.map_length / 2) /10000) as usize
        ][
            ((all_biosphere_information.organism_information_vec[organism_number].y_location +
                game_settings.map_height / 2) /10000) as usize
        ].push(CirclePositionRecord {
            center_x: all_biosphere_information.organism_information_vec
                [organism_number].x_location,
            center_y: all_biosphere_information.organism_information_vec
                [organism_number].y_location,
            radius: all_biosphere_information.organism_information_vec[organism_number].radius,
            background: all_biosphere_information.organism_information_vec
                [organism_number].background,
            circle_entity_type: CircleEntityType::Organism,
            identity_number: organism_number,
        });

        // For oblong organisms, then write record for the other circle positions.
        if all_biosphere_information.organism_information_vec[organism_number].oblong {
            for circle in all_biosphere_information.organism_information_vec[
                organism_number
            ].other_circle_positions.iter() {
                all_biosphere_information.collision_detection_grid[
                    (circle.x + game_settings.map_length / 2) as usize
                ][(circle.y + game_settings.map_height / 2) as usize].push(CirclePositionRecord {
                    center_x: circle.x,
                    center_y: circle.y,
                    radius: all_biosphere_information.organism_information_vec
                        [organism_number].radius,
                    background: all_biosphere_information.organism_information_vec
                        [organism_number].background,
                    circle_entity_type: CircleEntityType::Organism,
                    identity_number: organism_number,
                });
            }
        }
    }
}
