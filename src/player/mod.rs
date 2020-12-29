mod player;
mod gravity;
mod camera;
pub mod contacts;

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
            .add_event::<ContactEvent>()
            .add_event::<ShouldMoveEvent>()
            .add_system(contact_setup.system())
            .add_system(player_collider_reader.system())
            .add_startup_system_to_stage(startup_stage::POST_STARTUP, spawn_player_system.system())
            .add_system(gravity_and_move_system.system())
            .add_system(input_system.system());
    }
}
