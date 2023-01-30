use bevy::prelude::*;
#[cfg(feature = "dev")]
use bevy::reflect::Reflect;
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
#[cfg_attr(feature = "dev", derive(Reflect))]
pub struct Boid {
    speed: f32,
    lerp_factor: f32,
    separation_factor: f32,
    alignment_factor: f32,
    cohesion_factor: f32,
}

fn setup_boids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.25,
        subdivisions: 3,
    }));

    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.3, 0.3, 0.8),
        ..default()
    });

    let num_boids = 144usize;

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
                        transform: {
                            let mut t = Transform::default();
                            t.translation.x = (rng.gen::<f32>() - 0.5) * 10.0;
                            t.translation.y = (rng.gen::<f32>() - 0.5) * 10.0;
                            t.translation.z = (rng.gen::<f32>() - 0.5) * 10.0;
                            t.rotate_local_x((rng.gen::<f32>() - 0.5) * std::f32::consts::TAU);
                            t.rotate_local_x((rng.gen::<f32>() - 0.5) * std::f32::consts::TAU);
                            t.rotate_local_x((rng.gen::<f32>() - 0.5) * std::f32::consts::TAU);
                            t
                        },
                        ..default()
                    })
                    .insert(Name::new(format!("Boid {}", i)))
                    .insert(Boid {
                        speed: 1.0,
                        lerp_factor: 0.125,
                        separation_factor: 0.1,
                        alignment_factor: 0.2,
                        cohesion_factor: 3.0,
                    });
            }
        });
}

fn move_boid(mut boids_query: Query<(Entity, &mut Transform, &Boid)>, time: Res<Time>) {
    let mut net_heading = Vec::with_capacity(boids_query.iter().count());
    for (entity, transform, boid) in boids_query.iter() {
        let mut separation = Vec3::ZERO;
        let mut alignment = Vec3::ZERO;
        let mut cohesion = Vec3::ZERO;

        for (other_entity, other_transform, _) in boids_query.iter() {
            if other_entity == entity {
                continue;
            }

            let distance = transform.translation.distance(other_transform.translation);
            let distance_squared = distance.powi(2);

            separation += (transform.translation - other_transform.translation) / distance_squared;
            alignment += (other_transform.forward() - transform.forward()) / distance_squared;
            cohesion += (other_transform.translation - transform.translation) / distance;
        }

        separation *= boid.separation_factor;
        alignment *= boid.alignment_factor;
        cohesion *= boid.cohesion_factor / boids_query.iter().count() as f32;

        net_heading.push((separation + alignment + cohesion).normalize());
    }

    for (i, (_, mut transform, boid)) in boids_query.iter_mut().enumerate() {
        transform.rotation = transform.rotation.lerp(
            Quat::from_rotation_arc(transform.forward(), net_heading[i]),
            boid.lerp_factor,
        );
        let forward = transform.forward();
        transform.translation += forward * boid.speed * time.delta_seconds();
    }
}
