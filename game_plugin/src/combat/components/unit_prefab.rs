use bevy::{prelude::*, reflect::TypeUuid};
use serde::{Deserialize, Serialize};

use crate::prefab::{FromRaw, Prefab};

use super::{Health, Loot, WeaponSlot, WeaponSlotRaw, WeaponSlots};

#[derive(Serialize, Deserialize, TypeUuid, Default, Clone)]
#[uuid = "57f9ff4b-f4d1-4e51-9572-483113a861c9"]
#[serde(rename = "Unit")]
pub struct UnitPrefab {
    pub name: String,
    pub health: u32,
    pub weapon_slots: Vec<WeaponSlotRaw>,
    pub loot: Loot,
    pub color: [f32; 3],
}

impl Prefab for UnitPrefab {
    fn apply(&self, entity: Entity, world: &mut World) {
        let slots = self
            .weapon_slots
            .iter()
            .map(|slot_raw| WeaponSlot::from_raw(slot_raw, world))
            .collect::<Vec<_>>();

        let weapons = slots
            .iter()
            .filter_map(|slot| slot.weapon)
            .collect::<Vec<_>>();

        world
            .entity_mut(entity)
            .insert(Health::new(self.health))
            .insert(Name::new(self.name.clone()))
            .insert(WeaponSlots { slots })
            .insert(self.loot.clone())
            .insert_children(0, &weapons);
    }
}
