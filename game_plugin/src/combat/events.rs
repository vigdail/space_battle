use bevy::prelude::*;

use super::WeaponPrefab;

// TODO: Which entity to reward
pub struct RewardEvent {
    pub score: u32,
}

pub struct ShootEvent {
    pub shooter: Entity,
}

pub struct SpawnBulletEvent {
    pub weapon: Entity,
    pub shooter: Entity,
}

pub enum ContactEvent {
    HealthBullet(Entity, Entity),
}

pub struct EquipWeaponEvent {
    pub slot_entity: Entity,
    pub weapon: WeaponPrefab,
}
