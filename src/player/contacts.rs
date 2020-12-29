use crate::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

#[derive(Debug)]
pub struct ContactEvent {
    to: Entity,
    from: Entity,
    damage: u32,
    collision: Collision,
}

#[derive(Debug)]
pub struct ShouldMoveEvent {
    pub denied_direction: Vec2,
}

pub fn contact_setup(
    mut contact_events: ResMut<Events<ContactEvent>>,
    mut should_move_events: ResMut<Events<ShouldMoveEvent>>,
    player_query: Query<(&Transform, &Collider, Entity), With<Player>>,
    collider_query: Query<(&Transform, &Collider, &Damage, Entity), Without<Player>>,
) {
    player_query
        .iter()
        .for_each(|(transform, player_collider, player_entity)| {
            for (collider_transform, collider_sprite, collider_damage, collider_entity) in collider_query.iter() {

                let collision = collide(
                    transform.translation,
                    player_collider.as_vec2(),
                    collider_transform.translation,
                    collider_sprite.as_vec2(),
                );

                let collision = match collision {
                    Some(collision) => collision,
                    None => continue,
                };

                //let (reflect_x, reflect_y) =
                match collision {
                    Collision::Left => {
                        contact_events.send(ContactEvent {
                            to: player_entity,
                            from: collider_entity,
                            damage: collider_damage.value,
                            collision,
                        });
                        should_move_events.send(ShouldMoveEvent {
                            denied_direction: Vec2::new(-100., 0.)
                        });
                    },
                    Collision::Right => should_move_events.send(ShouldMoveEvent {
                        denied_direction: Vec2::new(20., 0.)
                    }),
                    Collision::Top => should_move_events.send(ShouldMoveEvent {
                        denied_direction: Vec2::new(0., 20.)
                    }),
                    Collision::Bottom => should_move_events.send(ShouldMoveEvent {
                        denied_direction: Vec2::new(0., -20.)
                    }),
                };
            }
        });
}

pub fn player_collider_reader(
    events: Res<Events<ContactEvent>>,
    mut reader: Local<EventReader<ContactEvent>>,
    mut player_query: Query<(&mut Player)>,
) {
    for (contact_event) in reader.iter(&events) {
        let mut player = player_query.get_mut(contact_event.to).unwrap();
        if player.life == 0 {
            continue;
        }
        player.life -= contact_event.damage;
        dbg!("player collided with an Obstacle {:?}", player);
    }
}
