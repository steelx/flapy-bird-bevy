use crate::prelude::*;

pub struct Velocity (pub Vec2);

#[derive(Debug, Default)]
pub struct Damage {
    pub value: u32,
}

pub struct Collider {
    pub width: f32,
    pub height: f32,
}

impl Collider {
    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
}
