use crate::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct RunState {
    pub player: Option<Entity>,
    pub materials: HashMap<String, Handle<ColorMaterial>>,
    pub texture_atlas: Handle<TextureAtlas>,
}
