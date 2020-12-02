mod components;
mod animations;
mod player;
mod obstacle;

pub mod prelude {
    pub use bevy::prelude::*;
    pub const GRAVITY: f32 = 40.0;
    pub use crate::animations::*;
    pub use crate::player::*;
    pub use crate::obstacle::*;
    pub use crate::components::*;
}

use prelude::*;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Flappy Bevy!".to_string(),
            width: 800,
            height: 600,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ObstaclePlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dComponents::default());
}