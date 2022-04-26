pub mod components;
pub mod events;
mod systems;

use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
};
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;

use crate::{prefab::RegisterPrefab, prefab_loader};

use self::systems::*;
pub use self::{components::*, events::*};

pub struct UnitLoader;
prefab_loader!(UnitLoader, UnitPrefab, ["unit.ron"]);

pub struct WeaponLoader;
prefab_loader!(WeaponLoader, WeaponPrefab, ["weapon.ron"]);

pub struct BulletLoader;
prefab_loader!(BulletLoader, BulletPrefab, ["bullet.ron"]);

pub struct EquipWeaponEvent {
    pub slot_entity: Entity,
    pub weapon: WeaponPrefab,
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
            .register_prefab::<BulletPrefab>()
            .add_asset_loader(UnitLoader)
            .add_asset_loader(WeaponLoader)
            .add_asset_loader(BulletLoader)
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
