mod player;
mod gravity;
mod camera;

use crate::prelude::*;
use bevy::app::startup_stage;
use player::*;
use gravity::*;
use camera::*;

pub struct PlayerPlugin;
pub use player::Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(CameraPlugin)
            .add_startup_system_to_stage(startup_stage::POST_STARTUP, spawn_player.system())
            .add_system(gravity_and_move.system())
            .add_system(jump.system());
    }
}
