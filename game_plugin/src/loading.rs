use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

use crate::{combat::UnitDefs, states::GameState};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .continue_to_state(GameState::MainMenu)
            .with_collection::<FontAssets>()
            .with_collection::<UnitAssets>()
            .build(app);
        app.add_system_set(SystemSet::on_enter(GameState::Loading).with_system(|| {
            println!("Loading...");
        }));
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/Boxfont Round.ttf")]
    pub font: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct UnitAssets {
    #[asset(path = "units/dragon.ron")]
    pub dragon: Handle<UnitDefs>,
}
