use bevy::prelude::*;
use heron::{prelude::*, SensorShape};

use crate::{player::Player, Lifetime, Owner};

use super::{
    components::{Bullet, Cooldown, Health, Loot, Scores, Weapon},
    events::{ContactEvent, RewardEvent, ShootEvent, SpawnBulletEvent},
    EquipWeaponEvent, WeaponSlot,
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
    keyboard_input: Res<Input<KeyCode>>,
    mut events: EventWriter<EquipWeaponEvent>,
    players: Query<&Children, With<Player>>,
    free_slots: Query<Entity, (With<WeaponSlot>, Without<Weapon>)>,
) {
    if keyboard_input.just_pressed(KeyCode::E) {
        for children in players.iter() {
            if let Some(slot_entity) = children
                .iter()
                .find_map(|child| free_slots.get(*child).ok())
            {
                events.send(EquipWeaponEvent {
                    slot_entity,
                    weapon: Weapon::Laser {
                        damage: 1,
                        cooldown: 0.2,
                    },
                });
            }
        }
    }
}

pub fn equip_weapon(
    mut commands: Commands,
    mut events: EventReader<EquipWeaponEvent>,
    transforms: Query<&Transform>,
) {
    for event in events.iter() {
        let transform = transforms
            .get(event.slot_entity)
            .cloned()
            .unwrap_or_default();
        commands
            .entity(event.slot_entity)
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::splat(8.0)),
                    ..default()
                },
                transform,
                ..default()
            })
            .insert(event.weapon.clone())
            .insert(Cooldown::from_seconds(event.weapon.cooldown()));
    }
}

pub fn handle_shoot_events(
    mut shoot_events: EventReader<ShootEvent>,
    mut spawn_bullet_events: EventWriter<SpawnBulletEvent>,
    children: Query<&Children>,
    weapons: Query<(Entity, &Cooldown), With<Weapon>>,
) {
    for &ShootEvent { shooter } in shoot_events.iter() {
        if let Ok(children) = children.get(shooter) {
            for (weapon, _) in children
                .iter()
                .filter_map(|child| weapons.get(*child).ok())
                .filter(|(_, cooldown)| cooldown.0.finished())
            {
                spawn_bullet_events.send(SpawnBulletEvent { weapon, shooter });
            }
        }
    }
}

pub fn spawn_bullets(
    mut commands: Commands,
    mut events: EventReader<SpawnBulletEvent>,
    mut weapons: Query<(&Weapon, &mut Cooldown, &Transform, &GlobalTransform)>,
) {
    for SpawnBulletEvent {
        weapon: weapon_entity,
        shooter,
    } in events.iter()
    {
        if let Ok((weapon, mut cooldown, transform, global_transform)) =
            weapons.get_mut(*weapon_entity)
        {
            cooldown.0.reset();
            let damage = weapon.damage();
            let size = Vec2::new(16.0, 8.0);
            let bullet_speed = 300.0;
            let bullet_rotation = transform.rotation;
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
