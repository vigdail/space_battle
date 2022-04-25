pub mod components;
pub mod events;
mod systems;

use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
};
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;

use crate::prefab::RegisterPrefab;

use self::systems::*;
pub use self::{components::*, events::*};

#[macro_export]
macro_rules! ron_loader {
    ($loader:ident, $prefab:ident, [$($exts:expr), +]) => {
        impl AssetLoader for $loader {
            fn load<'a>(
                &'a self,
                bytes: &'a [u8],
                load_context: &'a mut bevy::asset::LoadContext,
            ) -> bevy::asset::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
                Box::pin(async move {
                    let custom_asset = ron::de::from_bytes::<$prefab>(bytes)?;
                    load_context.set_default_asset(LoadedAsset::new(custom_asset));
                    Ok(())
                })
            }

            fn extensions(&self) -> &[&str] {
                &[$($exts),+]
            }
        }
    };
}

pub struct UnitLoader;
ron_loader!(UnitLoader, UnitPrefab, ["unit.ron"]);

pub struct WeaponLoader;
ron_loader!(WeaponLoader, WeaponPrefab, ["weapon.ron"]);

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
        app.register_prefab::<UnitPrefab>()
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
