use crate::settings::GameSettings;
use crate::simulation::AllSpatialBiosphereInformation;

use super::SMALL_GRID_SIZE;
use super::LARGE_GRID_SIZE;

pub fn make_detection_grids(
    all_spatial_biosphere_information: &mut AllSpatialBiosphereInformation,
    game_settings: &GameSettings
) {
    
    let mut grid_height: usize;
    let mut grid_width: usize;
    
    if game_settings.map_height % SMALL_GRID_SIZE == 0 {
        grid_height = (game_settings.map_height / SMALL_GRID_SIZE) as usize;
    } else {
        grid_height = (game_settings.map_height / SMALL_GRID_SIZE + 1) as usize;
    }

    if game_settings.map_height % SMALL_GRID_SIZE == 0 {
        grid_width = (game_settings.map_length / SMALL_GRID_SIZE) as usize;
    } else {
        grid_width = (game_settings.map_length / SMALL_GRID_SIZE + 1) as usize;
    }

    for height in 0..grid_height {

        all_spatial_biosphere_information.collision_detection_grid_small.push(Vec::new());
        all_spatial_biosphere_information.detritus_detection_grid.push(Vec::new());  

        for _width in 0..grid_width {

            all_spatial_biosphere_information.collision_detection_grid_small [height].push(Vec::new());
            all_spatial_biosphere_information.detritus_detection_grid [height].push(Vec::new()); 

        }
    }

    if game_settings.map_height % LARGE_GRID_SIZE == 0 {
        grid_height = (game_settings.map_height / LARGE_GRID_SIZE) as usize;
    } else {
        grid_height = (game_settings.map_height / LARGE_GRID_SIZE + 1) as usize;
    }

    if game_settings.map_height % LARGE_GRID_SIZE == 0 {
        grid_width = (game_settings.map_length / LARGE_GRID_SIZE) as usize;
    } else {
        grid_width = (game_settings.map_length / LARGE_GRID_SIZE + 1) as usize;
    }

    for height in 0..grid_height {

        all_spatial_biosphere_information.collision_detection_grid_large.push(Vec::new());

        for _width in 0..grid_width {

            all_spatial_biosphere_information.collision_detection_grid_large [height].push(Vec::new());

        }
    }
}
