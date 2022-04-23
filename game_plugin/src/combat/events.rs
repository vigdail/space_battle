use bevy::prelude::*;

use super::components::WeaponSlot;

// TODO: Which entity to reward
pub struct RewardEvent {
    pub score: u32,
}

pub struct ShootEvent {
    pub shooter: Entity,
}

pub struct SpawnBulletEvent {
    pub weapon_slot: WeaponSlot,
    pub shooter: Entity,
}

pub enum ContactEvent {
    HealthBullet(Entity, Entity),
}
