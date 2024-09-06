use crate::{settings::GameSettings, simulation::biosphere_simulation::collision_calculations::SMALL_GRID_CIRCLE_MAX_RADIUS, utility_functions::deterministic_trigonometry::DeterministicTrig};
use super::{CollideeCircleInfo, ColliderCircleInfo};


pub fn make_collidee_circle_list (
    collider_circle: &ColliderCircleInfo,
    blob_number: usize,
    game_settings: &GameSettings,
    deterministic_trig: &DeterministicTrig,
    x_move: i32,
    y_more: i32,
    r_move: i32,
) -> Vec<CollideeCircleInfo> {

    if collider_circle.radius <= SMALL_GRID_CIRCLE_MAX_RADIUS {


    } else {


    }

    return vec![];
}