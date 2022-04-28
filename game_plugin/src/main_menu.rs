#![allow(clippy::type_complexity)]

use bevy::{app::AppExit, prelude::*};
#[cfg(feature = "debug")]
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::{loading::FontAssets, states::GameState};

#[derive(Component)]
#[cfg_attr(feature = "debug", derive(Inspectable))]
enum MenuButtonTag {
    Start,
    Exit,
}

#[derive(Component)]
struct MainMenuTag;

pub struct StartGameEvent;

pub const NORMAL_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const HOVERED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
pub const PRESSED_BUTTON: Color = Color::rgb(0.45, 0.75, 0.45);

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        app.register_inspectable::<MenuButtonTag>();
        app.add_event::<StartGameEvent>()
            .add_system_set(
                SystemSet::on_enter(GameState::MainMenu).with_system(show_ui::<MainMenuTag>),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(handle_button_click)
                    .with_system(handle_keyboard),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::MainMenu).with_system(hide_ui::<MainMenuTag>),
            )
            .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(setup_main_menu))
            .add_system(handle_start_game)
            .add_system(button_color_system);
    }
}

fn button_color_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn handle_button_click(
    interaction_query: Query<(&Interaction, &MenuButtonTag), (Changed<Interaction>, With<Button>)>,
    mut start_game_events: EventWriter<StartGameEvent>,
    mut exit_events: EventWriter<AppExit>,
) {
    for (interaction, tag) in interaction_query.iter() {
        if let Interaction::Clicked = *interaction {
            match tag {
                MenuButtonTag::Start => start_game_events.send(StartGameEvent),
                MenuButtonTag::Exit => exit_events.send(AppExit),
            }
        }
    }
}

fn handle_keyboard(
    keyboard_input: Res<Input<KeyCode>>,
    mut start_game_events: EventWriter<StartGameEvent>,
    mut exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        start_game_events.send(StartGameEvent);
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit_events.send(AppExit);
    }
}

pub fn show_ui<T: Component>(mut ui: Query<&mut Style, With<T>>) {
    for mut style in ui.iter_mut() {
        style.display = Display::Flex;
    }
}

pub fn hide_ui<T: Component>(mut ui: Query<&mut Style, With<T>>) {
    for mut style in ui.iter_mut() {
        style.display = Display::None;
    }
}

fn handle_start_game(mut events: EventReader<StartGameEvent>, mut state: ResMut<State<GameState>>) {
    if events.iter().next().is_some() {
        state
            .set(GameState::Countdown)
            .expect("Unable to change state to Gameplay");
    }
}

fn setup_main_menu(mut commands: Commands, fonts: Res<FontAssets>) {
    let font = fonts.font.clone();

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                display: Display::None,
                position_type: PositionType::Absolute,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            spawn_button(parent, "Start", MenuButtonTag::Start, font.clone());
            spawn_button(parent, "Exit", MenuButtonTag::Exit, font);
        })
        .insert(MainMenuTag)
        .insert(Name::new("Main Menu UI"));
}

fn spawn_button(parent: &mut ChildBuilder, text: &str, tag: MenuButtonTag, font: Handle<Font>) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect {
                    top: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    left: Val::Auto,
                    right: Val::Auto,
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
                    text,
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
