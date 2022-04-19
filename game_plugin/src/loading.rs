use bevy::prelude::*;

use crate::states::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Loading).with_system(load_assets));
    }
}

// TODO
fn load_assets(mut states: ResMut<State<GameState>>) {
    states
        .set(GameState::MainMenu)
        .expect("Unable to switch game state to MainMenu");
}
