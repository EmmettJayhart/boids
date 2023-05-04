use bevy::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
#[cfg(not(feature = "dev"))]
use {bevy_boids::BoidDescriptor, bevy_inspector_egui::quick::ResourceInspectorPlugin};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(feature = "dev"))]
        app.add_plugin(ResourceInspectorPlugin::<BoidDescriptor>::default());

        #[cfg(feature = "dev")]
        app.add_plugin(WorldInspectorPlugin);
    }
}
