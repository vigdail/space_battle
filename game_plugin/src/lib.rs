use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(print_info);
    }
}

fn print_info() {
    println!("Game plugin start");
}
