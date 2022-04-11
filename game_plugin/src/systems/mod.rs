use bevy::prelude::*;
use bevy_rapier2d::{
    na::Vector2,
    physics::{
        ColliderBundle, IntoEntity, RapierConfiguration, RigidBodyBundle, RigidBodyPositionSync,
    },
    prelude::{
        ActiveEvents, ColliderShape, ColliderType, IntersectionEvent, RigidBodyForces,
        RigidBodyVelocity, RigidBodyVelocityComponent,
    },
};

use crate::components::{
    Bullet, EquipWeaponEvent, Health, Lifetime, Owner, Player, Weapon, WeaponSlots,
};

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_parameters: Res<RapierConfiguration>,
    mut players: Query<(&Player, &mut RigidBodyVelocityComponent)>,
) {
    for (player, mut vels) in players.iter_mut() {
        let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta: Vector2<_> = [x_axis as f32, y_axis as f32].into();
        if move_delta != Vector2::zeros() {
            move_delta /= move_delta.magnitude() * rapier_parameters.scale;
        }

        vels.linvel = move_delta * player.speed;
    }
}

pub fn player_shoot(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    rapier_config: Res<RapierConfiguration>,
    players: Query<(Entity, &WeaponSlots), With<Player>>,
    weapons: Query<(&Weapon, &GlobalTransform)>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    for (player_entity, slots) in players.iter() {
        for weapon in slots.weapons.iter().filter_map(|slot| slot.weapon) {
            if let Ok((weapon, &global_transform)) = weapons.get(weapon) {
                let damage = match weapon {
                    &Weapon::Laser { damage, .. } => damage,
                };
                let size = Vec2::new(8.0, 16.0);
                let collider_size = size / rapier_config.scale;
                let rigidbody = RigidBodyBundle {
                    velocity: RigidBodyVelocity {
                        linvel: [0.0, 300.0].into(),
                        ..Default::default()
                    }
                    .into(),
                    forces: RigidBodyForces {
                        gravity_scale: 0.0,
                        ..Default::default()
                    }
                    .into(),
                    position: [
                        global_transform.translation.x,
                        global_transform.translation.y,
                    ]
                    .into(),
                    ..Default::default()
                };

                let collider = ColliderBundle {
                    collider_type: ColliderType::Sensor.into(),
                    shape: ColliderShape::cuboid(collider_size.x / 2.0, collider_size.y / 2.0)
                        .into(),
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
                    .insert(Owner {
                        entity: player_entity,
                    });
            }
        }
    }
}

pub fn track_lifetime(
    mut cmd: Commands,
    mut query: Query<(Entity, &mut Lifetime)>,
    time: ResMut<Time>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        if lifetime.timer.tick(time.delta()).just_finished() {
            cmd.entity(entity).despawn();
        }
    }
}

pub fn print_intersections(
    mut commands: Commands,
    mut intersection_events: EventReader<IntersectionEvent>,
    bullets: Query<(&Bullet, &Owner)>,
    mut healths: Query<&mut Health>,
) {
    for event in intersection_events.iter() {
        if !event.intersecting {
            continue;
        }
        let entity1 = event.collider1.entity();
        let entity2 = event.collider2.entity();
        match (bullets.get(entity1), healths.get_mut(entity2)) {
            (Ok((bullet, owner)), Ok(mut health)) => {
                if owner.entity == entity2 {
                    continue;
                }

                health.current -= bullet.damage;
                commands.entity(entity1).despawn();
            }
            _ => {}
        }

        match (bullets.get(entity2), healths.get_mut(entity1)) {
            (Ok((bullet, owner)), Ok(mut health)) => {
                if owner.entity == entity1 {
                    continue;
                }

                health.current -= bullet.damage;
                commands.entity(entity2).despawn();
            }
            _ => {}
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
