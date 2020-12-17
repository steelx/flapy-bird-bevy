use crate::prelude::*;
use super::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup_system.system())
            .add_system(camera_follow_system.system());
    }
}

#[derive(Debug, Default)]
pub struct Camera;

fn setup_system(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default()).with(Camera);

    // Set cursor as hidden, replaced by crosshair
    // let window = windows.get_primary_mut().unwrap();
    // window.set_cursor_visibility(false);
}

fn camera_follow_system(
    windows: Res<Windows>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let smooth_speed: f32 = 0.125;
    let offset = Vec2::new(0., 2.);
    let window = windows.get_primary().unwrap();
    let half_height = window.height() as f32 * 0.5;

    for mut transform in camera_query.iter_mut() {
        for target_transform in player_query.iter() {
            let desired_position = target_transform.translation.truncate() + offset;
            if desired_position.y >= half_height || desired_position.y <= -(half_height) {
                return;// we stop camera if player goes out of bounds
            }

            let smoothed_position = transform
                .translation
                .truncate()
                .lerp(desired_position, smooth_speed);
            transform.translation = smoothed_position.extend(10.);
        }
    }
}
