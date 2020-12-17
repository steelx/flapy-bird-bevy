use crate::prelude::*;

use super::player::Player;
use crate::state::RunState;

pub fn gravity_and_move_system(
    time: Res<Time>,
    mut player_query: Query<(&mut Velocity, &mut Transform), With<Player>>,
) {
    player_query.iter_mut().for_each(|(mut velocity, mut player_position)| {
        let delta = time.delta_seconds();
        velocity.0.y -= GRAVITY * delta * 2.75_f32;

        let x = player_position.translation.x + velocity.0.x * delta;
        let y = player_position.translation.y + velocity.0.y * delta;

        player_position.translation.y = y;
        player_position.translation.x = x;
    });
}

pub fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    runstate: Res<RunState>,
    mut velocityz: Query<&mut Velocity>,
    query: Query<Mut<Player>>,
) {
    let player_entity = runstate.player.unwrap();

    let mut thrust = 0;

    if keyboard_input.pressed(KeyCode::Space) {
        thrust += 1
    }
    // if keyboard_input.pressed(KeyCode::A) {
    //     rotation += 1
    // }
    // if keyboard_input.pressed(KeyCode::D) {
    //     rotation -= 1
    // }

    if thrust != 0 {
        let player = query.get_component::<Player>(player_entity).unwrap();
        let mut velocity = velocityz.get_mut(player_entity).expect("Could not find player Velocity");
        velocity.0.y = GRAVITY * thrust as f32;
        velocity.0.x = player.thrust;
    }
}
