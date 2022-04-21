use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::physics::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
#[cfg(feature = "debug")]
use bevy_rapier2d::render::RapierRenderPlugin;
use game_plugin::GamePlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        width: 1280.0,
        height: 720.0,
        resizable: false,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(RapierConfiguration {
        scale: 32.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::default())
        .add_plugin(RapierRenderPlugin);
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(GamePlugin)
        .run();
}
