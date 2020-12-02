use crate::components::Velocity;
use crate::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Player;

pub fn spawn_player (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("pixels16x16.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 32, 32);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(2.0)),
            draw: Draw {
                is_transparent: true,
                is_visible: true,
                render_commands: Vec::new(),
            },
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(Player)
        .with(Velocity(0.0))
        .with(Animations {
            animations: vec![
                Animation {
                    current_frame: 0,
                    frames: vec![
                        AnimationFrame {
                            index: 17*30+0,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 17*30+1,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 17*30+2,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 17*30+3,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 17*30+4,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 17*30+5,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 17*30+6,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 17*30+7,
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