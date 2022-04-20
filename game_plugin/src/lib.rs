mod combat;
mod countdown;
mod enemy;
mod game_over;
mod loading;
mod main_menu;
mod player;
mod states;

use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::{
    physics::{ColliderBundle, ColliderPositionSync, RapierConfiguration, RigidBodyBundle},
    prelude::{ColliderMaterial, ColliderShape, RigidBodyType},
    render::ColliderDebugRender,
};

use combat::CombatPlugin;
use countdown::CountdownPlugin;
use enemy::EnemyPlugin;
use game_over::GameOverPlugin;
use loading::LoadingPlugin;
use main_menu::MainMenuPlugin;
use player::PlayerPlugin;
use states::GameState;

#[derive(Component)]
#[cfg_attr(feature = "debug", derive(Inspectable))]
pub struct Owner {
    pub entity: Entity,
}

#[derive(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        app.register_inspectable::<Owner>();
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(CountdownPlugin)
            .add_plugin(GameOverPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(CombatPlugin)
            .add_plugin(EnemyPlugin)
            .add_startup_system(spawn_bounds)
            .add_startup_system(spawn_cameras)
            .add_system(track_lifetime);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
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

fn spawn_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
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
