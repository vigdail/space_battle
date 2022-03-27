use bevy::prelude::*;

use crate::{
    components::{Player, Velocity},
    resources::InputAxis,
};

pub fn moving_system(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

pub fn input_system(keys: Res<Input<KeyCode>>, mut axis: ResMut<InputAxis>) {
    axis.vertical = 0.0;
    axis.horizontal = 0.0;
    if keys.pressed(KeyCode::W) {
        axis.vertical -= 1.0;
    }
    if keys.pressed(KeyCode::S) {
        axis.vertical += 1.0;
    }
    if keys.pressed(KeyCode::A) {
        axis.horizontal -= 1.0;
    }
    if keys.pressed(KeyCode::D) {
        axis.horizontal += 1.0;
    }
}

pub fn player_move_system(
    mut commands: Commands,
    mut players: Query<Entity, With<Player>>,
    axis: ResMut<InputAxis>,
) {
    let velocity = Vec2::new(axis.horizontal, -axis.vertical);
    let velocity = if velocity.length_squared() > 0.0 {
        velocity.normalize() * 50.0
    } else {
        velocity
    };
    for player in players.iter_mut() {
        commands
            .entity(player)
            .insert(Velocity(velocity.extend(0.0)));
    }
}
