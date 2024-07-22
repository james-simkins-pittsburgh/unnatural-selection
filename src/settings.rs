use bevy::prelude::*;

// This resource holds the game settings.
#[derive(Resource)]
pub struct GameSettings {
    map_width: usize,
    map_depth: usize,
    current_intensity: i32,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            map_width: 10000,
            map_depth: 19000,
            current_intensity: 100,
        }
    }
}
