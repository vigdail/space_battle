use bevy::prelude::*;
use heron::{prelude::*, SensorShape};

use crate::{player::Player, Lifetime, Owner};

use super::{
    components::{Bullet, Cooldown, Health, Loot, Scores, Weapon, WeaponSlots},
    events::{ContactEvent, RewardEvent, ShootEvent, SpawnBulletEvent},
    EquipWeaponEvent,
};

pub fn handle_intersections(
    mut collision_events: EventReader<CollisionEvent>,
    bullets: Query<&Bullet>,
    healths: Query<&Health>,
    owners: Query<&Owner>,
    mut contact_events: EventWriter<ContactEvent>,
) {
    for (data1, data2) in collision_events.iter().filter_map(|e| match e {
        CollisionEvent::Started(data1, data2) => Some((data1, data2)),
        _ => None,
    }) {
        let entity1 = data1.rigid_body_entity();
        let entity2 = data2.rigid_body_entity();

        let mut check_bullet = |health_entity, bullet_entity| {
            if healths.get(health_entity).is_ok() && bullets.get(bullet_entity).is_ok() {
                let is_owner = owners
                    .get(bullet_entity)
                    .map(|owner| owner.entity == health_entity)
                    .unwrap_or(false);

                if !is_owner {
                    contact_events.send(ContactEvent::HealthBullet(health_entity, bullet_entity));
                }
            }
        };

        check_bullet(entity1, entity2);
        check_bullet(entity2, entity1);
    }
}

pub fn handle_contacts(
    mut commands: Commands,
    mut contact_events: EventReader<ContactEvent>,
    mut healths: Query<&mut Health>,
    bullets: Query<&Bullet>,
) {
    for event in contact_events.iter() {
        match *event {
            ContactEvent::HealthBullet(health_entity, bullet_entity) => {
                if let Some((mut health, bullet)) = healths
                    .get_mut(health_entity)
                    .ok()
                    .zip(bullets.get(bullet_entity).ok())
                {
                    health.current = health.current.saturating_sub(bullet.damage);
                }
                commands.entity(bullet_entity).despawn();
            }
        }
    }
}

pub fn despawn_dead(
    mut commands: Commands,
    mut reward_events: EventWriter<RewardEvent>,
    healths: Query<(Entity, &Health, Option<&Loot>), Changed<Health>>,
) {
    for (entity, health, loot) in healths.iter() {
        if health.is_dead() {
            if let Some(loot) = loot {
                reward_events.send(RewardEvent { score: loot.score });
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn update_cooldowns(time: Res<Time>, mut cooldowns: Query<&mut Cooldown>) {
    for mut cooldown in cooldowns.iter_mut() {
        cooldown.0.tick(time.delta());
    }
}

pub fn test_equip_weapon(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut events: EventWriter<EquipWeaponEvent>,
    players: Query<(Entity, &WeaponSlots), With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::E) {
        for (entity, slots) in players.iter() {
            let slot_index = slots.slots.iter().position(|slot| slot.weapon.is_none());

            if let Some(slot_index) = slot_index {
                let weapon = Weapon::Laser {
                    damage: 1,
                    cooldown: 0.2,
                };
                let weapon_entity = commands
                    .spawn()
                    .insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::GREEN,
                            custom_size: Some(Vec2::splat(8.0)),
                            ..default()
                        },
                        visibility: Visibility { is_visible: false },
                        ..default()
                    })
                    .insert(weapon.clone())
                    .insert(Cooldown::from_seconds(weapon.cooldown()))
                    .insert(Name::new(format!("Weapon {}", slot_index)))
                    .id();

                events.send(EquipWeaponEvent {
                    entity,
                    weapon_entity,
                    slot_index,
                });
            }
        }
    }
}

pub fn equip_weapon(
    mut commands: Commands,
    mut events: EventReader<EquipWeaponEvent>,
    mut slots: Query<&mut WeaponSlots>,
) {
    for event in events.iter() {
        if let Ok(mut slots) = slots.get_mut(event.entity) {
            if let Some(slot) = slots.slots.get_mut(event.slot_index) {
                slot.weapon = Some(event.weapon_entity);
                commands
                    .entity(event.weapon_entity)
                    .insert(Transform::from_xyz(
                        slot.transform.translation.x,
                        slot.transform.translation.y,
                        0.0,
                    ))
                    .insert(Visibility { is_visible: true });
                commands.entity(event.entity).add_child(event.weapon_entity);
            }
        }
    }
}

pub fn handle_shoot_events(
    mut shoot_events: EventReader<ShootEvent>,
    mut spawn_bullet_events: EventWriter<SpawnBulletEvent>,
    weapon_slots: Query<&WeaponSlots>,
    weapons: Query<(&Weapon, &Cooldown)>,
) {
    for ShootEvent { shooter } in shoot_events.iter() {
        if let Ok(slots) = weapon_slots.get(*shooter) {
            for weapon_slot in slots.slots.iter().filter(|slot| {
                slot.weapon
                    .and_then(|weapon| {
                        weapons
                            .get(weapon)
                            .ok()
                            .filter(|(_, cooldown)| cooldown.0.finished())
                    })
                    .is_some()
            }) {
                spawn_bullet_events.send(SpawnBulletEvent {
                    weapon_slot: weapon_slot.clone(),
                    shooter: *shooter,
                });
            }
        }
    }
}

pub fn spawn_bullets(
    mut commands: Commands,
    mut events: EventReader<SpawnBulletEvent>,
    mut weapons: Query<(&Weapon, &mut Cooldown, &GlobalTransform)>,
) {
    for SpawnBulletEvent {
        weapon_slot,
        shooter,
    } in events.iter()
    {
        if let Some((weapon, mut cooldown, global_transform)) = weapon_slot
            .weapon
            .and_then(|weapon_entity| weapons.get_mut(weapon_entity).ok())
        {
            cooldown.0.reset();
            let damage = weapon.damage();
            let size = Vec2::new(16.0, 8.0);
            let bullet_speed = 300.0;
            let bullet_rotation = weapon_slot.transform.rotation;
            let bullet_velocity = bullet_rotation.mul_vec3(Vec3::X * bullet_speed);

            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLUE,
                        custom_size: Some(size),
                        ..default()
                    },
                    transform: Transform::from_translation(global_transform.translation)
                        .with_rotation(bullet_rotation),
                    ..default()
                })
                .insert(Bullet { damage })
                .insert(Lifetime {
                    timer: Timer::from_seconds(1.0, false),
                })
                .insert(RigidBody::KinematicVelocityBased)
                .insert(SensorShape)
                .insert(CollisionShape::Cuboid {
                    half_extends: size.extend(0.0) / 2.0,
                    border_radius: None,
                })
                .insert(Velocity::from_linear(bullet_velocity))
                .insert(Owner { entity: *shooter });
        }
    }
}

pub fn apply_score_reward(
    mut reward_events: EventReader<RewardEvent>,
    mut players: Query<&mut Scores, With<Player>>,
) {
    for RewardEvent { score } in reward_events.iter() {
        for mut scores in players.iter_mut() {
            scores.amount += score;
        }
    }
}
