use crate::prelude::*;

use super::player::{Player, PlayerCamera};

pub fn gravity_and_move(
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &mut Velocity, &mut Transform)>,
    mut camera_query: Query<With<PlayerCamera, &mut Transform>>,
) {
    player_query.iter_mut().for_each(|(mut player, mut velocity, mut player_position)| {
        player.camera_pitch = player.camera_pitch.max(1f32.to_radians()).min(179f32.to_radians());
        player.camera_distance = player.camera_distance.max(5.).min(30.);

        *velocity.0.y_mut() -= GRAVITY * time.delta_seconds * 2.;

        let x = player_position.translation.x();
        let y = player_position.translation.y();

        transformlayer.translation.set_y(y + velocity.0.y() * time.delta_seconds);
        player_position.translation.set_x(x + velocity.0.x() + 1.);

        if let Some(camera_entity) = player.camera_entity {
            let cam_pos = Vec3::new(0., player.camera_pitch.cos(), -player.camera_pitch.sin()).normalize() * player.camera_distance;
            if let Ok(mut cam_trans) = camera_query.get_mut(camera_entity) {
                cam_trans.translation = cam_pos;
            }
        }
    });
}

pub fn jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut velocityz: Query<&mut Velocity>,
    players: Query<(Entity, &Player)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Some((entity, _player)) = players.iter().next() {
            let mut velocity = velocityz.get_mut(entity).expect("Could not find player entity");
            // velocity.0.y_mut() -= GRAVITY * time.delta_seconds;
            *velocity.0.y_mut() -= GRAVITY * 3.;
        }
    }
}
