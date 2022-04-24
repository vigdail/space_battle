use std::time::Duration;

use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

use crate::prefab::Prefab;

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
#[derive(Default, Clone, Component)]
pub struct WeaponSlot;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component)]
pub struct Bullet {
    pub damage: u32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename = "WeaponSlot")]
pub struct WeaponSlotPrefab {
    pub weapon: Option<Weapon>,
    pub position: Vec2,
    pub rotation: f32,
}

impl Prefab for WeaponSlotPrefab {
    fn apply(&self, entity: Entity, world: &mut World) {
        let transform = Transform::from_translation(self.position.extend(0.0))
            .with_rotation(Quat::from_rotation_z(self.rotation.to_radians()));
        let mut entity = world.entity_mut(entity);
        if let Some(weapon) = self.weapon.clone() {
            entity
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2::splat(8.0)),
                        ..default()
                    },
                    ..default()
                })
                .insert(weapon.clone())
                .insert(Cooldown::from_seconds(weapon.cooldown()));
        }
        entity
            .insert(Name::new("Weapon"))
            .insert(WeaponSlot)
            .insert_bundle(TransformBundle::from_transform(transform));
    }
}
