use crate::prelude::*;

use super::player::Player;

pub fn gravity_and_move(
    time: Res<Time>,
    mut player_query: Query<With<Player, (&mut Velocity, &mut Transform)>>,
) {
    player_query.iter_mut().for_each(|(mut velocity, mut player_position)| {
        *velocity.0.y_mut() -= GRAVITY * time.delta_seconds * 2.75;

        let speed: f32 = 1.25;
        let x = player_position.translation.x() + speed;
        let y = player_position.translation.y() + velocity.0.y() * time.delta_seconds;

        player_position.translation.set_y(y);
        player_position.translation.set_x(x);
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
            // *velocity.0.y_mut() -= GRAVITY * time.delta_seconds;
            velocity.0.set_y(GRAVITY*4.);
        }
    }
}
