use crate::prelude::*;

use super::player::Player;
use bevy_rapier2d::physics::RigidBodyHandleComponent;
use bevy_rapier2d::rapier::dynamics::RigidBodySet;
use bevy_rapier2d::na::Vector2;

pub fn jump_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut runstate: ResMut<RunState>,
    mut bodies: ResMut<RigidBodySet>,
    mut query: Query<(&RigidBodyHandleComponent, Mut<Player>)>,
) {
    let player_entity = runstate.player.unwrap();
    let mut rotation = 0;
    let mut thrust = 0;

    if keyboard_input.pressed(KeyCode::Space) {
        thrust += 1
    }
    if keyboard_input.pressed(KeyCode::A) {
        rotation += 1
    }
    if keyboard_input.pressed(KeyCode::D) {
        rotation -= 1
    }

    if let Ok(body_handle) = query.get_component::<RigidBodyHandleComponent>(player_entity) {
        let body = bodies.get_mut(body_handle.handle()).unwrap();
        let player = query.get_component::<Player>(player_entity).unwrap();
        if rotation != 0 {
            let rotation = rotation as f32 * player.rotation_speed;
            body.apply_torque_impulse(rotation, true);
        }
        if thrust != 0 {
            let force = body.position().rotation.transform_vector(&Vector2::y())
                * thrust as f32
                * player.thrust;
            body.apply_force(force, true);
        }
    }
}
