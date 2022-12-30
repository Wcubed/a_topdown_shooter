use crate::camera::MainCamera;
use bevy::math::{Vec2, Vec3Swizzles};
use bevy::prelude::*;

#[derive(Resource, Default, Deref)]
pub struct GlobalCursorPosition(Vec2);

pub fn update_global_cursor_position(
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Res<Windows>,
    mut global_cursor_position: ResMut<GlobalCursorPosition>,
) {
    let Ok((camera, camera_transform)) = camera_query.get_single() else { return; };
    let Some(primary_window) = windows.get_primary() else { return; };
    let Some(viewport_position) = primary_window.cursor_position() else { return; };

    let Some(ray) = camera.viewport_to_world(camera_transform, viewport_position) else { return; };

    global_cursor_position.0 = ray.origin.xy();
}
