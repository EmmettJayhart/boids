use bevy::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::Inspectable;
use rand::prelude::*;

use crate::{input, player};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_world)
            .add_startup_system(setup_boids);

        app.add_system(move_boid);

        app.add_plugin(input::InputPlugin)
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

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Inspectable))]
pub struct Boid {
    speed: f32,
}

fn setup_boids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.025,
        subdivisions: 3,
    }));

    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.3, 0.3, 0.8),
        ..default()
    });

    let num_boids = 20usize;

    commands
        .spawn(SpatialBundle::default())
        .insert(Name::new("Boids"))
        .with_children(|parent| {
            let mut rng = rand::thread_rng();
            for i in 0..num_boids {
                parent
                    .spawn(PbrBundle {
                        mesh: mesh.clone(),
                        material: material.clone(),
                        transform: Transform::from_xyz(
                            rng.gen::<f32>() * i as f32,
                            rng.gen::<f32>() * i as f32,
                            rng.gen::<f32>() * i as f32,
                        ),
                        ..default()
                    })
                    .insert(Name::new(format!("Boid {}", i)))
                    .insert(Boid { speed: 0.5 });
            }
        });
}

fn move_boid(mut boids_query: Query<(Entity, &mut Transform, &Boid)>, time: Res<Time>) {
    let mut net_force = Vec::with_capacity(boids_query.iter().count());
    for (entity, transform, _) in boids_query.iter() {
        let mut separation = Vec3::ZERO;
        let mut cohesion = Vec3::ZERO;

        for (other_entity, other_transform, _) in boids_query.iter() {
            if other_entity == entity {
                continue;
            }

            let distance = transform.translation.distance(other_transform.translation);
            let distance_squared = distance.powi(2);

            separation += (transform.translation - other_transform.translation).normalize()
                / distance_squared;

            cohesion += (other_transform.translation - transform.translation).normalize()
                * distance_squared;
        }

        cohesion /= boids_query.iter().count() as f32;

        net_force.push(separation + cohesion);
    }

    for (i, (_, mut transform, boid)) in boids_query.iter_mut().enumerate() {
        transform.translation += net_force[i] * boid.speed * time.delta_seconds();
    }
}
