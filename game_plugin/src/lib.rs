mod components;
mod systems;

use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;
use components::{Player, Velocity};
use systems::moving_system;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<Direction>()
            .add_startup_system(spawn_player)
            .add_startup_system(spawn_camera)
            .add_system(moving_system);
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity(Vec3::new(5.0, 7.0, 0.0)));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
