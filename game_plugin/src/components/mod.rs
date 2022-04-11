use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component, Inspectable, Debug)]
pub enum Weapon {
    Laser { damage: f32, cooldown: f32 },
}

#[derive(Component, Inspectable)]
pub struct Bullet {
    pub damage: f32,
}

#[derive(Component, Inspectable)]
pub struct Owner {
    pub entity: Entity,
}

#[derive(Component, Inspectable)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(amount: f32) -> Self {
        Self {
            current: amount,
            max: amount,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
}

#[derive(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

#[derive(Inspectable, Default, Clone)]
pub struct WeaponSlot {
    pub weapon: Option<Entity>,
    pub position: Vec2,
}

#[derive(Component, Inspectable)]
pub struct WeaponSlots {
    pub weapons: Vec<WeaponSlot>,
}

pub struct EquipWeaponEvent {
    pub entity: Entity,
    pub weapon_entity: Entity,
    pub slot_index: usize,
}

pub enum Contact {
    HealthBullet(Entity, Entity),
}
