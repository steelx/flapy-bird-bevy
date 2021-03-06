use crate::prelude::*;

use crate::obstacle::OffsceenDeletion;
use crate::player::Player;

pub fn offscreen_deletion_system (
    commands: &mut Commands,
    windows: Res<Windows>,
    mut obstacle_query: Query<(Entity, &mut Transform), With<OffsceenDeletion>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    // check Camera2dComponents VisibleEntities ?
    let window = windows.get_primary().unwrap();
    let padding = window.width() as f32;

    let last_player_pos = player_query.iter_mut().next().unwrap();

    obstacle_query.iter_mut().for_each(|(entity, transform)| {
        if transform.translation.x + padding < last_player_pos.translation.x {
            commands.despawn(entity);
        }
    });
}
