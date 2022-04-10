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

#[derive(Component)]
pub struct DespawnTimer {
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
