use std::time::Duration;

use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};
#[cfg(feature = "debug")]
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::{
    na::Isometry2,
    physics::{
        ColliderBundle, IntoEntity, RapierConfiguration, RigidBodyBundle, RigidBodyPositionSync,
    },
    prelude::{
        ActiveEvents, ColliderShape, ColliderType, IntersectionEvent, RigidBodyForces,
        RigidBodyVelocity,
    },
};
use serde::{Deserialize, Serialize};

use crate::{player::Player, Lifetime, Owner};

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Default, Component)]
pub struct Scores {
    pub amount: u32,
}

// TODO: Which entity to reward
pub struct RewardEvent {
    pub score: u32,
}

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Default, Component, Clone, Serialize, Deserialize)]
pub struct Loot {
    pub score: u32,
}

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Default, Component)]
pub struct Cooldown(#[cfg_attr(feature = "debug", inspectable(ignore))] pub Timer);

impl Cooldown {
    pub fn from_seconds(seconds: f32) -> Self {
        Self(Timer::new(Duration::from_secs_f32(seconds), false))
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
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct WeaponSlot {
    pub weapon: Option<Entity>,
    #[cfg_attr(feature = "debug", inspectable(ignore))]
    pub transform: Isometry2<f32>,
}

impl WeaponSlot {
    pub fn from_def(def: &WeaponSlotDef, weapon: Option<Entity>) -> Self {
        Self {
            weapon,
            transform: def.transform,
        }
    }
}

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component, Serialize, Deserialize)]
pub struct WeaponSlots {
    pub weapons: Vec<WeaponSlot>,
}

pub struct ShootEvent {
    pub shooter: Entity,
}

pub struct SpawnBulletEvent {
    pub weapon_slot: WeaponSlot,
    pub shooter: Entity,
}

pub enum Contact {
    HealthBullet(Entity, Entity),
}

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component)]
pub struct Bullet {
    pub damage: u32,
}

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Health {
    pub fn new(amount: u32) -> Self {
        Self {
            current: amount,
            max: amount,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current == 0
    }
}

#[derive(Serialize, Deserialize)]
pub struct WeaponSlotDef {
    pub weapon: Weapon,
    pub transform: Isometry2<f32>,
}

#[derive(Serialize, Deserialize, TypeUuid)]
#[uuid = "57f9ff4b-f4d1-4e51-9572-483113a861c9"]
pub struct UnitDefs {
    pub name: String,
    pub health: u32,
    pub weapon_slots: Vec<WeaponSlotDef>,
    pub loot: Loot,
    pub color: [f32; 3],
}

#[derive(Default)]
pub struct UnitDefsLoader;

impl AssetLoader for UnitDefsLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<UnitDefs>(bytes)?;
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
        app.add_asset::<UnitDefs>()
            .init_asset_loader::<UnitDefsLoader>()
            .add_event::<EquipWeaponEvent>()
            .add_event::<ShootEvent>()
            .add_event::<SpawnBulletEvent>()
            .add_event::<RewardEvent>()
            .add_event::<Contact>()
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

pub fn handle_intersections(
    mut intersection_events: EventReader<IntersectionEvent>,
    bullets: Query<&Bullet>,
    healths: Query<&Health>,
    owners: Query<&Owner>,
    mut contact_events: EventWriter<Contact>,
) {
    for event in intersection_events.iter().filter(|e| e.intersecting) {
        let entity1 = event.collider1.entity();
        let entity2 = event.collider2.entity();

        let mut check_bullet = |health_entity, bullet_entity| {
            if healths.get(health_entity).is_ok() && bullets.get(bullet_entity).is_ok() {
                let is_owner = owners
                    .get(bullet_entity)
                    .map(|owner| owner.entity == health_entity)
                    .unwrap_or(false);

                if !is_owner {
                    contact_events.send(Contact::HealthBullet(health_entity, bullet_entity));
                }
            }
        };

        check_bullet(entity1, entity2);
        check_bullet(entity2, entity1);
    }
}

pub fn handle_contacts(
    mut commands: Commands,
    mut contact_events: EventReader<Contact>,
    mut healths: Query<&mut Health>,
    bullets: Query<&Bullet>,
) {
    for event in contact_events.iter() {
        match *event {
            Contact::HealthBullet(health_entity, bullet_entity) => {
                if let Some((mut health, bullet)) = healths
                    .get_mut(health_entity)
                    .ok()
                    .zip(bullets.get(bullet_entity).ok())
                {
                    health.current = health.current.saturating_sub(bullet.damage);
                }
                commands.entity(bullet_entity).despawn();
            }
        }
    }
}

pub fn despawn_dead(
    mut commands: Commands,
    mut reward_events: EventWriter<RewardEvent>,
    healths: Query<(Entity, &Health, Option<&Loot>), Changed<Health>>,
) {
    for (entity, health, loot) in healths.iter() {
        if health.is_dead() {
            if let Some(loot) = loot {
                reward_events.send(RewardEvent { score: loot.score });
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn update_cooldowns(time: Res<Time>, mut cooldowns: Query<&mut Cooldown>) {
    for mut cooldown in cooldowns.iter_mut() {
        cooldown.0.tick(time.delta());
    }
}

pub fn test_equip_weapon(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut events: EventWriter<EquipWeaponEvent>,
    players: Query<(Entity, &WeaponSlots), With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::E) {
        for (entity, slots) in players.iter() {
            let slot_index = slots.weapons.iter().position(|slot| slot.weapon.is_none());

            if let Some(slot_index) = slot_index {
                let weapon_entity = commands
                    .spawn()
                    .insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::GREEN,
                            custom_size: Some(Vec2::splat(8.0)),
                            ..Default::default()
                        },
                        visibility: Visibility { is_visible: false },
                        ..Default::default()
                    })
                    .insert(Weapon::Laser {
                        damage: 1,
                        cooldown: 0.2,
                    })
                    .insert(Cooldown::from_seconds(0.2)) // TODO
                    .insert(Name::new(format!("Weapon {}", slot_index)))
                    .id();

                events.send(EquipWeaponEvent {
                    entity,
                    weapon_entity,
                    slot_index,
                });
            }
        }
    }
}

pub fn equip_weapon(
    mut commands: Commands,
    mut events: EventReader<EquipWeaponEvent>,
    mut slots: Query<&mut WeaponSlots>,
) {
    for event in events.iter() {
        if let Ok(mut slots) = slots.get_mut(event.entity) {
            if let Some(slot) = slots.weapons.get_mut(event.slot_index) {
                slot.weapon = Some(event.weapon_entity);
                commands
                    .entity(event.weapon_entity)
                    .insert(Transform::from_xyz(
                        slot.transform.translation.x,
                        slot.transform.translation.y,
                        0.0,
                    ))
                    .insert(Visibility { is_visible: true });
                commands.entity(event.entity).add_child(event.weapon_entity);
            }
        }
    }
}

fn handle_shoot_events(
    mut shoot_events: EventReader<ShootEvent>,
    mut spawn_bullet_events: EventWriter<SpawnBulletEvent>,
    weapon_slots: Query<&WeaponSlots>,
    weapons: Query<(&Weapon, &Cooldown)>,
) {
    for ShootEvent { shooter } in shoot_events.iter() {
        if let Ok(slots) = weapon_slots.get(*shooter) {
            for weapon_slot in slots.weapons.iter().filter(|slot| {
                slot.weapon
                    .and_then(|weapon| {
                        weapons
                            .get(weapon)
                            .ok()
                            .filter(|(_, cooldown)| cooldown.0.finished())
                    })
                    .is_some()
            }) {
                spawn_bullet_events.send(SpawnBulletEvent {
                    weapon_slot: weapon_slot.clone(),
                    shooter: *shooter,
                });
            }
        }
    }
}

fn spawn_bullets(
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    mut events: EventReader<SpawnBulletEvent>,
    mut weapons: Query<(&Weapon, &mut Cooldown, &GlobalTransform)>,
) {
    for SpawnBulletEvent {
        weapon_slot,
        shooter,
    } in events.iter()
    {
        if let Some((weapon, mut cooldown, global_transform)) = weapon_slot
            .weapon
            .and_then(|weapon_entity| weapons.get_mut(weapon_entity).ok())
        {
            cooldown.0.reset();
            let damage = weapon.damage();
            let size = Vec2::new(16.0, 8.0);
            let collider_size = size / rapier_config.scale;
            let bullet_speed = 300.0;
            let bullet_rotation = weapon_slot.transform.rotation;
            let bullet_velocity = bullet_rotation.transform_vector(&[bullet_speed, 0.0].into());

            let rigidbody = RigidBodyBundle {
                velocity: RigidBodyVelocity {
                    linvel: bullet_velocity,
                    ..Default::default()
                }
                .into(),
                forces: RigidBodyForces {
                    gravity_scale: 0.0,
                    ..Default::default()
                }
                .into(),
                position: (
                    global_transform.translation.truncate(),
                    weapon_slot.transform.rotation.angle(),
                )
                    .into(),
                ..Default::default()
            };

            let collider = ColliderBundle {
                collider_type: ColliderType::Sensor.into(),
                shape: ColliderShape::cuboid(collider_size.x / 2.0, collider_size.y / 2.0).into(),
                flags: (ActiveEvents::INTERSECTION_EVENTS).into(),
                ..Default::default()
            };
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLUE,
                        custom_size: Some(size),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Bullet { damage })
                .insert(Lifetime {
                    timer: Timer::from_seconds(1.0, false),
                })
                .insert_bundle(rigidbody)
                .insert_bundle(collider)
                .insert(RigidBodyPositionSync::Discrete)
                .insert(Owner { entity: *shooter });
        }
    }
}

fn apply_score_reward(
    mut reward_events: EventReader<RewardEvent>,
    mut players: Query<&mut Scores, With<Player>>,
) {
    for RewardEvent { score } in reward_events.iter() {
        for mut scores in players.iter_mut() {
            scores.amount += score;
        }
    }
}
