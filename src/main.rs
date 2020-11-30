mod components;
mod animations;
mod player;

pub mod prelude {
    pub use bevy::prelude::*;
    pub use crate::animations::*;
    pub use crate::player::*;
}

use prelude::*;
use bevy::app::startup_stage;
use std::time::Duration;

fn main() {
    App::build()
        .add_resource(ClearColor(Color::hex("003049").unwrap()))
        .add_resource(WindowDescriptor {
            title: "Flappy Bevy!".to_string(),
            width: 800,
            height: 600,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationPlugin)
        .add_startup_system(setup.system())
        .add_startup_system_to_stage(startup_stage::POST_STARTUP, spawn_player.system())
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dComponents::default());
}