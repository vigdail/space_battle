use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
use game_plugin::GamePlugin;
use heron::prelude::*;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        width: 1280.0,
        height: 720.0,
        resizable: false,
        ..default()
    })
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::default());
    app.add_plugin(PhysicsPlugin::default())
        .add_plugin(GamePlugin)
        .run();
}
