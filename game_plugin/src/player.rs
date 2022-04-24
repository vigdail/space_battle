#![allow(clippy::type_complexity)]

use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use heron::prelude::*;

use crate::{
    combat::{Health, Scores, ShootEvent, UnitPrefab},
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

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_size = Vec2::splat(32.0);
    let prefab_handle: Handle<UnitPrefab> = asset_server.get_handle("units/player.ron");
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: player_size.extend(0.0) / 2.0,
            border_radius: None,
        })
        .insert(Velocity::default())
        .insert(RotationConstraints::lock())
        .insert(PhysicMaterial {
            friction: 0.0,
            ..default()
        })
        .insert(Player { speed: 200.0 })
        .insert(Scores::default())
        .insert(prefab_handle)
        .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(
            0.0, -150.0, 0.0,
        )));
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
    mut players: Query<(&Player, &mut Velocity)>,
) {
    for (player, mut velocity) in players.iter_mut() {
        let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let move_delta = Vec3::new(x_axis as f32, y_axis as f32, 0.0).normalize_or_zero();

        velocity.linear = move_delta * player.speed;
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
