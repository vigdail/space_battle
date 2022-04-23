mod combat;
mod countdown;
mod enemy;
mod game_over;
mod game_ui;
mod loading;
mod main_menu;
mod player;
mod states;

use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use heron::prelude::*;

use combat::CombatPlugin;
use countdown::CountdownPlugin;
use enemy::EnemyPlugin;
use game_over::GameOverPlugin;
use game_ui::GameUiPlugin;
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
            .add_plugin(GameUiPlugin)
            .add_startup_system(spawn_bounds)
            .add_startup_system(spawn_cameras)
            .add_system(track_lifetime);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

pub fn despawn_with<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_bounds(mut commands: Commands, window: Res<WindowDescriptor>) {
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
        commands
            .spawn_bundle(TransformBundle {
                local: Transform::from_translation(position.extend(0.0)),
                ..Default::default()
            })
            .insert(RigidBody::Static)
            .insert(CollisionShape::Cuboid {
                half_extends: size.extend(0.0) / 2.0,
                border_radius: None,
            })
            .insert(PhysicMaterial {
                friction: 0.0,
                ..Default::default()
            })
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
