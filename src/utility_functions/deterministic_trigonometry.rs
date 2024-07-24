use bevy::prelude::*;
use deterministic_trigonometry::DTrig;

#[derive(Resource)]
pub struct DeterministicTrig {
    pub d_trig: DTrig,
}

impl Default for DeterministicTrig {
    fn default() -> Self {
        DeterministicTrig { d_trig: DTrig::initialize() }
    }
}