use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::input::mouse::MouseScrollUnit;
use bevy::render::camera::ScalingMode;
use bevy::window::PrimaryWindow;

const PAN_TOP_SPEED: f32 = 48.0;
const ZOOM_OUT_MAX: f32 = 12.0;
const ZOOM_SPEED: f32 = 0.1;

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

    main_camera.scale = 4.0;
    main_camera.scaling_mode = ScalingMode::AutoMax { max_width: 1920.0, max_height: 1080.0 };
}

// This controls the pan and zoom abilities of the camera.

pub fn camera_pan_and_zoom(
    mut scroll_event_reader: EventReader<MouseWheel>,
    mut main_camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let mut main_camera = main_camera_query.single_mut();

    // This pans the camera if the cursor position is on the edge of the screen.

    if let Some(position) = window_query.single().cursor_position() {
        if position.y > window_query.single().height() - 50.0 {
            main_camera.0.translation.y -=
                (PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) / 2.0;
        }

        if position.y < 50.0 {
            main_camera.0.translation.y +=
                (PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) / 2.0;
        }

        if position.x > window_query.single().width() - 50.0 {
            main_camera.0.translation.x +=
                (PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) / 2.0;
        }

        if position.x < 50.0 {
            main_camera.0.translation.x -=
                (PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) / 2.0;
        }
    }

    // This zooms in or out if the mouse wheel is turned.

    for event in scroll_event_reader.read() {
        match event.unit {
            MouseScrollUnit::Line => {
                if event.y > 0.0 {
                    main_camera.1.scale *= 1.0 - ZOOM_SPEED;
                } else if event.y < 0.0 {
                    main_camera.1.scale *= 1.0 + ZOOM_SPEED;
                }
            }
            MouseScrollUnit::Pixel => {
                if event.y > 0.0 {
                    main_camera.1.scale *= 1.0 - ZOOM_SPEED;
                } else if event.y < 0.0 {
                    main_camera.1.scale *= 1.0 + ZOOM_SPEED;
                }
            }
        }
    }

    main_camera.1.scale = main_camera.1.scale.clamp(1.0, ZOOM_OUT_MAX);
}
