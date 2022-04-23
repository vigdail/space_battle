use bevy::{prelude::*, reflect::TypeUuid};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Default, Component)]
pub struct Scores {
    pub amount: u32,
}
#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Default, Component, Clone, Serialize, Deserialize)]
pub struct Loot {
    pub score: u32,
}

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Default, Component)]
pub struct Cooldown(#[cfg_attr(feature = "debug", inspectable(ignore))] pub Timer);

impl Cooldown {
    pub fn from_seconds(seconds: f32) -> Self {
        Self(Timer::new(Duration::from_secs_f32(seconds), false))
    }
}

impl From<f32> for Cooldown {
    fn from(secs: f32) -> Self {
        Self::from_seconds(secs)
    }
}

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub enum Weapon {
    Laser { damage: u32, cooldown: f32 },
}

impl Weapon {
    pub fn damage(&self) -> u32 {
        match self {
            Weapon::Laser { damage, .. } => *damage,
        }
    }

    pub fn cooldown(&self) -> f32 {
        match self {
            Weapon::Laser { cooldown, .. } => *cooldown,
        }
    }
}

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Default, Clone)]
pub struct WeaponSlot {
    pub weapon: Option<Entity>,
    pub transform: Transform,
}

impl WeaponSlot {
    pub fn from_raw(def: &WeaponSlotRaw, weapon: Option<Entity>) -> Self {
        Self {
            weapon,
            transform: Transform::from_translation(def.position.extend(0.0))
                .with_rotation(Quat::from_rotation_z(def.rotation.to_radians())),
        }
    }
}

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component)]
pub struct WeaponSlots {
    pub slots: Vec<WeaponSlot>,
}

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component)]
pub struct Bullet {
    pub damage: u32,
}

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Health {
    pub fn new(amount: u32) -> Self {
        Self {
            current: amount,
            max: amount,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current == 0
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "WeaponSlot")]
pub struct WeaponSlotRaw {
    pub weapon: Weapon,
    pub position: Vec2,
    pub rotation: f32,
}

#[derive(Serialize, Deserialize, TypeUuid, Default)]
#[uuid = "57f9ff4b-f4d1-4e51-9572-483113a861c9"]
#[serde(rename = "Unit")]
pub struct UnitRaw {
    pub name: String,
    pub health: u32,
    pub weapon_slots: Vec<WeaponSlotRaw>,
    pub loot: Loot,
    pub color: [f32; 3],
}
