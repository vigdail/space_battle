use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::{
    na::Vector2,
    physics::{
        ColliderBundle, ColliderPositionSync, RapierConfiguration, RigidBodyBundle,
        RigidBodyPositionSync,
    },
    prelude::{
        ActiveEvents, ColliderMaterial, ColliderShape, ColliderType, RigidBodyForces,
        RigidBodyMassProps, RigidBodyMassPropsFlags, RigidBodyVelocity, RigidBodyVelocityComponent,
    },
};

use crate::{
    combat::{Bullet, Health, Weapon, WeaponSlot, WeaponSlots},
    Lifetime, Owner,
};

#[derive(Component, Inspectable)]
pub struct Player {
    pub speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<Player>()
            .add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(player_shoot);
    }
}

fn spawn_player(mut commands: Commands, rapier_config: Res<RapierConfiguration>) {
    let player_size = Vec2::splat(32.0);
    let collider_size = player_size / rapier_config.scale;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(player_size),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(collider_size.x / 2.0, collider_size.y / 2.0).into(),
            material: ColliderMaterial {
                friction: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            mass_properties: RigidBodyMassProps {
                flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Player { speed: 200.0 })
        .insert(Health::new(1.0))
        .insert(Name::new("Player"))
        .insert(WeaponSlots {
            weapons: vec![
                WeaponSlot {
                    weapon: None,
                    position: Vec2::new(0.0, 20.0),
                },
                WeaponSlot {
                    weapon: None,
                    position: Vec2::new(-15.0, 20.0),
                },
                WeaponSlot {
                    weapon: None,
                    position: Vec2::new(15.0, 20.0),
                },
            ],
        });
}

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
