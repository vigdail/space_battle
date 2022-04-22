use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

use crate::states::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .continue_to_state(GameState::MainMenu)
            .with_collection::<FontAssets>()
            .with_collection::<AssetsFolder>()
            .build(app);
        app.add_system_set(SystemSet::on_enter(GameState::Loading).with_system(
            |asset_server: ResMut<AssetServer>| {
                println!("Loading...");
                asset_server
                    .watch_for_changes()
                    .unwrap_or_else(|err| info!("AssetServer unable to watch changes: {}", err));
            },
        ));
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/Boxfont Round.ttf")]
    pub font: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AssetsFolder {
    #[asset(path = "units", folder)]
    pub units: Vec<HandleUntyped>,
}
