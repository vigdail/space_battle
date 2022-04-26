use bevy::{prelude::*, reflect::TypeUuid};

#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;
use heron::{CollisionShape, RigidBody, SensorShape};
use serde::{Deserialize, Serialize};

use crate::{
    prefab::{self, Prefab, PrefabHandle},
    Lifetime,
};

use super::Cooldown;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Default, Component, Clone, Serialize, Deserialize)]
pub struct Damage(pub u32);

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component, Debug, Clone)]
pub struct Weapon {
    pub bullet: Handle<BulletPrefab>,
}

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Default, Clone, Component)]
pub struct WeaponSlot;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component)]
pub struct Bullet {
    pub damage: u32,
}

#[derive(Serialize, Deserialize, TypeUuid, Clone)]
#[uuid = "4825c543-fe54-4aec-82b8-5cbf413f3a88"]
#[serde(rename = "Weapon")]
pub struct WeaponPrefab {
    pub bullet: PrefabHandle<BulletPrefab>,
    pub damage: Damage,
    pub cooldown: Cooldown,
}

impl Prefab for WeaponPrefab {
    fn apply(&self, entity: Entity, world: &mut World) {
        let bullet_handle = self.bullet.as_handle(world);

        let mut entity = world.entity_mut(entity);
        entity
            .insert_bundle(prefab::SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::splat(8.0)),
                    ..default()
                },
                ..default()
            })
            .insert(self.damage.clone())
            .insert(Weapon {
                bullet: bullet_handle,
            })
            .insert(self.cooldown.clone());
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename = "WeaponSlot")]
pub struct WeaponSlotPrefab {
    pub weapon: Option<PrefabHandle<WeaponPrefab>>,
    pub position: Vec2,
    pub rotation: f32,
}

impl Prefab for WeaponSlotPrefab {
    fn apply(&self, entity: Entity, world: &mut World) {
        let transform = Transform::from_translation(self.position.extend(0.0))
            .with_rotation(Quat::from_rotation_z(self.rotation.to_radians()));

        if let Some(weapon) = &self.weapon {
            weapon.apply(entity, world);
        }
        let mut entity = world.entity_mut(entity);
        entity
            .insert(Name::new("Weapon"))
            .insert(WeaponSlot)
            .insert_bundle(TransformBundle::from_transform(transform));
    }
}

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Serialize, Deserialize, Clone, TypeUuid)]
#[serde(rename = "Bullet")]
#[uuid = "e6e66da1-8bdf-4972-be42-7fce4db7d07b"]
pub struct BulletPrefab {
    pub size: Vec2,
    pub color: [f32; 3],
}

impl Prefab for BulletPrefab {
    fn apply(&self, entity: Entity, world: &mut World) {
        world
            .entity_mut(entity)
            .insert_bundle(prefab::SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(self.size),
                    color: self.color.into(),
                    ..default()
                },
                ..default()
            })
            .insert(Lifetime {
                timer: Timer::from_seconds(1.0, false),
            })
            .insert(RigidBody::KinematicVelocityBased)
            .insert(SensorShape)
            .insert(CollisionShape::Cuboid {
                half_extends: self.size.extend(0.0) / 2.0,
                border_radius: None,
            });
    }
}
