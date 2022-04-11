use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::{physics::IntoEntity, prelude::IntersectionEvent};

use crate::{player::Player, Owner};

#[derive(Component, Inspectable, Debug)]
pub enum Weapon {
    Laser { damage: f32, cooldown: f32 },
}

#[derive(Inspectable, Default, Clone)]
pub struct WeaponSlot {
    pub weapon: Option<Entity>,
    pub position: Vec2,
}

#[derive(Component, Inspectable)]
pub struct WeaponSlots {
    pub weapons: Vec<WeaponSlot>,
}

pub enum Contact {
    HealthBullet(Entity, Entity),
}

#[derive(Component, Inspectable)]
pub struct Bullet {
    pub damage: f32,
}

#[derive(Component, Inspectable)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(amount: f32) -> Self {
        Self {
            current: amount,
            max: amount,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
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
        app.register_inspectable::<Weapon>()
            .register_inspectable::<WeaponSlot>()
            .register_inspectable::<WeaponSlots>()
            .register_inspectable::<Bullet>()
            .register_inspectable::<Health>()
            .add_event::<EquipWeaponEvent>()
            .add_event::<Contact>()
            .add_system(equip_weapon)
            .add_system(handle_intersections)
            .add_system(handle_contacts)
            .add_system(despawn_dead)
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
    mut events: EventReader<Contact>,
    mut healths: Query<&mut Health>,
    bullets: Query<&Bullet>,
) {
    for event in events.iter() {
        match *event {
            Contact::HealthBullet(health_entity, bullet_entity) => {
                let mut health = healths.get_mut(health_entity).unwrap();
                let bullet = bullets.get(bullet_entity).unwrap();
                commands.entity(bullet_entity).despawn();
                health.current -= bullet.damage;
            }
        }
    }
}

pub fn despawn_dead(mut commands: Commands, healths: Query<(Entity, &Health), Changed<Health>>) {
    for (entity, health) in healths.iter() {
        if health.is_dead() {
            commands.entity(entity).despawn();
        }
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
                        damage: 1.0,
                        cooldown: 1.0,
                    })
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
                    .insert(Transform::from_xyz(slot.position.x, slot.position.y, 0.0))
                    .insert(Visibility { is_visible: true });
                commands.entity(event.entity).add_child(event.weapon_entity);
            }
        }
    }
}
