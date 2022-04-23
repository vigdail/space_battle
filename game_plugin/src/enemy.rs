use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use heron::prelude::*;
use rand::{prelude::random, seq::SliceRandom};

use crate::{
    combat::{Cooldown, Health, ShootEvent, UnitRaw, WeaponSlot, WeaponSlots},
    despawn_with,
    loading::AssetsFolder,
    player::Player,
    states::GameState,
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
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(count_enemies.label(COUNT_ENEMIES_LABEL))
                    .with_system(spawn_enemy.after(COUNT_ENEMIES_LABEL))
                    .with_system(movement)
                    .with_system(enemy_shoot)
                    .with_system(test_chase),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Countdown).with_system(despawn_with::<Enemy>),
            );
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
    unit_handles: Res<AssetsFolder>,
    units: Res<Assets<UnitRaw>>,
    mut events: EventReader<SpawnEnemyEvent>,
) {
    let mut spawn = || {
        let mut rng = rand::thread_rng();
        let unit_handle = unit_handles.units.choose(&mut rng).unwrap();
        let unit = units.get(unit_handle).unwrap();

        let position = Vec3::new(
            random::<f32>() * 400.0 - 200.0,
            random::<f32>() * 200.0,
            0.0,
        );
        let size = Vec2::splat(32.0);

        let (weapons, slots): (Vec<_>, Vec<_>) = unit
            .weapon_slots
            .iter()
            .map(|slot_def| {
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
                    .insert(slot_def.weapon.clone())
                    .insert(Cooldown::from_seconds(slot_def.weapon.cooldown()))
                    .insert(Name::new("Laser"))
                    .insert(Transform::from_xyz(
                        slot_def.position.x,
                        slot_def.position.y,
                        0.0,
                    ))
                    .id();

                let weapon_slot = WeaponSlot::from_raw(slot_def, Some(weapon_entity));
                (weapon_entity, weapon_slot)
            })
            .unzip();

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
                    color: unit.color.into(),
                    custom_size: Some(size),
                    ..Default::default()
                },
                transform: Transform::from_translation(position)
                    .with_rotation(Quat::from_rotation_z(180.0f32.to_radians())),
                ..Default::default()
            })
            .insert(RigidBody::KinematicPositionBased)
            .insert(CollisionShape::Cuboid {
                half_extends: size.extend(0.0) / 2.0,
                border_radius: None,
            })
            .insert(RotationConstraints::lock())
            .insert(Enemy)
            .insert(movement)
            .insert(Health::new(unit.health))
            .insert(Name::new(unit.name.clone()))
            .insert(WeaponSlots { slots })
            .insert(unit.loot.clone())
            .insert_children(0, &weapons);
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
    mut enemies: Query<(&mut Movement, &mut Transform)>,
    // TODO: use query set, because the target may have Movement component
    transforms: Query<&Transform, Without<Movement>>,
) {
    for (mut movement, mut transform) in enemies.iter_mut() {
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
                transform.translation +=
                    current_dir.as_f32() * 100.0 * time.delta_seconds() * Vec3::X;
            }
            Movement::Chase { target } => {
                if let Some(target_transform) =
                    target.and_then(|target| transforms.get(target).ok())
                {
                    let target_position = target_transform.translation;
                    let dir = (target_position - transform.translation).normalize_or_zero();
                    let speed = 40.0;
                    transform.translation += speed * dir * time.delta_seconds();
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
                transform.translation = Vec3::new(x, y, 0.0);
            }
            Movement::Static => {}
        }
    }
}

fn enemy_shoot(mut shoot_events: EventWriter<ShootEvent>, enemies: Query<Entity, With<Enemy>>) {
    for shooter in enemies.iter() {
        shoot_events.send(ShootEvent { shooter });
    }
}
