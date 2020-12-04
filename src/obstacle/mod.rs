use crate::prelude::*;
mod obstacle;

use obstacle::*;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(spawn_obstacle.system())
            .add_resource(SpawnTimer(Timer::from_seconds(2., true)));
    }
}