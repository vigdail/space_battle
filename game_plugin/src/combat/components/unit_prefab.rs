use bevy::{prelude::*, reflect::TypeUuid, render::texture::DEFAULT_IMAGE_HANDLE};
use heron::CollisionShape;
use serde::{Deserialize, Serialize};

use crate::prefab::Prefab;

use super::{Health, Loot, WeaponSlotPrefab};

#[derive(Serialize, Deserialize, TypeUuid, Default, Clone)]
#[uuid = "57f9ff4b-f4d1-4e51-9572-483113a861c9"]
#[serde(rename = "Unit")]
pub struct UnitPrefab {
    pub name: String,
    pub health: u32,
    pub weapon_slots: Vec<WeaponSlotPrefab>,
    pub loot: Loot,
    pub color: [f32; 3],
}

impl Prefab for UnitPrefab {
    fn apply(&self, entity: Entity, world: &mut World) {
        let size = Vec2::splat(32.0);
        let weapons = self
            .weapon_slots
            .iter()
            .map(|slot| {
                let entity = world.spawn().id();
                slot.apply(entity, world);
                entity
            })
            .collect::<Vec<_>>();

        let texture: Handle<Image> = DEFAULT_IMAGE_HANDLE.typed();

        world
            .entity_mut(entity)
            .insert(Sprite {
                color: self.color.into(),
                custom_size: Some(size),
                ..default()
            })
            .insert(texture)
            .insert(Visibility::default())
            .insert(CollisionShape::Cuboid {
                half_extends: size.extend(0.0) / 2.0,
                border_radius: None,
            })
            .insert(Health::new(self.health))
            .insert(Name::new(self.name.clone()))
            .insert(self.loot.clone())
            .insert_children(0, &weapons);
    }
}
