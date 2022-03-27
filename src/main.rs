use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use game_plugin::GamePlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        width: 800.0,
        height: 600.0,
        resizable: false,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::default());
    app.add_plugin(GamePlugin).run();
}
