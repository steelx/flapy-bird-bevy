use crate::prelude::*;
use std::collections::HashMap;

pub struct Velocity (pub Vec2);

#[derive(Debug, Default)]
pub struct ColorMaterialStorage {
    pub materials: HashMap<String, Handle<ColorMaterial>>,
    pub texture_atlas: Handle<TextureAtlas>,
}
