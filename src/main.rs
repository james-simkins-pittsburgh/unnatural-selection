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
// Scaffold code is code that exists to stand in for parts of coe that haven't been written yet.
pub mod scaffold_code;
// Test code is code that exists to test parts of code without initializing the app.
pub mod test_code;

// Set as constant for testing purposes This will eventually not be a constant
pub const MAP_WIDTH: usize = 10000;
pub const MAP_MAX_HEIGHT: usize = 19000;

/* This is a toggle that allows quick testing of test code.
Delete the following line before release. */
const USE_TEST_CODE: bool = false;

fn main() {
    // Delete starting here to remove test code runner before release.

    if USE_TEST_CODE {
        test_code::run_test_code()
    } else {
        // End delete here.

        App::new()
            .add_plugins((EmbeddedAssetPlugin::default(), DefaultPlugins))
            .add_plugins(bevy_framepace::FramepacePlugin)
            .add_systems(Startup, framepace_setup)
            .add_systems(
                Startup,
                (
                    scaffold_code::quick_start::create_basic_world,
                    scaffold_code::quick_start::populate_basic_world,
                ).chain()
            )

            .run();
        // Delete following "}" before release.
    }
}

/* This limits the framerate to 30 fps. This is a deliberate decision to deemphasize graphic quality
in order to double the potential size of the simulation since this is a CPU bound appllication. */
fn framepace_setup(mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    settings.limiter = bevy_framepace::Limiter::from_framerate(30.0);
}
