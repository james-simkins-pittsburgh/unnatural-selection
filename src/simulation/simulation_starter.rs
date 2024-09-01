use bevy::prelude::*;

use crate::simulation::biosphere_simulation::collision_calculations::detection_grid_maker::make_detection_grid;

use super::
    biosphere_simulation::collision_calculations::detection_grid_updater
;

pub fn create_simulation(
    game_settings: Res<crate::settings::GameSettings>,
    mut all_spatial_biosphere_information: Query<&mut crate::simulation::AllSpatialBiosphereInformation>
) {
    for mut spatial_biosphere in &mut all_spatial_biosphere_information {
        make_detection_grid(&mut spatial_biosphere, &game_settings);

        // This is scaffold code.
        for organism_number in 1..spatial_biosphere.organism_information_vec.len() {
            let previous_circle_positions =
                spatial_biosphere.organism_information_vec[
                    organism_number
                ].other_circle_positions.clone();

            let previous_x = spatial_biosphere.organism_information_vec[organism_number].x_location + 10001;

            let previous_y = spatial_biosphere.organism_information_vec[organism_number].y_location + 10001;

            detection_grid_updater::update_for_movement(
                &mut spatial_biosphere,
                previous_x,
                previous_y,
                &previous_circle_positions,
                organism_number,
                &game_settings
            );
            // End scaffold code.
        }
    }
}
