use crate::prelude::*;

use super::player::{Player, PlayerCamera};

pub fn gravity_and_move(
    time: Res<Time>,
    mut player_move_timer: ResMut<PlayerMoveTimer>,
    mut player_query: Query<(&mut Player, &mut Velocity, &mut Transform)>,
    mut camera_query: Query<With<PlayerCamera, &mut Transform>>,
) {
    player_move_timer.0.tick(time.delta_seconds);
    if !player_move_timer.0.finished {
        return;
    }

    player_query.iter_mut().for_each(|(mut player, mut velocity, mut player_position)| {
        // player.camera_pitch = player.camera_pitch.max(1f32.to_radians()).min(179f32.to_radians());
        player.camera_distance = player.camera_distance.max(5.).min(30.);

        *velocity.0.y_mut() -= GRAVITY * time.delta_seconds * 2.;

        let x = player_position.translation.x() + 1.;
        let y = player_position.translation.y() + velocity.0.y();

        player_position.translation.set_y(y);
        player_position.translation.set_x(x);

        if let Some(camera_entity) = player.camera_entity {
            // let cam_pos = Vec3::new(x, 0., 0.).normalize() * player.camera_distance;
            if let Ok(mut cam_trans) = camera_query.get_mut(camera_entity) {
                cam_trans.translation.set_x(x);
                cam_trans.translation.set_x(y);
            }
        }
    });
}

pub fn jump(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut velocityz: Query<&mut Velocity>,
    players: Query<(Entity, &Player)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Some((entity, _player)) = players.iter().next() {
            let mut velocity = velocityz.get_mut(entity).expect("Could not find player entity");
            *velocity.0.y_mut() -= GRAVITY * time.delta_seconds;
            // *velocity.0.y_mut() -= GRAVITY * 3.;
        }
    }
}
