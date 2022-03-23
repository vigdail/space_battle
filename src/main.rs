use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use game_plugin::GamePlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::default());
    app.add_plugin(GamePlugin).run();
}
