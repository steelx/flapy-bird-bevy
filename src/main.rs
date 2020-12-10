mod state;
mod animations;
mod player;
mod obstacle;
mod despawn;

pub mod prelude {
    pub use bevy::prelude::*;
    pub const GRAVITY: f32 = 75.0;
    pub use crate::animations::*;
    pub use crate::player::*;
    pub use crate::obstacle::*;
    pub use crate::state::*;
}

use prelude::*;
use std::collections::HashMap;
use bevy_rapier2d::physics::{RapierPhysicsPlugin, RapierConfiguration};
use bevy_rapier2d::na::Vector2;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Flappy Bevy!".to_string(),
            width: 800,
            height: 600,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_resource(RapierConfiguration {
            gravity: Vector2::zeros(),
            ..Default::default()
        })
        .add_plugin(AnimationPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ObstaclePlugin)
        .add_startup_system(setup_system)
        .add_system(despawn::offscreen_deletion_system)
        .run();
}

fn setup_system(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    let texture_handle = asset_server.load("pixels16x16.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 32, 32);
    //ColorMaterialStorage need to fix texture_atlas to able to insert to a HashMap
    let mut material_storage = RunState {
        materials: HashMap::new(),
        texture_atlas: texture_atlases.add(texture_atlas),
        player: None,
    };
    material_storage.materials.insert(
        "wall_color".to_string(),
        materials.add(Color::hex("ffe8d6").unwrap().into()),
    );

    commands.insert_resource(material_storage);
}