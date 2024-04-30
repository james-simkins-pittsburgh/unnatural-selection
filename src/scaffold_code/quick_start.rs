use bevy::prelude::*;

use crate::simulation;

fn create_basic_world(mut commands: Commands) {

    commands.spawn(crate::simulation::gameworld_bundle) {};


}

fn populate_basic_world() {


}
