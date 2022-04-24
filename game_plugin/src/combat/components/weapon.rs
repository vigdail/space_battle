use std::time::Duration;

use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

use crate::prefab::FromRaw;

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

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename = "WeaponSlot")]
pub struct WeaponSlotRaw {
    pub weapon: Weapon,
    pub position: Vec2,
    pub rotation: f32,
}

impl FromRaw for WeaponSlot {
    type Raw = WeaponSlotRaw;

    fn from_raw(raw: &Self::Raw, world: &mut World) -> Self {
        let transform = Transform::from_translation(raw.position.extend(0.0))
            .with_rotation(Quat::from_rotation_z(raw.rotation.to_radians()));
        let weapon = world
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::splat(8.0)),
                    ..default()
                },
                transform,
                ..default()
            })
            .insert(Name::new("Weapon"))
            .insert(raw.weapon.clone())
            .insert(Cooldown::from_seconds(raw.weapon.cooldown()))
            .id();

        Self {
            weapon: Some(weapon),
            transform,
        }
    }
}
