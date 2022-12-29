use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, KeyCode, Res};
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

pub type ActionRes<'a> = Res<'a, ActionState<Action>>;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .add_startup_system(add_input_actions);
    }
}

#[derive(Actionlike, Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum Action {
    HelloAction,
    CameraUp,
    CameraDown,
    CameraLeft,
    CameraRight,
    CameraZoomIn,
    CameraZoomOut,
}

fn add_input_actions(mut commands: Commands) {
    let mut input_map = InputMap::default();
    input_map
        .insert(KeyCode::Space, Action::HelloAction)
        .insert(KeyCode::W, Action::CameraUp)
        .insert(KeyCode::S, Action::CameraDown)
        .insert(KeyCode::A, Action::CameraLeft)
        .insert(KeyCode::D, Action::CameraRight)
        .insert(
            InputKind::MouseWheel(MouseWheelDirection::Up),
            Action::CameraZoomIn,
        )
        .insert(
            InputKind::MouseWheel(MouseWheelDirection::Down),
            Action::CameraZoomOut,
        )
        .insert_chord(
            [
                InputKind::Modifier(Modifier::Shift),
                InputKind::Keyboard(KeyCode::H),
            ],
            Action::HelloAction,
        );

    commands.insert_resource(input_map);
    commands.insert_resource(ActionState::<Action>::default())
}
