use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::input::mouse::MouseScrollUnit;
use bevy::render::camera::ScalingMode;
use bevy::window::PrimaryWindow;

// This is a marker component for the main camera.

#[derive(Component)]
pub struct MainCamera;

// This spawns the camera using the Bevy default 2D settings.

pub fn camera_setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

pub fn set_initial_camera(
    mut main_camera_query: Query<&mut OrthographicProjection, With<MainCamera>>
) {
    let mut main_camera = main_camera_query.single_mut();

    main_camera.scale = 2.0;
    main_camera.scaling_mode = ScalingMode::AutoMax { max_width: 1920.0, max_height: 1080.0 };

}
