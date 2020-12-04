// use rand::{Rng, thread_rng};

use crate::prelude::*;

/// Obstacle
pub struct Obstacle;

pub struct SpawnTimer(pub Timer);

// pub fn hit_obstacle(obstacle: Query<(&Obstacle, &Transform)>, player_q: Query<With<Player, &Transform>>) -> bool {
//     let player = player_q.iter().next().unwrap();
//     let half_size = self.size / 2;
//     let does_x_match = self.x == player.x;
//     let player_above_gap = player.y < self.y - half_size;
//     let player_below_gap = player.y > self.y + half_size;
//     does_x_match && (player_above_gap || player_below_gap)
// }

pub fn spawn_obstacle(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_timer.0.tick(time.delta_seconds);
    if !spawn_timer.0.finished {
        return;
    }

    let window = windows.get_primary().unwrap();
    // let mut rng = rand::thread_rng();

    let score = time.delta_seconds;
    let half_width = window.width() as f32 * 0.5;
    let half_height = window.height() as f32 * 0.5;
    let x: f32 = half_width;
    // let height: f32 = rng.gen_range(10., 40.);
    let gap_size = f32::max(100., half_height - score);


    //bottom
    commands
        .spawn(SpriteComponents {
            sprite: Sprite::new(Vec2::new(20.0, half_height)),
            material: materials.add(ColorMaterial::from(Color::hex("1affff").unwrap())),
            transform: Transform{
                translation: Vec3::new(
                    x,
                    -(half_height + gap_size),
                    0.2,
                ),
                scale: Vec3::splat(3.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Obstacle);

    //top
    commands
        .spawn(SpriteComponents {
            sprite: Sprite::new(Vec2::new(20.0, half_height)),
            material: materials.add(ColorMaterial::from(Color::hex("1affff").unwrap())),
            transform: Transform{
                translation: Vec3::new(
                    x,
                    half_height + gap_size,
                    0.2,
                ),
                scale: Vec3::splat(3.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Obstacle);
}
