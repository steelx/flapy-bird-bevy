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
use std::collections::HashMap;

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
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dComponents::default());

    let texture_handle = asset_server.load("pixels16x16.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 32, 32);
    //ColorMaterialStorage need to fix texture_atlas to able to insert to a HashMap
    let mut material_storage = ColorMaterialStorage{ materials: HashMap::new(), texture_atlas: texture_atlases.add(texture_atlas) };
    material_storage.materials.insert(
        "wall_color".to_string(),
        materials.add(Color::hex("ffe8d6").unwrap().into()),
    );

    commands.insert_resource(material_storage);
}