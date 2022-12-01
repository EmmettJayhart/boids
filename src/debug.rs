use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
// use bevy_rapier3d::prelude::RapierDebugRenderPlugin;

use crate::player;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(WorldInspectorPlugin::default())
            // .add_plugin(RapierDebugRenderPlugin::default())
            ;

        app.register_inspectable::<player::Player>();
    }
}
