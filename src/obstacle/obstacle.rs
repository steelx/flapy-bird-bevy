use crate::prelude::*;
use rand::prelude::ThreadRng;
use rand::Rng;

/// Obstacle
struct Obstacle {
    x: i32,
    y: i32,
    size: i32,//size defines length of the obstacle thru which player can pass
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        Obstacle {
            x,
            y: rng.gen_range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }
}

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
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(SpriteComponents {
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            material: materials.add(Color::hex("22223b").unwrap().into()),
            ..Default::default()
        })
        .with(Obstacle::new(50, 0));
}
