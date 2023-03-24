use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_boids::{Boid, BoidDescriptor};
use bevy_rapier3d::prelude::*;
use rand::prelude::*;

use crate::{input, player, ui};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_world)
            .add_startup_system(setup_boids);

        app.add_plugin(ui::UiPlugin)
            .add_plugin(input::InputPlugin)
            .add_plugin(player::PlayerPlugin);
    }
}

fn setup_world(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.,
    });

    commands
        .spawn(DirectionalLightBundle::default())
        .insert(Name::new("Sunlight"));
}

fn setup_boids(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let num_boids = 512;
    commands.insert_resource(BoidDescriptor::default());

    let mesh = asset_server.load("models/bird.glb#Mesh0/Primitive0");

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    commands
        .spawn(SpatialBundle::default())
        .insert(Name::new("Boids"))
        .with_children(|parent| {
            let mut rng = rand::thread_rng();
            for i in 0..num_boids {
                let mut transform = Transform::default();
                transform.translation.x = (rng.gen::<f32>() - 0.5) * 10.0;
                transform.translation.y = (rng.gen::<f32>() - 0.5) * 10.0;
                transform.translation.z = (rng.gen::<f32>() - 0.5) * 10.0;

                parent
                    .spawn(PbrBundle {
                        mesh: mesh.clone(),
                        material: debug_material.clone(),
                        transform,
                        ..default()
                    })
                    .insert(Name::new(format!("Boid {i}")))
                    .insert(Boid)
                    .insert(Collider::ball(0.5));
            }
        });
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}
