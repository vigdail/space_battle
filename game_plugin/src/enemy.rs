use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::{
    physics::{ColliderBundle, RapierConfiguration, RigidBodyBundle, RigidBodyPositionSync},
    prelude::{
        ActiveEvents, ColliderMaterial, ColliderShape, RigidBodyMassProps, RigidBodyMassPropsFlags,
        RigidBodyType,
    },
};
use rand::prelude::random;

use crate::combat::Health;

#[derive(Component, Inspectable)]
pub struct Enemy;

pub struct SpawnEnemyEvent;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<Enemy>()
            .add_event::<SpawnEnemyEvent>()
            .add_system(enemy_spawner.label("enemy_count"))
            .add_system(spawn_enemy.after("enemy_count"));
    }
}

fn enemy_spawner(mut events: EventWriter<SpawnEnemyEvent>, enemies: Query<&Enemy>) {
    let enemy_count = enemies.iter().count();
    if enemy_count < 3 {
        for _ in 0..(3 - enemy_count) {
            events.send(SpawnEnemyEvent);
        }
    }
}

fn spawn_enemy(
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    mut events: EventReader<SpawnEnemyEvent>,
) {
    let mut spawn = || {
        let position = Vec2::new(random::<f32>() * 400.0 - 200.0, random::<f32>() * 200.0);
        let size = Vec2::splat(32.0);
        let collider_size = size / rapier_config.scale;
        let collider = ColliderBundle {
            shape: ColliderShape::cuboid(collider_size.x / 2.0, collider_size.y / 2.0).into(),
            material: ColliderMaterial {
                friction: 0.0,
                ..Default::default()
            }
            .into(),
            flags: (ActiveEvents::INTERSECTION_EVENTS).into(),
            ..Default::default()
        };

        let rigidbody = RigidBodyBundle {
            body_type: RigidBodyType::KinematicVelocityBased.into(),
            position: position.into(),
            mass_properties: RigidBodyMassProps {
                flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        };

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::OLIVE,
                    custom_size: Some(size),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert_bundle(collider)
            .insert_bundle(rigidbody)
            .insert(RigidBodyPositionSync::Discrete)
            .insert(Health::new(3.0))
            .insert(Enemy)
            .insert(Name::new("Enemy"));
    };

    for _ in events.iter() {
        spawn();
    }
}
