use crate::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

pub struct ContactsPlugin;

impl Plugin for ContactsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<ContactEvent>()
            .add_system(contact_setup.system());
    }
}

struct ContactEvent {
    to: Entity,
    from: Entity,
    damage: u32,
    collision: Collision,
}

fn contact_setup(
    // collision_events: ResMut<Events<ContactEvent>>,
    player_query: Query<(&Transform, &Collider), With<Player>>,
    collider_query: Query<(&Transform, &Collider, &Damage), Without<Player>>,
) {
    player_query
        .iter()
        .for_each(|(transform, player_collider)| {
            for (collider_transform, collider_sprite, collider_damage) in collider_query.iter() {

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
                    Collision::Left => println!("left collision {}", collider_damage.value),
                    Collision::Right => println!("right collision"),
                    Collision::Top => println!("top collision"),
                    Collision::Bottom => println!("bottom collision"),
                };
            }
        });
}
