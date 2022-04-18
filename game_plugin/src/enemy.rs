use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::{
    na::Rotation2,
    physics::{ColliderBundle, RapierConfiguration, RigidBodyBundle, RigidBodyPositionSync},
    prelude::{
        ActiveEvents, ColliderMaterial, ColliderShape, ColliderType, RigidBodyForces,
        RigidBodyMassProps, RigidBodyMassPropsFlags, RigidBodyPositionComponent, RigidBodyType,
        RigidBodyVelocity,
    },
};
use rand::prelude::random;

use crate::{
    combat::{Bullet, Cooldown, Health, Radian, Weapon, WeaponSlot, WeaponSlots},
    player::Player,
    Lifetime, Owner,
};

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component)]
pub struct Enemy;

#[cfg_attr(feature = "debug", derive(Inspectable))]
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

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Copy, Clone)]
pub enum RotationDir {
    Clockwise,
    CounterClockwise,
}

impl Default for RotationDir {
    fn default() -> Self {
        Self::Clockwise
    }
}

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component)]
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
        rotation_dir: RotationDir,
        current_angle: f32,
    },
}

impl Default for Movement {
    fn default() -> Self {
        Self::Static
    }
}

impl Movement {
    pub fn circle(center: Vec2, radius: f32, rotation_dir: RotationDir) -> Self {
        Self::Circle {
            center,
            radius,
            rotation_dir,
            current_angle: 0.0,
        }
    }
}

pub struct SpawnEnemyEvent;

const COUNT_ENEMIES_LABEL: &str = "count_enemies";

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        app.register_inspectable::<Enemy>()
            .register_inspectable::<Dir>()
            .register_inspectable::<Movement>();
        app.add_event::<SpawnEnemyEvent>()
            .add_system(count_enemies.label(COUNT_ENEMIES_LABEL))
            .add_system(spawn_enemy.after(COUNT_ENEMIES_LABEL))
            .add_system(movement)
            .add_system(enemy_shoot)
            .add_system(test_chase);
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
            body_type: RigidBodyType::KinematicPositionBased.into(),
            position: position.into(),
            mass_properties: RigidBodyMassProps {
                flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        };

        let weapon_entity = commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::splat(8.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Weapon::Laser {
                damage: 1.0,
                cooldown: Cooldown::from_seconds(1.0),
            })
            .insert(Name::new(format!("Laser")))
            .insert(Transform::from_xyz(0.0, -20.0, 0.0))
            .id();

        let weapon_slots = WeaponSlots {
            weapons: vec![WeaponSlot {
                weapon: Some(weapon_entity),
                position: Vec2::new(0.0, -20.0),
                angle: Radian::down(),
            }],
        };

        let movement = if random::<bool>() {
            Movement::Horizontal {
                min: random::<f32>() * 300.0 - 300.0,
                max: random::<f32>() * 300.0,
                current_dir: Dir::Left,
            }
        } else {
            Movement::circle(
                Vec2::new(random::<f32>() * 100.0, random::<f32>() * 100.0),
                random::<f32>() * 100.0 + 10.0,
                RotationDir::Clockwise,
            )
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
            .insert(movement)
            .insert(Name::new("Enemy"))
            .insert(weapon_slots)
            .add_child(weapon_entity);
    };

    for _ in events.iter() {
        spawn();
    }
}

fn test_chase(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    players: Query<Entity, With<Player>>,
    movements: Query<(Entity, &Movement)>,
) {
    if !input.just_pressed(KeyCode::C) {
        return;
    }
    if let Some((player_entity, chasing_entity)) = players.get_single().ok().zip(
        movements
            .iter()
            .find(|(_, m)| !matches!(m, Movement::Chase { .. }))
            .map(|(e, _)| e),
    ) {
        commands.entity(chasing_entity).insert(Movement::Chase {
            target: Some(player_entity),
        });
    }
}

fn movement(
    time: Res<Time>,
    mut enemies: Query<(&mut Movement, &Transform, &mut RigidBodyPositionComponent)>,
    transforms: Query<&Transform>,
) {
    for (mut movement, transform, mut position) in enemies.iter_mut() {
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
                position.next_position.translation = [
                    position.position.translation.x
                        + current_dir.as_f32() * 100.0 * time.delta_seconds(),
                    position.position.translation.y,
                ]
                .into();
            }
            Movement::Chase { target } => {
                if let Some(target_transform) =
                    target.and_then(|target| transforms.get(target).ok())
                {
                    let target_position = target_transform.translation;
                    let dir = Vec2::new(
                        target_position.x - position.position.translation.x,
                        target_position.y - position.position.translation.y,
                    )
                    .normalize();
                    let speed = 40.0;
                    let x = position.position.translation.x + dir.x * time.delta_seconds() * speed;
                    let y = position.position.translation.y + dir.y * time.delta_seconds() * speed;
                    position.next_position = [x, y].into();
                }
            }
            Movement::Circle {
                center,
                ref mut current_angle,
                radius,
                rotation_dir,
            } => {
                let x = center.x + radius * current_angle.cos();
                let y = center.y + radius * current_angle.sin();
                let angular_speed = 1.0;
                let dir = match rotation_dir {
                    RotationDir::Clockwise => 1.0,
                    RotationDir::CounterClockwise => -1.0,
                };
                *current_angle += dir * time.delta_seconds() * angular_speed;
                position.next_position = [x, y].into();
            }
            Movement::Static => {}
        }
    }
}

fn enemy_shoot(
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    enemies: Query<(Entity, &WeaponSlots), With<Enemy>>,
    mut weapons: Query<(&mut Weapon, &GlobalTransform)>,
) {
    for (enemy_entity, slots) in enemies.iter() {
        for weapon_slot in slots.weapons.iter().filter(|slot| slot.weapon.is_some()) {
            if let Ok((mut weapon, &global_transform)) =
                weapons.get_mut(weapon_slot.weapon.unwrap())
            {
                if !weapon.cooldown().0.finished() {
                    continue;
                }
                weapon.cooldown_mut().0.reset();
                let damage = weapon.damage();
                let size = Vec2::new(16.0, 8.0);
                let collider_size = size / rapier_config.scale;
                let bullet_speed = 300.0;
                let bullet_rotation = Rotation2::new(f32::from(weapon_slot.angle));
                let bullet_velocity = bullet_rotation.transform_vector(&[bullet_speed, 0.0].into());
                let rigidbody = RigidBodyBundle {
                    velocity: RigidBodyVelocity {
                        linvel: bullet_velocity,
                        ..Default::default()
                    }
                    .into(),
                    forces: RigidBodyForces {
                        gravity_scale: 0.0,
                        ..Default::default()
                    }
                    .into(),
                    position: (
                        global_transform.translation.truncate(),
                        f32::from(weapon_slot.angle),
                    )
                        .into(),
                    ..Default::default()
                };

                let collider = ColliderBundle {
                    collider_type: ColliderType::Sensor.into(),
                    shape: ColliderShape::cuboid(collider_size.x / 2.0, collider_size.y / 2.0)
                        .into(),
                    flags: (ActiveEvents::INTERSECTION_EVENTS).into(),
                    ..Default::default()
                };
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::BLUE,
                            custom_size: Some(size),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Bullet { damage })
                    .insert(Lifetime {
                        timer: Timer::from_seconds(1.0, false),
                    })
                    .insert_bundle(rigidbody)
                    .insert_bundle(collider)
                    .insert(RigidBodyPositionSync::Discrete)
                    .insert(Owner {
                        entity: enemy_entity,
                    });
            }
        }
    }
}
