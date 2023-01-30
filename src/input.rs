use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_plugin(InputManagerPlugin::<CameraAction>::default());
    }
}

#[derive(Actionlike, Clone, Copy)]
pub enum PlayerAction {
    Leftward,
    Rightward,
    Upward,
    Downward,
    Backward,
    Forward,
    Sprint,
    HorizontalPan,
}

pub fn player_input_map() -> InputMap<PlayerAction> {
    InputMap::new([
        // WASD
        (KeyCode::W, PlayerAction::Forward),
        (KeyCode::A, PlayerAction::Leftward),
        (KeyCode::S, PlayerAction::Backward),
        (KeyCode::D, PlayerAction::Rightward),
        // Arrows
        (KeyCode::Up, PlayerAction::Forward),
        (KeyCode::Left, PlayerAction::Leftward),
        (KeyCode::Down, PlayerAction::Backward),
        (KeyCode::Right, PlayerAction::Rightward),
        // Vertical
        (KeyCode::E, PlayerAction::Upward),
        (KeyCode::Space, PlayerAction::Upward),
        (KeyCode::Q, PlayerAction::Downward),
        (KeyCode::LControl, PlayerAction::Downward),
        // Misc
        (KeyCode::LShift, PlayerAction::Sprint),
    ])
    .insert(SingleAxis::mouse_motion_x(), PlayerAction::HorizontalPan)
    .build()
}

#[derive(Actionlike, Clone, Copy)]
pub enum CameraAction {
    VerticalPan,
}

pub fn camera_input_map() -> InputMap<CameraAction> {
    InputMap::default()
        .insert(SingleAxis::mouse_motion_y(), CameraAction::VerticalPan)
        .build()
}
