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
            .add_system(spawn_obstacle.system());
    }
}