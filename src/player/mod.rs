mod player;
mod gravity;

use crate::prelude::*;
use bevy::app::startup_stage;
use player::*;
use gravity::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system_to_stage(startup_stage::POST_STARTUP, spawn_player.system())
            .add_system(gravity_and_move.system())
            .add_system(flap.system());
    }
}