pub mod components;
pub mod events;
mod systems;

use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
use serde::de::DeserializeOwned;

use crate::prefab::RegisterPrefab;

use self::systems::*;
pub use self::{components::*, events::*};

pub struct UnitLoader;

impl AssetLoader for UnitLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        load_ron::<UnitPrefab>(bytes, load_context)
    }

    fn extensions(&self) -> &[&str] {
        &["unit.ron"]
    }
}

pub struct WeaponLoader;

impl AssetLoader for WeaponLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        load_ron::<WeaponPrefab>(bytes, load_context)
    }

    fn extensions(&self) -> &[&str] {
        &["weapon.ron"]
    }
}

fn load_ron<'a, T>(
    bytes: &'a [u8],
    load_context: &'a mut bevy::asset::LoadContext,
) -> bevy::asset::BoxedFuture<'a, Result<(), anyhow::Error>>
where
    T: Send + Sync + DeserializeOwned + TypeUuid + 'static,
{
    Box::pin(async move {
        let custom_asset = ron::de::from_bytes::<T>(bytes)?;
        load_context.set_default_asset(LoadedAsset::new(custom_asset));
        Ok(())
    })
}

pub struct EquipWeaponEvent {
    pub slot_entity: Entity,
    pub weapon: Weapon,
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        app.register_inspectable::<Weapon>()
            .register_inspectable::<WeaponSlot>()
            .register_inspectable::<Bullet>()
            .register_inspectable::<Loot>()
            .register_inspectable::<Health>();
        app.add_asset::<UnitPrefab>()
            .add_asset::<WeaponPrefab>()
            .register_prefab::<UnitPrefab>()
            .register_prefab::<WeaponPrefab>()
            .add_asset_loader(UnitLoader)
            .add_asset_loader(WeaponLoader)
            .add_event::<EquipWeaponEvent>()
            .add_event::<ShootEvent>()
            .add_event::<SpawnBulletEvent>()
            .add_event::<RewardEvent>()
            .add_event::<ContactEvent>()
            .add_system(equip_weapon)
            .add_system(handle_intersections)
            .add_system(handle_contacts)
            .add_system(despawn_dead)
            .add_system(update_cooldowns)
            .add_system(handle_shoot_events)
            .add_system(spawn_bullets)
            .add_system(apply_score_reward)
            .add_system(test_equip_weapon);
    }
}
