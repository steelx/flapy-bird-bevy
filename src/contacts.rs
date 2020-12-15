use crate::prelude::*;
use bevy_rapier2d::physics::EventQueue;
use bevy_rapier2d::rapier::dynamics::RigidBodySet;
use bevy_rapier2d::rapier::geometry::ContactEvent;
use crate::components::Damage;

enum Contacts {
    PlayerObstacle(Entity, Entity),
    PlayerOutbounds(Entity),
}

pub fn contacts_system(
    events: Res<EventQueue>,
    bodies: ResMut<RigidBodySet>,
    damages: Query<&Damage>,
    mut player_query: Query<Mut<Player>>,
) {
    let mut contacts = vec![];

    while let Ok(contact_event) = events.contact_events.pop() {
        match contact_event {
            ContactEvent::Started(h1, h2) => {
                let b1 = bodies.get(h1).unwrap();
                let b2 = bodies.get(h2).unwrap();
                let e1 = Entity::from_bits(b1.user_data as u64);
                let e2 = Entity::from_bits(b2.user_data as u64);

                if player_query.get_component::<Player>(e1).is_ok() && damages.get_component::<Damage>(e2).is_ok() {
                    contacts.push(Contacts::PlayerObstacle(e1, e2));
                } else if player_query.get_component::<Player>(e2).is_ok() && damages.get_component::<Damage>(e1).is_ok() {
                    contacts.push(Contacts::PlayerObstacle(e2, e1));
                }
            }
            _ => (),
        }
    }

    contacts
        .iter()
        .for_each(|contact| {
            match contact {
                Contacts::PlayerObstacle(_e1, _e2) => {
                    println!("player hit !!");
                },
                _ => (),
            }
        });
}