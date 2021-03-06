#![allow(clippy::type_complexity)]
use bevy::{app::AppExit, prelude::*};

use crate::{
    loading::FontAssets,
    main_menu::{hide_ui, show_ui, StartGameEvent, NORMAL_BUTTON},
    player::GameOverEvent,
    states::GameState,
};

#[derive(Component, Clone, Copy)]
pub enum GameOverButton {
    Restart,
    Exit,
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
pub struct GameOverMenu;

impl From<GameOverButton> for String {
    fn from(tag: GameOverButton) -> Self {
        match tag {
            GameOverButton::Restart => "Restart".into(),
            GameOverButton::Exit => "Exit".into(),
        }
    }
}

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::GameOver).with_system(show_ui::<GameOverMenu>),
        )
        .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(handle_button_click))
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver).with_system(hide_ui::<GameOverMenu>),
        )
        .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(setup_ui))
        .add_system(handle_game_over_events);
    }
}

fn handle_game_over_events(
    mut events: EventReader<GameOverEvent>,
    mut score_text: Query<&mut Text, With<ScoreText>>,
) {
    for GameOverEvent { score } in events.iter() {
        if let Ok(mut text) = score_text.get_single_mut() {
            text.sections[0].value = format!("Scores: {}", score);
        }
    }
}

fn handle_button_click(
    interaction_query: Query<(&Interaction, &GameOverButton), (Changed<Interaction>, With<Button>)>,
    mut start_game_events: EventWriter<StartGameEvent>,
    mut exit_events: EventWriter<AppExit>,
) {
    for (interaction, tag) in interaction_query.iter() {
        if let Interaction::Clicked = *interaction {
            match tag {
                GameOverButton::Restart => start_game_events.send(StartGameEvent),
                GameOverButton::Exit => exit_events.send(AppExit),
            }
        }
    }
}

fn setup_ui(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                display: Display::None,
                position_type: PositionType::Absolute,
                ..default()
            },
            color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
            ..default()
        })
        .insert(GameOverMenu)
        .insert(Name::new("Game Over UI"))
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "GAME OVER",
                    TextStyle {
                        font: fonts.font.clone(),
                        font_size: 96.0,
                        color: Color::ORANGE_RED,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..default()
            });
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "SCORES: 0",
                        TextStyle {
                            font: fonts.font.clone(),
                            font_size: 72.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..default()
                })
                .insert(ScoreText);
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        padding: Rect {
                            top: Val::Px(20.0),
                            ..default()
                        },
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    spawn_button(parent, GameOverButton::Restart, fonts.font.clone());
                    spawn_button(parent, GameOverButton::Exit, fonts.font.clone());
                });
        });
}

fn spawn_button(parent: &mut ChildBuilder, tag: GameOverButton, font: Handle<Font>) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect {
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|button| {
            button.spawn_bundle(TextBundle {
                text: Text::with_section(
                    tag,
                    TextStyle {
                        font,
                        font_size: 40.0,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                ..default()
            });
        })
        .insert(tag);
}
