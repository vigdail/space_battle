mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;
use bevy_rapier2d::{
    physics::{ColliderBundle, ColliderPositionSync, RapierConfiguration, RigidBodyBundle},
    prelude::{ColliderShape, ColliderShapeComponent, RigidBodyType, RigidBodyTypeComponent},
};
use components::Player;
use systems::player_movement;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<Player>()
            .add_startup_system(spawn_player)
            .add_startup_system(spawn_wall)
            .add_startup_system(spawn_camera)
            .add_system(player_movement);
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
            shape: ColliderShapeComponent(ColliderShape::cuboid(
                collider_size.x / 2.0,
                collider_size.y / 2.0,
            )),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Player { speed: 30.0 })
        .insert(Name::new("Player"));
}

fn spawn_wall(mut commands: Commands, rapier_config: Res<RapierConfiguration>) {
    let wall_size = Vec2::new(32.0, 128.0);
    let collider_size = wall_size / rapier_config.scale;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(wall_size),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Static),
            position: [32.0, 64.0].into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShapeComponent(ColliderShape::cuboid(
                collider_size.x / 2.0,
                collider_size.y / 2.0,
            )),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Name::new("Wall"));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
