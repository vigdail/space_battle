use bevy::prelude::*;
use bevy_rapier2d::{
    na::Vector2,
    physics::{RapierConfiguration, RigidBodyBundle, RigidBodyPositionSync},
    prelude::{RigidBodyForces, RigidBodyVelocity, RigidBodyVelocityComponent},
};

use crate::components::{Bullet, DespawnTimer, EquipWeaponEvent, Player, Weapon, WeaponSlots};

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
    players: Query<&Children, With<Player>>,
    weapons: Query<(&Weapon, &GlobalTransform)>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    for children in players.iter() {
        for child in children.iter() {
            if let Ok((weapon, &global_transform)) = weapons.get(*child) {
                let damage = match weapon {
                    &Weapon::Laser { damage, .. } => damage,
                };
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::BLUE,
                            custom_size: Some(Vec2::new(8.0, 16.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Bullet { damage })
                    .insert(DespawnTimer {
                        timer: Timer::from_seconds(1.0, false),
                    })
                    .insert_bundle(RigidBodyBundle {
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
                    })
                    .insert(RigidBodyPositionSync::Discrete);
            }
        }
    }
}

pub fn update_despawn_timers(time: Res<Time>, mut timers: Query<&mut DespawnTimer>) {
    for mut timer in timers.iter_mut() {
        timer.timer.tick(time.delta());
    }
}

pub fn despawning(mut commands: Commands, timers: Query<(Entity, &DespawnTimer)>) {
    for (entity, timer) in timers.iter() {
        if timer.timer.finished() {
            commands.entity(entity).despawn_recursive();
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
                commands.entity(event.entity).add_child(event.weapon_entity);
                commands
                    .entity(event.weapon_entity)
                    .insert(Transform::from_xyz(slot.position.x, slot.position.y, 0.0))
                    .insert(Visibility { is_visible: true });
            }
        }
    }
}
