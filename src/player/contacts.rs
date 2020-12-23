use crate::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

pub struct ContactsPlugin;

impl Plugin for ContactsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<ContactEvent>()
            .add_system(contact_setup.system())
            .add_system(player_collider_reader.system());
    }
}

pub struct ContactEvent {
    to: Entity,
    from: Entity,
    damage: u32,
    collision: Collision,
}

fn contact_setup(
    mut contact_events: ResMut<Events<ContactEvent>>,
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
                    Collision::Left => contact_events.send(ContactEvent {
                        to: player_entity,
                        from: collider_entity,
                        damage: collider_damage.value,
                        collision,
                    }),
                    Collision::Right => println!("right collision"),
                    Collision::Top => println!("top collision"),
                    Collision::Bottom => println!("bottom collision"),
                };
            }
        });
}

fn player_collider_reader(
    events: Res<Events<ContactEvent>>,
    mut reader: Local<EventReader<ContactEvent>>,
) {
    for (contact_event) in reader.iter(&events) {
        println!("player collided with an Obstacle");
    }
}
