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

#[derive(Debug, Default, Properties)]
pub struct Camera;

fn setup_system(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands.spawn(Camera2dComponents::default()).with(Camera);

    // Set cursor as hidden, replaced by crosshair
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
}

fn camera_follow_system(
    mut query1: Query<With<Camera, &mut Transform>>,
    query2: Query<With<Player, &Transform>>,
) {
    let smooth_speed: f32 = 0.125;
    let offset = Vec2::new(0., 2.);

    for mut transform in query1.iter_mut() {
        for target_transform in query2.iter() {
            let desired_position = target_transform.translation.truncate() + offset;
            let smoothed_position = transform
                .translation
                .truncate()
                .lerp(desired_position, smooth_speed);
            transform.translation = smoothed_position.extend(10.);
        }
    }
}
