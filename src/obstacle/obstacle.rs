use rand::Rng;
use super::OffsceenDeletion;

use crate::prelude::*;

/// Obstacle
pub struct Obstacle;

pub struct ObstacleSettings {
    // distance from upper and lower pipe, in precentage
    min_pipe_distance: f32,
    max_pipe_distance: f32,
    max_center_delta: f32,
    last_pos: f32,
    last_pos_x: f32,
}

impl ObstacleSettings {
    pub fn new() -> Self {
        Self {
            min_pipe_distance: 200.0,
            max_pipe_distance: 500.0,
            max_center_delta: 0.4,
            last_pos: 0.0,// center pos of pipes, in precentage
            last_pos_x: 0.0,// actual x pixel position
        }
    }
}

pub struct SpawnTimer(pub Timer);

pub fn spawn_obstacle(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut obstacle_settings: ResMut<ObstacleSettings>,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_timer.0.tick(time.delta_seconds);
    if !spawn_timer.0.finished {
        return;
    }

    let mut rng = rand::thread_rng();
    let mut new_center_pos = obstacle_settings.last_pos - rng.gen_range(-obstacle_settings.max_center_delta, obstacle_settings.max_center_delta);

    let window = windows.get_primary().unwrap();
    let win_height = window.height() as f32;
    let win_width = window.width() as f32;

    // This is the extent from the center in Y, a pipe can go maximum, until it flies in the air
    const SCALE: f32 = 3.0;
    let clamp_range = (win_height - (SCALE * 128.0)) / win_height;

    // Clamp func seem to be nightly only for now
    new_center_pos = new_center_pos.min(clamp_range);
    new_center_pos = new_center_pos.max(-clamp_range);

    obstacle_settings.last_pos = new_center_pos;

    // to world units
    new_center_pos *= win_height * 0.5;//half height

    let obstacle_offset_y = (SCALE * 128.0) * 0.5;
    let obstacle_offset_x = (SCALE * 32.0) * 0.5;

    let mut obstacle_delta = rng.gen_range(
        obstacle_settings.min_pipe_distance,
        obstacle_settings.max_pipe_distance,
    );
    // half the size because both pipes will be offseted in opposide direction
    obstacle_delta *= 0.5;
    let x_pos = win_width * 0.5 + obstacle_offset_x;
    obstacle_settings.last_pos_x += x_pos;

    let obstacle_size = Vec2::new(32., 128.);

    //bottom
    commands
        .spawn(SpriteComponents {
            sprite: Sprite::new(obstacle_size),
            material: materials.add(ColorMaterial::from(Color::hex("1a00fa").unwrap())),
            draw: Draw {
                is_transparent: true,
                is_visible: true,
                render_commands: Vec::new(),
            },
            transform: Transform{
                translation: Vec3::new(
                    obstacle_settings.last_pos_x, -obstacle_offset_y + new_center_pos - obstacle_delta, 3.0
                ),
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Obstacle)
        .with(OffsceenDeletion);

    //top
    commands
        .spawn(SpriteComponents {
            sprite: Sprite::new(obstacle_size),
            material: materials.add(ColorMaterial::from(Color::hex("1affff").unwrap())),
            transform: Transform{
                translation: Vec3::new(
                    obstacle_settings.last_pos_x, obstacle_offset_y + new_center_pos + obstacle_delta, 3.0
                ),
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Obstacle)
        .with(OffsceenDeletion);
}
