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
    y_more: i32,
    r_move: i32,
    all_spatial_biosphere_information: &AllSpatialBiosphereInformation
) -> Vec<CollideeCircleInfo> {
    let mut collidee_circles: Vec<CollideeCircleInfo> = Vec::new();

    // If it is a small circle.
    if collider_circle.radius <= SMALL_GRID_CIRCLE_MAX_RADIUS {
        let x_index_main = ((collider_circle.x + game_settings.map_width / 2) /
            SMALL_GRID_SIZE) as usize;
        let y_index_main = ((collider_circle.y + game_settings.map_height / 2) /
            SMALL_GRID_SIZE) as usize;

        // This checks the detection grid the collider circle is in.
        for circle in all_spatial_biosphere_information.collision_detection_grid_small[
            x_index_main
        ][y_index_main].iter() {
            // If the circle is either a mineral or it is from a different blob.
            if
                circle.circle_entity_type == CircleEntityType::Mineral ||
                all_spatial_biosphere_information.organism_information_vec
                    [circle.identity_number].blob_number != blob_number
            {
                
                if circle.circle_entity_type == CircleEntityType::Mineral {
                // Then add it to the collidee list.
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
                    circle_entity_type: circle.circle_entity_type,

                });

                }

            
            }
        }

        // This covers the more complicated case of big circles
    } else {
    }

    return collidee_circles;
}
