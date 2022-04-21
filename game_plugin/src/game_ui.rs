use bevy::prelude::*;

use crate::{
    combat::Scores,
    loading::FontAssets,
    main_menu::{hide_ui, show_ui},
    player::Player,
    states::GameState,
};

#[derive(Component)]
pub struct PlayerScoresText;

#[derive(Component)]
pub struct GameUITag;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Gameplay).with_system(show_ui::<GameUITag>),
        )
        .add_system_set(SystemSet::on_exit(GameState::Gameplay).with_system(hide_ui::<GameUITag>))
        .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(setup_ui))
        .add_system(display_scores);
    }
}

fn display_scores(
    mut text: Query<&mut Text, With<PlayerScoresText>>,
    scores: Query<&Scores, (Changed<Scores>, With<Player>)>,
) {
    if let Some((scores, mut text)) = scores.get_single().ok().zip(text.get_single_mut().ok()) {
        text.sections[0].value = format!("Score: {}", scores.amount);
    }
}

fn setup_ui(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                position_type: PositionType::Absolute,
                display: Display::None,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(GameUITag)
        .insert(Name::new("Game UI"))
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(32.0)),
                        flex_direction: FlexDirection::RowReverse,
                        align_items: AlignItems::FlexEnd,
                        padding: Rect::all(Val::Px(10.0)),
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Score: 0",
                                TextStyle {
                                    font: fonts.font.clone(),
                                    font_size: 26.0,
                                    color: Color::WHITE,
                                },
                                TextAlignment {
                                    vertical: VerticalAlign::Center,
                                    horizontal: HorizontalAlign::Right,
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(PlayerScoresText);
                });
        });
}
