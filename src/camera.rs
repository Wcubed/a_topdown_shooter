use crate::input::{Action, ActionRes};
use crate::GameState;
use bevy::prelude::*;

/// Speed in game units per second.
const CAMERA_SPEED: f32 = 500.0;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Main).with_system(camera_setup_system))
            .add_system_set(
                SystemSet::on_update(GameState::Main).with_system(camera_movement_system),
            );
    }
}

#[derive(Component)]
struct GameCameraMarker;

fn camera_setup_system(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(GameCameraMarker);
}

fn camera_movement_system(
    mut camera_query: Query<&mut Transform, With<GameCameraMarker>>,
    time: Res<Time>,
    actions: ActionRes,
) {
    let Ok(mut transform) = camera_query.get_single_mut() else { return; };

    let mut movement_per_second = Vec2::default();

    if actions.pressed(Action::CameraUp) {
        movement_per_second.y += CAMERA_SPEED;
    }
    if actions.pressed(Action::CameraDown) {
        movement_per_second.y -= CAMERA_SPEED;
    }
    if actions.pressed(Action::CameraLeft) {
        movement_per_second.x -= CAMERA_SPEED;
    }
    if actions.pressed(Action::CameraRight) {
        movement_per_second.x += CAMERA_SPEED;
    }

    let movement = movement_per_second * time.delta_seconds();
    transform.translation.x += movement.x;
    transform.translation.y += movement.y;
}
