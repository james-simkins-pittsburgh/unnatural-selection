use bevy::prelude::*;

use crate::simulation::biosphere_simulation::collision_detector::make_detection_grid;

pub fn create_simulation(
    game_settings: Res<crate::settings::GameSettings>,
    mut all_biosphere_information: Query<&mut crate::simulation::AllBiosphereInformation>
) {

    for mut biosphere in &mut all_biosphere_information {

    make_detection_grid(&mut biosphere, &game_settings);

    }
}
