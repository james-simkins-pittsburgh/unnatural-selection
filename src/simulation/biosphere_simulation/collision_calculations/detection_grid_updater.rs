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
    // If it is not an oblong organism.
    if !all_spatial_biosphere_information.organism_information_vec[organism_number].oblong {
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
                        [organism_number].x_location +
                        game_settings.map_height / 2) /
                        SMALL_GRID_SIZE
            {
                // Erase the old record for the main circle
                all_spatial_biosphere_information.collision_detection_grid_small[
                    ((previous_x + game_settings.map_width / 2) / SMALL_GRID_SIZE) as usize
                ][((previous_y + game_settings.map_height / 2) / SMALL_GRID_SIZE) as usize].retain(
                    |&index|
                        index.identity_number != organism_number ||
                        index.circle_entity_type != CircleEntityType::Organism
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
                    main_circle: true,
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
                        [organism_number].x_location +
                        game_settings.map_height / 2) /
                        LARGE_GRID_SIZE
            {
                // Erase the old record for the main circle
                all_spatial_biosphere_information.collision_detection_grid_large[
                    ((previous_x + game_settings.map_width / 2) / LARGE_GRID_SIZE) as usize
                ][((previous_y + game_settings.map_height / 2) / LARGE_GRID_SIZE) as usize].retain(
                    |&index|
                        index.identity_number != organism_number ||
                        index.circle_entity_type != CircleEntityType::Organism
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
                    main_circle: true,
                });
            }
        }
        // If it is an oblong organism.
    } else {
        // These lists allow what new records need to be written and which old records need to be erased.

        let mut list_of_previous_grid_spots = Vec::new();

        let mut list_of_new_grid_spots = Vec::new();

        // If its radius is smaller or equal to the maximum for the small grid.
        if
            all_spatial_biosphere_information.organism_information_vec[organism_number].radius <=
            SMALL_GRID_CIRCLE_MAX_RADIUS
        {
            // Add the previous main circle to the list of previous spots.
            list_of_previous_grid_spots.push((
                (previous_x + game_settings.map_width / 2) / SMALL_GRID_SIZE,
                (previous_y + game_settings.map_height / 2) / SMALL_GRID_SIZE,
            ));

            // Add the new main circle to the list of new spots.
            list_of_new_grid_spots.push((
                (all_spatial_biosphere_information.organism_information_vec
                    [organism_number].x_location +
                    game_settings.map_width / 2) /
                    SMALL_GRID_SIZE,
                (all_spatial_biosphere_information.organism_information_vec
                    [organism_number].y_location +
                    game_settings.map_height / 2) /
                    SMALL_GRID_SIZE,
            ));

            // Add the previous other circles to the list of old spots.
            for previous_other_circles in previous_other_circles.iter() {
                list_of_previous_grid_spots.push((
                    (previous_other_circles.x + game_settings.map_width / 2) / SMALL_GRID_SIZE,
                    (previous_other_circles.y + game_settings.map_height / 2) / SMALL_GRID_SIZE,
                ));
            }

            // Add the new other circles to the list of new spots.
            for circle in all_spatial_biosphere_information.organism_information_vec[
                organism_number
            ].other_circle_positions.iter() {
                list_of_new_grid_spots.push((
                    (circle.x + game_settings.map_width / 2) / SMALL_GRID_SIZE,
                    (circle.y + game_settings.map_height / 2) / SMALL_GRID_SIZE,
                ));
            }

            // Erase records for grid spots that are in previous but not in new
            for grid_spot in list_of_previous_grid_spots.iter() {
                if !list_of_new_grid_spots.contains(grid_spot) {
                    all_spatial_biosphere_information.collision_detection_grid_small[
                        grid_spot.0 as usize
                    ][grid_spot.1 as usize].retain(
                        |&index|
                            index.identity_number != organism_number ||
                            index.circle_entity_type != CircleEntityType::Organism
                    );
                }
            }

            // Write record for grid spots that are in new but not previous
            for grid_spot in list_of_new_grid_spots.iter() {
                if !list_of_previous_grid_spots.contains(grid_spot) {
                    all_spatial_biosphere_information.collision_detection_grid_small[
                        grid_spot.0 as usize
                    ][grid_spot.1 as usize].push(CirclePositionRecord {
                        circle_entity_type: CircleEntityType::Organism,
                        identity_number: organism_number,
                        main_circle: true,
                    });
                }
            }

            // Make sure the correct record is marked as main
            // If the main circle is now in a new grid spot.
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
                // Mark the old record as not main.
                for record in all_spatial_biosphere_information.collision_detection_grid_small[
                    ((previous_x + game_settings.map_width / 2) / SMALL_GRID_SIZE) as usize
                ][
                    ((previous_y + game_settings.map_height / 2) / SMALL_GRID_SIZE) as usize
                ].iter_mut() {
                    if record.main_circle == true && record.identity_number == organism_number {
                        record.main_circle = false;
                    }
                }

                // Mark the new record as main.
                for record in all_spatial_biosphere_information.collision_detection_grid_small[
                    ((all_spatial_biosphere_information.organism_information_vec
                        [organism_number].x_location +
                        game_settings.map_width / 2) /
                        SMALL_GRID_SIZE) as usize
                ][
                    ((all_spatial_biosphere_information.organism_information_vec
                        [organism_number].y_location +
                        game_settings.map_height / 2) /
                        SMALL_GRID_SIZE) as usize
                ].iter_mut() {
                    if record.main_circle == false && record.identity_number == organism_number {
                        record.main_circle = true;
                    }
                }
            }

            // If the radius is larger than the maximum for the small grid.
        } else {
            // Add the previous main circle to the list of previous spots.
            list_of_previous_grid_spots.push((
                (previous_x + game_settings.map_width / 2) / LARGE_GRID_SIZE,
                (previous_y + game_settings.map_height / 2) / LARGE_GRID_SIZE,
            ));

            // Add the new main circle to the list of new spots.
            list_of_new_grid_spots.push((
                (all_spatial_biosphere_information.organism_information_vec
                    [organism_number].x_location +
                    game_settings.map_width / 2) /
                    LARGE_GRID_SIZE,
                (all_spatial_biosphere_information.organism_information_vec
                    [organism_number].y_location +
                    game_settings.map_height / 2) /
                    LARGE_GRID_SIZE,
            ));

            // Add the previous other circles to the list of old spots.
            for previous_other_circles in previous_other_circles.iter() {
                list_of_previous_grid_spots.push((
                    (previous_other_circles.x + game_settings.map_width / 2) / LARGE_GRID_SIZE,
                    (previous_other_circles.y + game_settings.map_height / 2) / LARGE_GRID_SIZE,
                ));
            }

            // Add the new other circles to the list of new spots.
            for circle in all_spatial_biosphere_information.organism_information_vec[
                organism_number
            ].other_circle_positions.iter() {
                list_of_new_grid_spots.push((
                    (circle.x + game_settings.map_width / 2) / LARGE_GRID_SIZE,
                    (circle.y + game_settings.map_height / 2) / LARGE_GRID_SIZE,
                ));
            }

            // Erase records for grid spots that are in previous but not in new
            for grid_spot in list_of_previous_grid_spots.iter() {
                if !list_of_new_grid_spots.contains(grid_spot) {
                    all_spatial_biosphere_information.collision_detection_grid_large[
                        grid_spot.0 as usize
                    ][grid_spot.1 as usize].retain(
                        |&index|
                            index.identity_number != organism_number ||
                            index.circle_entity_type != CircleEntityType::Organism
                    );
                }
            }

            // Write record for grid spots that are in new but not previous
            for grid_spot in list_of_new_grid_spots.iter() {
                if !list_of_previous_grid_spots.contains(grid_spot) {
                    all_spatial_biosphere_information.collision_detection_grid_large[
                        grid_spot.0 as usize
                    ][grid_spot.1 as usize].push(CirclePositionRecord {
                        circle_entity_type: CircleEntityType::Organism,
                        identity_number: organism_number,
                        main_circle: true,
                    });
                }
            }

            // Make sure the correct record is marked as main
            // If the main circle is now in a new grid spot.
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
                // Mark the old record as not main.
                for record in all_spatial_biosphere_information.collision_detection_grid_large[
                    ((previous_x + game_settings.map_width / 2) / LARGE_GRID_SIZE) as usize
                ][
                    ((previous_y + game_settings.map_height / 2) / LARGE_GRID_SIZE) as usize
                ].iter_mut() {
                    if record.main_circle == true && record.identity_number == organism_number {
                        record.main_circle = false;
                    }
                }

                // Mark the new record as main.
                for record in all_spatial_biosphere_information.collision_detection_grid_large[
                    ((all_spatial_biosphere_information.organism_information_vec
                        [organism_number].x_location +
                        game_settings.map_width / 2) /
                        LARGE_GRID_SIZE) as usize
                ][
                    ((all_spatial_biosphere_information.organism_information_vec
                        [organism_number].y_location +
                        game_settings.map_height / 2) /
                        LARGE_GRID_SIZE) as usize
                ].iter_mut() {
                    if record.main_circle == false && record.identity_number == organism_number {
                        record.main_circle = true;
                    }
                }
            }
        }
    }
}
