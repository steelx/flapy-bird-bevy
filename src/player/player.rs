use crate::components::Velocity;
use crate::prelude::*;
use bevy_rapier2d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier2d::rapier::geometry::ColliderBuilder;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Player;

pub fn spawn_player_system (
    commands: &mut Commands,
    material_storage: Res<ColorMaterialStorage>,
) {
    let player_entity = commands
        .spawn(SpriteSheetBundle {
            texture_atlas: material_storage.texture_atlas.clone(),
            transform: Transform::from_scale(Vec3::splat(2.0)),
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
        })
        .current_entity().unwrap();

    let body = RigidBodyBuilder::new_dynamic().user_data(player_entity.to_bits() as u128);
    let collider = ColliderBuilder::ball(1.0);

    commands.insert(player_entity, (body, collider));
}