pub mod components;
pub mod events;
mod systems;

use std::marker::PhantomData;

use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
use serde::de::DeserializeOwned;

use self::systems::*;
pub use self::{components::*, events::*};

#[derive(Default)]
pub struct RonLoader<T> {
    _phantom: PhantomData<T>,
}

impl<T> AssetLoader for RonLoader<T>
where
    T: Send + Sync + DeserializeOwned + TypeUuid + 'static,
{
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<T>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

pub struct EquipWeaponEvent {
    pub entity: Entity,
    pub weapon_entity: Entity,
    pub slot_index: usize,
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        app.register_inspectable::<Weapon>()
            .register_inspectable::<WeaponSlot>()
            .register_inspectable::<WeaponSlots>()
            .register_inspectable::<Bullet>()
            .register_inspectable::<Loot>()
            .register_inspectable::<Health>();
        app.add_asset::<UnitPrefab>()
            .init_asset_loader::<RonLoader<UnitPrefab>>()
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
