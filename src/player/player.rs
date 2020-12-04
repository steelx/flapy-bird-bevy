use crate::components::Velocity;
use crate::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Player {
    pub camera_distance: f32,
    pub camera_pitch: f32,
    pub camera_entity: Option<Entity>,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            camera_distance: 20.,
            camera_pitch: 30.0f32.to_radians(),
            camera_entity: None,
        }
    }
}

pub struct PlayerCamera;

pub fn spawn_player (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("pixels16x16.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 32, 32);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Spawn camera and player, set entity for camera on player.
    let camera_entity = commands
        .spawn(Camera2dComponents::default())
        .with(PlayerCamera)
        .current_entity();

    let player_entity = commands
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
        .with(Timer::from_seconds(0.1, true))//animation timer
        .with(Player {
            camera_entity,
            ..Default::default()
        })
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
        })
        .current_entity();

    commands
        // Append camera to player as child.
        .push_children(player_entity.unwrap(), &[camera_entity.unwrap()]);
}