use crate::prelude::*;

pub struct AnimationPlugin;

pub struct AnimationFrame {
    pub index: i32,
    pub time: f32,
}

pub struct Animation {
    pub frames: Vec<AnimationFrame>,
    pub current_frame: i32,
}

pub struct Animations {
    pub animations: Vec<Animation>,
    pub current_animation: i32,
}

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(animate_system.system());
    }
}

fn animate_system(
    timer: Res<TickTimer>,
    mut query: Query<(
        &mut TextureAtlasSprite,
        &mut Animations,
    )>,
) {
    query
        .iter_mut()
        .for_each(|(mut sprite, mut animations)| {
            if timer.0.finished {
                let current_animation_index = animations.current_animation;
                match animations.animations.get_mut(current_animation_index as usize) {
                    Some(animation) => {
                        animation.current_frame += 1;
                        if animation.current_frame as usize >= animation.frames.len() {
                            animation.current_frame = 0;
                        }

                        if let Some(frame) = animation.frames.get(animation.current_frame as usize) {
                            sprite.index = frame.index as u32;
                        }
                    }
                    None => {}
                }
            }
        });
}
