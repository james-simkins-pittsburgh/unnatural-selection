use crate::{ settings::GameSettings, simulation::{ AllBiosphereInformation, CircleEntityType, CirclePositionRecord } };

pub fn update_for_movement(
    all_biosphere_information: &mut AllBiosphereInformation,
    previous_x: i32,
    previous_y: i32,
    organism_number: usize,
    game_settings: &GameSettings
) {
    // If the center of the organism is now in a different grid
    if
        (all_biosphere_information.organism_information_vec[organism_number].x_location +
            game_settings.map_length / 2) /
            10000 != (previous_x + game_settings.map_length / 2) / 10000 ||
        (all_biosphere_information.organism_information_vec[organism_number].y_location +
            game_settings.map_height / 2) /
            10000 != (previous_y + game_settings.map_height / 2) / 10000
    {
        // Erase the old record(s)
        let previous_grid_index_1 = ((previous_x + game_settings.map_length / 2) / 10000) as usize;
        let previous_grid_index_2 = ((previous_y + game_settings.map_height / 2) / 10000) as usize;

        all_biosphere_information.collision_detection_grid[previous_grid_index_1][
            previous_grid_index_2
        ].retain(
            |&index|
                (index.identity_number != organism_number ||
                index.circle_entity_type != CircleEntityType::Organism)
        );

        // Write the new record(s)
        // TO DO: CODE NEEDS TO BE UPDATED FOR MULTI-CIRCLE ORGANISM!!!!
        all_biosphere_information.collision_detection_grid [(all_biosphere_information.organism_information_vec[organism_number].x_location +
            game_settings.map_length / 2) as usize] [(all_biosphere_information.organism_information_vec[organism_number].y_location +
                game_settings.map_height / 2) as usize].push(CirclePositionRecord{

                    center_x: all_biosphere_information.organism_information_vec[organism_number].x_location,
                    center_y: all_biosphere_information.organism_information_vec[organism_number].y_location,
                    radius: all_biosphere_information.organism_information_vec[organism_number].radius,
                    background: false,
                    circle_entity_type: CircleEntityType::Organism,
                    identity_number: organism_number,
                    
                })


    }
}
