use crate::components::Velocity;
use crate::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Player;

pub fn spawn_player (
    mut commands: Commands,
    material_storage: Res<ColorMaterialStorage>,
) {
    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: material_storage.texture_atlas.clone(),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            draw: Draw {
                is_transparent: true,
                is_visible: true,
                render_commands: Vec::new(),
            },
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))//animation timer
        .with(Player)
        .with(Velocity(Vec2::zero()))
        .with(Animations {
            animations: vec![
                Animation {
                    current_frame: 0,
                    frames: vec![
                        AnimationFrame {
                            index: 17*32+0,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 17*32+1,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 17*32+2,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 17*32+3,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 17*32+4,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 17*32+5,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 17*32+6,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 17*32+7,
                            time: 0.1,
                        },
                    ],
                },
                Animation {
                    current_frame: 0,
                    frames: vec![
                        AnimationFrame {
                            index: 20,
                            time: 0.2,
                        },
                        AnimationFrame {
                            index: 21,
                            time: 0.1,
                        },
                    ],
                },
            ],
            current_animation: 0,
        });
}