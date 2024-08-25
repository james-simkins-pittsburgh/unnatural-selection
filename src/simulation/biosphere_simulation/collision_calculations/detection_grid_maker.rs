use crate::settings::GameSettings;
use crate::simulation::AllBiosphereInformation;

use super::GRID_SIZE;

pub fn make_detection_grid(
    all_biosphere_information: &mut AllBiosphereInformation,
    game_settings: &GameSettings
) {
    
    let grid_height: usize;
    let grid_width: usize;
    
    if game_settings.map_height % GRID_SIZE == 0 {
        grid_height = (game_settings.map_height / GRID_SIZE) as usize;
    } else {
        grid_height = (game_settings.map_height / GRID_SIZE + 1) as usize;
    }

    if game_settings.map_height % GRID_SIZE == 0 {
        grid_width = (game_settings.map_length / GRID_SIZE) as usize;
    } else {
        grid_width = (game_settings.map_length / GRID_SIZE + 1) as usize;
    }

    for height in 0..grid_height {

        all_biosphere_information.collision_detection_grid.push(Vec::new());
        all_biosphere_information.detritus_detection_grid.push(Vec::new());  

        for _width in 0..grid_width {

            all_biosphere_information.collision_detection_grid [height].push(Vec::new());
            all_biosphere_information.detritus_detection_grid [height].push(Vec::new()); 

        }
    }
}
