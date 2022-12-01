use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::Inspectable;
use leafwing_input_manager::prelude::*;

use crate::input;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(camera_vertical_pan)
            .add_system(player_horizontal_pan)
            .add_system(player_movement);
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Inspectable))]
pub struct Player {
    speed: f32,
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn(SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(Player { speed: 2.5 })
        .insert(InputManagerBundle::<input::PlayerAction> {
            input_map: input::player_input_map(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 0.4, 0.0),
                    ..default()
                })
                .insert(Name::new("Camera"))
                .insert(InputManagerBundle::<input::CameraAction> {
                    input_map: input::camera_input_map(),
                    ..default()
                });
        });
}

fn camera_vertical_pan(
    mut query: Query<(&mut Transform, &ActionState<input::CameraAction>), With<Camera3d>>,
    time: Res<Time>,
) {
    const VERTICAL_PAN_RATE: f32 = 0.4;

    for (mut transform, action_state) in query.iter_mut() {
        let vertical_pan = -action_state.value(input::CameraAction::VerticalPan);
        let delta = vertical_pan * VERTICAL_PAN_RATE * time.delta_seconds();

        let new_rotation_x = transform.rotation.to_euler(EulerRot::XYZ).0 + delta;
        if new_rotation_x > -FRAC_PI_2 && new_rotation_x < FRAC_PI_2 {
            transform.rotate_x(delta);
        }
    }
}

fn player_horizontal_pan(
    mut query: Query<(&mut Transform, &ActionState<input::PlayerAction>), With<Player>>,
    time: Res<Time>,
) {
    const HORIZONTAL_PAN_RATE: f32 = 0.4;

    for (mut transform, action_state) in query.iter_mut() {
        let horizontal_pan = -action_state.value(input::PlayerAction::HorizontalPan);
        let delta = horizontal_pan * HORIZONTAL_PAN_RATE * time.delta_seconds();

        transform.rotate_y(delta);
    }
}

fn player_movement(
    mut player_query: Query<(&mut Transform, &Player, &ActionState<input::PlayerAction>)>,
    mut camera_query: Query<&mut GlobalTransform, With<Camera3d>>,
    time: Res<Time>,
) {
    for (mut player_transform, player, action_state) in player_query.iter_mut() {
        let forward = action_state.pressed(input::PlayerAction::Forward);
        let leftward = action_state.pressed(input::PlayerAction::Leftward);
        let backward = action_state.pressed(input::PlayerAction::Backward);
        let rightward = action_state.pressed(input::PlayerAction::Rightward);
        let upward = action_state.pressed(input::PlayerAction::Upward);
        let downward = action_state.pressed(input::PlayerAction::Downward);

        if forward || leftward || backward || rightward || upward || downward {
            let mut movement = Vec3::ZERO;

            if forward {
                movement.z -= 1.0;
            }
            if leftward {
                movement.x -= 1.0;
            }
            if backward {
                movement.z += 1.0;
            }
            if rightward {
                movement.x += 1.0;
            }
            if upward {
                movement.y += 1.0;
            }
            if downward {
                movement.y -= 1.0;
            }

            if movement != Vec3::ZERO {
                let movement = movement.normalize_or_zero();

                let direction = match camera_query.get_single_mut() {
                    Ok(camera_transform) => {
                        (movement.x * camera_transform.right())
                            + (movement.y * camera_transform.up())
                            + (movement.z * camera_transform.back())
                    }

                    Err(_) => {
                        (movement.x * player_transform.right())
                            + (movement.y * player_transform.up())
                            + (movement.z * player_transform.back())
                    }
                };

                player_transform.translation += direction * player.speed * time.delta_seconds();
            }
        }
    }
}
