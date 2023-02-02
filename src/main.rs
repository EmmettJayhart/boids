#![cfg_attr(
    target_os = "windows",
    cfg_attr(not(feature = "dev"), windows_subsystem = "windows")
)]

#[cfg(feature = "dev")]
mod debug;
mod game;
mod input;
mod player;

use std::io::Cursor;

use bevy::{
    prelude::*,
    window::{CursorGrabMode, PresentMode, WindowId, WindowMode},
    winit::WinitWindows,
};
use bevy_boids::*;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};
use winit::window::Icon;

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb_u8(90, 112, 56)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Boids".to_string(),
                        present_mode: PresentMode::AutoVsync,
                        mode: WindowMode::BorderlessFullscreen,
                        resizable: false,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(BoidsPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(game::GamePlugin);

    #[cfg(feature = "dev")]
    app.add_plugin(debug::DebugPlugin);

    app.add_startup_system(set_window_icon)
        .add_startup_system(grab_mouse)
        .run();
}

fn set_window_icon(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let icon_buf = Cursor::new(include_bytes!("../assets/textures/icon_128x128.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}

fn grab_mouse(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_grab_mode(CursorGrabMode::Locked);
    window.set_cursor_visibility(false);
}
