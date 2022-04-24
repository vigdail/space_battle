use bevy::prelude::*;

use game_plugin::GamePlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        width: 1280.0,
        height: 720.0,
        resizable: false,
        ..default()
    })
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugin(GamePlugin)
    .run();
}
