use bevy::prelude::*;

// This resource holds the game settings.
#[derive(Resource)]
pub struct GameSettings {
    pub map_width: usize,
    pub map_depth: usize,
    pub current_intensity: i32,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            map_width: 100000,
            map_depth: 190000,
            current_intensity: 100,
        }
    }
}
