use crate::GameState;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

/// Speed in game units per second.
const CAMERA_SPEED: f32 = 500.0;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<CameraAction>::default())
            .add_system_set(SystemSet::on_enter(GameState::Main).with_system(camera_setup_system))
            .add_system_set(
                SystemSet::on_update(GameState::Main).with_system(camera_movement_system),
            );
    }
}

#[derive(Component)]
struct GameCameraMarker;

fn camera_setup_system(mut commands: Commands) {
    let input_map = InputMap::new([
        (KeyCode::W, CameraAction::Up),
        (KeyCode::S, CameraAction::Down),
        (KeyCode::A, CameraAction::Left),
        (KeyCode::D, CameraAction::Right),
    ]);

    commands
        .spawn(Camera2dBundle::default())
        .insert(InputManagerBundle::<CameraAction> {
            action_state: ActionState::default(),
            input_map,
        })
        .insert(GameCameraMarker);
}

fn camera_movement_system(
    time: Res<Time>,
    mut query: Query<(&ActionState<CameraAction>, &mut Transform), With<GameCameraMarker>>,
) {
    if let Ok((action_state, mut transform)) = query.get_single_mut() {
        let mut movement_per_second = Vec2::default();

        if action_state.pressed(CameraAction::Up) {
            movement_per_second.y += CAMERA_SPEED;
        }
        if action_state.pressed(CameraAction::Down) {
            movement_per_second.y -= CAMERA_SPEED;
        }
        if action_state.pressed(CameraAction::Left) {
            movement_per_second.x -= CAMERA_SPEED;
        }
        if action_state.pressed(CameraAction::Right) {
            movement_per_second.x += CAMERA_SPEED;
        }

        let movement = movement_per_second * time.delta_seconds();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
    }
}

#[derive(Actionlike, Eq, PartialEq, Clone, Copy, Hash, Debug)]
enum CameraAction {
    Up,
    Down,
    Left,
    Right,
}
