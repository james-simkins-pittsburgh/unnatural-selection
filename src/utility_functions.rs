use bevy::prelude::*;
use deterministic_trigonometry::DTrig;

// This module provides a cheaper alternative to using the random number generator every time.
pub mod cheap_random;

#[derive(Resource)]
pub struct DeterministicTrig {
    pub d_trig: DTrig,
}

impl Default for DeterministicTrig {
    fn default() -> Self {
        DeterministicTrig { d_trig: DTrig::initialize() }
    }
}