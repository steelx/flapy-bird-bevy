use crate::prelude::*;

use super::player::Player;

pub fn gravity_and_move(
    time: Res<Time>,
    mut query: Query<With<Player, (&mut Velocity, &mut Transform)>>,
) {
    query.iter_mut().for_each(|(mut velocity, mut transform)| {
        velocity.0 -= GRAVITY * time.delta_seconds * 2.;

        let x = transform.translation.x();
        let y = transform.translation.y();

        transform.translation.set_y(y + velocity.0 * time.delta_seconds);
        transform.translation.set_x(x + 1.);
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
            velocity.0 = GRAVITY * 3.;
        }
    }
}
