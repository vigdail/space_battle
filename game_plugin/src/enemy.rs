use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::{
    physics::{ColliderBundle, RapierConfiguration, RigidBodyBundle, RigidBodyPositionSync},
    prelude::{
        ActiveEvents, ColliderMaterial, ColliderShape, RigidBodyMassProps, RigidBodyMassPropsFlags,
        RigidBodyType, RigidBodyVelocityComponent,
    },
};
use rand::prelude::random;

use crate::{combat::Health, player::Player};

#[derive(Component, Inspectable)]
pub struct Enemy;

#[derive(Inspectable)]
pub enum Dir {
    Left,
    Right,
}

impl Dir {
    pub fn as_f32(&self) -> f32 {
        match self {
            Dir::Left => -1.0,
            Dir::Right => 1.0,
        }
    }
}

impl Default for Dir {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Component, Inspectable)]
pub enum Movement {
    Static,
    Horizontal {
        min: f32,
        max: f32,
        current_dir: Dir,
    },
    Chase {
        target: Option<Entity>,
    },
    Circle {
        center: Vec2,
        radius: f32,
        current_angle: f32,
    },
}

impl Default for Movement {
    fn default() -> Self {
        Self::Static
    }
}

pub struct SpawnEnemyEvent;

const COUNT_ENEMIES_LABEL: &str = "count_enemies";

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<Enemy>()
            .register_inspectable::<Dir>()
            .register_inspectable::<Movement>()
            .add_event::<SpawnEnemyEvent>()
            .add_system(count_enemies.label(COUNT_ENEMIES_LABEL))
            .add_system(spawn_enemy.after(COUNT_ENEMIES_LABEL))
            .add_system(movement);
    }
}

fn count_enemies(mut events: EventWriter<SpawnEnemyEvent>, enemies: Query<&Enemy>) {
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
            .insert(Movement::Horizontal {
                min: -random::<f32>() * 350.0,
                max: random::<f32>() * 350.0,
                current_dir: Dir::Left,
            })
            .insert(Name::new("Enemy"));
    };

    for _ in events.iter() {
        spawn();
    }
}

fn movement(
    mut enemies: Query<
        (&mut Movement, &Transform, &mut RigidBodyVelocityComponent),
        Without<Player>,
    >,
) {
    for (mut movement, transform, mut velocity) in enemies.iter_mut() {
        match *movement {
            Movement::Horizontal {
                min,
                max,
                ref mut current_dir,
            } => {
                if transform.translation.x <= min {
                    *current_dir = Dir::Right;
                } else if transform.translation.x >= max {
                    *current_dir = Dir::Left;
                }
                velocity.linvel.x = current_dir.as_f32() * 100.0;
            }
            Movement::Chase { .. } => todo!(),
            Movement::Circle { .. } => todo!(),
            Movement::Static => {}
        }
    }
}
