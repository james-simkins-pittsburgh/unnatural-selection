// This module holds everything related to collisions.

// This module makes the detection grid.
pub mod detection_grid_maker;

// This module uses the detection grid to detect collisions.
pub mod collision_detector;

// This module calculates the results of organisms colliding.
pub mod organism_combination;

// This module updates the detection grid;
pub mod detection_grid_updater;

pub const SMALL_GRID_SIZE: i32 = 6000;
pub const LARGE_GRID_SIZE: i32 = 12000;
pub const SMALL_GRID_CIRCLE_MAX_RADIUS: i32 = 2500;
pub const LARGE_GRID_CIRCLE_MAX_RADIUS: i32 = 5500;
pub const SPEED_LIMIT: i32 = 900;