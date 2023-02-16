use bevy::prelude::*;
use bevy_boids::BoidDescriptor;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ResourceInspectorPlugin::<BoidDescriptor>::default());
    }
}
