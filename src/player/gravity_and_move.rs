use crate::prelude::*;

pub fn gravity_and_move(time: Res<Time>, mut query: Query<With<Player, (&mut Velocity, &mut Transform)>>) {

    query.iter_mut().for_each(|(mut velocity, mut transform)| {
        velocity.0 -= GRAVITY * time.delta_seconds * 2.;

        let x = transform.translation.x();
        let y = transform.translation.y();

        transform.translation.set_y(y + velocity.0 * time.delta_seconds);
        transform.translation.set_x(x);
    });
}

pub fn flap(keyboard_input: Res<Input<KeyCode>>, mut query: Query<Mut<Velocity>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Some(mut velocity) = query.iter_mut().next() {
            velocity.0 = GRAVITY * 4.;
        }
    }
}
