use bevy::{prelude::*, reflect::TypeUuid};
use heron::CollisionShape;
use serde::{Deserialize, Serialize};

use crate::{
    prefab::{self, Prefab},
    prefab_loader,
};

use super::{Health, Loot, WeaponSlotPrefab};

pub struct UnitLoader;
prefab_loader!(UnitLoader, UnitPrefab, ["unit.ron"]);

#[derive(Serialize, Deserialize, TypeUuid, Clone)]
#[uuid = "57f9ff4b-f4d1-4e51-9572-483113a861c9"]
#[serde(rename = "Unit")]
pub struct UnitPrefab {
    pub name: String,
    pub health: u32,
    pub weapon_slots: Vec<WeaponSlotPrefab>,
    pub loot: Loot,
    pub body: String,
}

impl Prefab for UnitPrefab {
    fn apply(&self, entity: Entity, world: &mut World) {
        let size = Vec2::splat(32.0);

        let texture: Handle<Image> = world.resource::<AssetServer>().load(&self.body);

        world
            .entity_mut(entity)
            .insert_bundle(prefab::SpriteBundle {
                texture,
                ..default()
            })
            .insert(CollisionShape::Cuboid {
                half_extends: size.extend(0.0) / 2.0,
                border_radius: None,
            })
            .insert(Health::new(self.health))
            .insert(Name::new(self.name.clone()))
            .insert(self.loot.clone());

        self.weapon_slots.apply(entity, world);
    }
}
