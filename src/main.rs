// This hides the command prompt
#![windows_subsystem = "windows"]

use bevy::prelude::*;
use rand::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use rand_chacha::ChaCha8Rng;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((EmbeddedAssetPlugin::default(), DefaultPlugins))
        .run();
}