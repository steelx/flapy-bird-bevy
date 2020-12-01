use crate::components::Velocity;
use crate::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Player;

pub fn spawn_player (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("blue_ghost_50x50.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 50.0), 5, 5);
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
                            index: 0,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 1,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 2,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 3,
                            time: 0.1,
                        },
                        AnimationFrame {
                            index: 4,
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