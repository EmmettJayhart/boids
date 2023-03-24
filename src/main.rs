#![cfg_attr(
    target_os = "windows",
    cfg_attr(not(feature = "dev"), windows_subsystem = "windows")
)]

#[cfg(feature = "dev")]
mod debug;
mod game;
mod input;
mod player;
mod ui;

use std::io::Cursor as IoCursor;

use bevy::{
    prelude::*,
    window::{Cursor, PresentMode, WindowMode},
    winit::WinitWindows,
};
use bevy_boids::BoidsPlugin;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};
use winit::window::Icon;

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::rgb_u8(90, 112, 56)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Boids".to_string(),
                        present_mode: PresentMode::AutoVsync,
                        mode: WindowMode::BorderlessFullscreen,
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(BoidsPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(game::GamePlugin);

    #[cfg(feature = "dev")]
    app.add_plugin(debug::DebugPlugin);

    app.add_startup_system(set_window_icon).run();
}

fn set_window_icon(query: Query<Entity, With<Window>>, windows: NonSend<WinitWindows>) {
    let icon_buf = IoCursor::new(include_bytes!("../assets/textures/icon_128x128.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();

        for entity in query.iter() {
            let window = windows.get_window(entity).unwrap();
            window.set_window_icon(Some(icon.clone()));
        }
    };
}
