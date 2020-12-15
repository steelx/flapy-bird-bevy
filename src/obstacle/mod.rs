use crate::prelude::*;
mod obstacle;

use obstacle::*;

pub struct ObstaclePlugin;
pub struct OffsceenDeletion;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(SpawnTimer(Timer::from_seconds(1., true)))
            .add_resource(ObstacleSettings::new())
            .add_event::<ObstacleSpawnEvent>()
            .add_system(setup_obstacle_system)
            .add_system(spawn_obstacle_system);
    }
}

pub struct ObstacleSpawnEvent {
    pub size: Vec2,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}