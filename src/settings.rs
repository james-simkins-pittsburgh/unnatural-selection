use bevy::prelude::*;

// This resource holds the game settings.
#[derive(Resource)]
pub struct GameSettings {
    pub map_width: i32,
    pub map_height: i32,
    pub current_intensity: i32,
    pub number_of_currents: i32,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            map_width: 100000,
            map_height: 100000,
            current_intensity: 2,
            number_of_currents: 4,
        }
    }
}
