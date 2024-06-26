use bevy::prelude::*;

// This module provides a cheaper alternative to using the random number generator every time.
pub mod cheap_random;
// This module provides deterministic trignometry
pub mod deterministic_trig;

#[derive(Resource, Default)]

pub struct DeterministicTrig {

    pub deterministic_trig: deterministic_trig::DeterministicTrig,

}
