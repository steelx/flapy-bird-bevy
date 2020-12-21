use crate::components::Velocity;
use crate::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Player {
    pub rotation_speed: f32,
    pub thrust: f32,
    pub life: u32,
}

pub fn spawn_player_system (
    commands: &mut Commands,
    mut runstate: ResMut<RunState>,
) {
    let player_entity = commands
        .spawn(SpriteSheetBundle {
            texture_atlas: runstate.texture_atlas.clone(),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))//animation timer
        .with(Player {
            life: 100,
            rotation_speed: 0.1,
            thrust: 60.0,
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
        .current_entity().unwrap();

    // commands.insert(player_entity, ()); //can be used to bind with something
    runstate.player = Some(player_entity);
}
