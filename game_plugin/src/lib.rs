mod components;
mod systems;

use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;
use bevy_rapier2d::{
    physics::{ColliderBundle, ColliderPositionSync, RapierConfiguration, RigidBodyBundle},
    prelude::{
        ColliderMaterial, ColliderShape, RigidBodyMassProps, RigidBodyMassPropsFlags, RigidBodyType,
    },
    render::ColliderDebugRender,
};
use components::Player;
use systems::player_movement;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<Player>()
            .add_startup_system(spawn_player)
            .add_startup_system(spawn_bounds)
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
        .insert(Name::new("Player"));
}

fn spawn_bounds(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    rapier_config: Res<RapierConfiguration>,
) {
    let thickness = 32.0;
    let sizes = vec![
        Vec2::new(thickness, window.height - 2.0 * thickness),
        Vec2::new(thickness, window.height - 2.0 * thickness),
        Vec2::new(window.width, thickness),
        Vec2::new(window.width, thickness),
    ];
    let positions = vec![
        Vec2::new((-window.width + thickness) / 2.0, 0.0),
        Vec2::new((window.width - thickness) / 2.0, 0.0),
        Vec2::new(0.0, (-window.height + thickness) / 2.0),
        Vec2::new(0.0, (window.height - thickness) / 2.0),
    ];
    for (size, position) in sizes.into_iter().zip(positions.iter()) {
        let collider_size = size / rapier_config.scale;
        commands
            .spawn_bundle(RigidBodyBundle {
                body_type: RigidBodyType::Static.into(),
                position: [position.x, position.y].into(),
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
            .insert(ColliderPositionSync::Discrete)
            .insert(ColliderDebugRender::with_id(1))
            .insert(Name::new("Wall"));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
