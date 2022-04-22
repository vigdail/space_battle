#![allow(clippy::type_complexity)]

use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::{
    na::Vector2,
    physics::{ColliderBundle, ColliderPositionSync, RapierConfiguration, RigidBodyBundle},
    prelude::{
        ColliderMaterial, ColliderShape, RigidBodyForces, RigidBodyMassProps,
        RigidBodyMassPropsFlags, RigidBodyVelocityComponent,
    },
};

use crate::{
    combat::{Health, Scores, ShootEvent, WeaponSlot, WeaponSlots},
    states::GameState,
};

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

pub struct GameOverEvent {
    pub score: u32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        app.register_inspectable::<Player>();
        app.add_event::<GameOverEvent>()
            .add_system_set(SystemSet::on_enter(GameState::Countdown).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(player_movement)
                    .with_system(player_shoot)
                    .with_system(track_player_dead)
                    .with_system(handle_game_over),
            );
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
            position: [0.0, -150.0].into(),
            forces: RigidBodyForces {
                gravity_scale: 0.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Player { speed: 200.0 })
        .insert(Health::new(1))
        .insert(Name::new("Player"))
        .insert(WeaponSlots {
            slots: vec![
                WeaponSlot {
                    weapon: None,
                    transform: (Vec2::new(0.0, 20.0), 90.0f32.to_radians()).into(),
                },
                WeaponSlot {
                    weapon: None,
                    transform: (Vec2::new(-15.0, 20.0), 90.0f32.to_radians()).into(),
                },
                WeaponSlot {
                    weapon: None,
                    transform: (Vec2::new(15.0, 20.0), 45.0f32.to_radians()).into(),
                },
            ],
        })
        .insert(Scores::default());
}

pub fn track_player_dead(
    mut game_over_events: EventWriter<GameOverEvent>,
    players: Query<(&Health, Option<&Scores>), (With<Player>, Changed<Health>)>,
) {
    if let Ok((health, scores)) = players.get_single() {
        if health.is_dead() {
            game_over_events.send(GameOverEvent {
                score: scores.map(|scores| scores.amount).unwrap_or(0),
            });
        }
    }
}

pub fn handle_game_over(
    mut events: EventReader<GameOverEvent>,
    mut state: ResMut<State<GameState>>,
) {
    if events.iter().next().is_some() {
        state
            .set(GameState::GameOver)
            .expect("Unable to set state to GameOver");
    }
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
    mut shoot_events: EventWriter<ShootEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    players: Query<Entity, With<Player>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        for shooter in players.iter() {
            shoot_events.send(ShootEvent { shooter })
        }
    }
}
