// This hides the command prompt
#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;

pub mod graphical_world;
pub mod user_interface;
pub mod network_communicator;
pub mod simulation;
pub mod computer_players;
pub mod utility_functions;
pub mod test_code;

/* Set as constant for testing purposes
This will eventually not be a constant */

const MAP_WIDTH:usize = 250000;
const MAP_MAX_DEPTH:usize = 150000;

fn main() {
    App::new()
        .add_plugins((EmbeddedAssetPlugin::default(), DefaultPlugins))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .run();
}