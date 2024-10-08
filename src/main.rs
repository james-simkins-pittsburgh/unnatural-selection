// This hides the command prompt
// #![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;

pub mod graphical_world;
pub mod user_interface;
pub mod network_communicator;
pub mod simulation;
pub mod computer_players;
pub mod utility_functions;
pub mod settings;
// Scaffold code is code that exists to stand in for parts of code that haven't been written yet.
pub mod scaffold_code;

fn main() {
    App::new()
        .add_plugins((EmbeddedAssetPlugin::default(), DefaultPlugins))
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, framepace_setup)
        .add_systems(
            Startup,
            (
                user_interface::camera::camera_setup,
                user_interface::camera::set_initial_camera,
            ).chain()
        )
        .init_resource::<graphical_world::texture_loader::TextureAtlasHandles>()
        .add_systems(Startup, graphical_world::texture_loader::texture_loader)
        .init_resource::<utility_functions::deterministic_trigonometry::DeterministicTrig>()
        .init_resource::<settings::GameSettings>()
        .init_resource::<graphical_world::OrganismsToUnboundFromGraphicalPartner>()
        .init_resource::<graphical_world::OrganismsThatNeedGraphicalPartner>()
        .init_resource::<graphical_world::NumberOfUnboundOrganisms>()
        .add_systems(
            Startup,
            (
                scaffold_code::quick_start::create_basic_world,
                scaffold_code::quick_start::populate_basic_world,
                scaffold_code::quick_start::add_to_graphics,
                simulation::simulation_starter::create_simulation,
            ).chain()
        )
        .add_systems(
            Update,
            (
                simulation::simulation_stepper::step_spatial_simulation,
                user_interface::camera::camera_pan_and_zoom,
                graphical_world::graphics_assigner::unassign_graphical_entities,
                graphical_world::graphics_assigner::create_graphical_entities,
                graphical_world::graphics_assigner::assign_graphical_entities,
                graphical_world::graphics_updater::update_graphical_world,
            ).chain()
        )

        .run();
}

/* This limits the framerate to 30 fps. This is a deliberate decision to deemphasize graphic quality
in order to double the potential size of the simulation since this is a CPU bound application. */
fn framepace_setup(mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    settings.limiter = bevy_framepace::Limiter::from_framerate(30.0);
}
