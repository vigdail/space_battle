use bevy::prelude::*;

use crate::components::Velocity;

pub fn moving_system(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}
