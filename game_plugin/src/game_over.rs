use bevy::prelude::*;

use crate::states::GameState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(print_message));
    }
}

fn print_message() {
    info!("Game over");
}
