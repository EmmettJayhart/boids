use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_rapier3d::prelude::RapierDebugRenderPlugin;

use crate::{game::Boid, player::Player};

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_plugin(WorldInspectorPlugin)
            // .add_plugin(RapierDebugRenderPlugin::default())
            ;

        app.register_type::<Player>().register_type::<Boid>();
    }
}
