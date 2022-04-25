use std::time::Duration;

use bevy::{asset::Asset, prelude::*, reflect::TypeUuid};

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

#[derive(Serialize, Deserialize, Clone, TypeUuid)]
#[uuid = "4825c543-fe54-4aec-82b8-5cbf413f3a88"]
#[serde(rename = "Weapon")]
pub struct WeaponPrefab {
    weapon: Weapon,
}

impl Prefab for WeaponPrefab {
    fn apply(&self, entity: Entity, world: &mut World) {
        let transform = world.entity(entity).get::<Transform>().cloned();

        let mut entity = world.entity_mut(entity);
        entity
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::splat(8.0)),
                    ..default()
                },
                ..default()
            })
            .insert(self.weapon.clone())
            .insert(Cooldown::from_seconds(self.weapon.cooldown()));
        if let Some(transform) = transform {
            entity.insert(transform);
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PrefabAsset<T: Prefab + Clone> {
    Prefab(T),
    Asset(String),
}

impl<T: Prefab + Clone + Asset> PrefabAsset<T> {
    pub fn as_handle(&self, world: &mut World) -> Handle<T> {
        match self {
            PrefabAsset::Prefab(prefab) => world.resource_mut::<Assets<T>>().add(prefab.clone()),
            PrefabAsset::Asset(path) => world.resource::<AssetServer>().get_handle(path),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename = "WeaponSlot")]
pub struct WeaponSlotPrefab {
    pub weapon: Option<PrefabAsset<WeaponPrefab>>,
    pub position: Vec2,
    pub rotation: f32,
}

impl Prefab for WeaponSlotPrefab {
    fn apply(&self, entity: Entity, world: &mut World) {
        let transform = Transform::from_translation(self.position.extend(0.0))
            .with_rotation(Quat::from_rotation_z(self.rotation.to_radians()));

        let weapon = self.weapon.as_ref().map(|weapon| weapon.as_handle(world));

        let mut entity = world.entity_mut(entity);
        if let Some(weapon) = weapon {
            entity.insert(weapon);
        }
        entity
            .insert(Name::new("Weapon"))
            .insert(WeaponSlot)
            .insert_bundle(TransformBundle::from_transform(transform));
    }
}
