use bevy::{app::AppExit, prelude::*};
#[cfg(feature = "debug")]
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::states::GameState;

#[derive(Component)]
#[cfg_attr(feature = "debug", derive(Inspectable))]
enum MenuButtonTag {
    Start,
    Exit,
}

#[derive(Component)]
struct MainMenuTag;

pub struct StartGameEvent;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        app.register_inspectable::<MenuButtonTag>();
        app.add_event::<StartGameEvent>()
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_main_menu))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(button_system)
                    .with_system(handle_start_game),
            )
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(despawn_main_menu));
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &MenuButtonTag),
        (Changed<Interaction>, With<Button>),
    >,
    mut start_game_events: EventWriter<StartGameEvent>,
    mut exit_events: EventWriter<AppExit>,
) {
    for (interaction, mut color, tag) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match tag {
                    MenuButtonTag::Start => start_game_events.send(StartGameEvent),
                    MenuButtonTag::Exit => {
                        exit_events.send(AppExit);
                    }
                }
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

fn handle_start_game(mut events: EventReader<StartGameEvent>, mut state: ResMut<State<GameState>>) {
    if events.iter().next().is_some() {
        state
            .set(GameState::Gameplay)
            .expect("Unable to change state to Gameplay");
    }
}

fn despawn_main_menu(mut commands: Commands, menu_query: Query<Entity, With<MainMenuTag>>) {
    for menu in menu_query.iter() {
        commands.entity(menu).despawn_recursive();
    }
}

fn setup_main_menu(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            spawn_button(parent, "Start", MenuButtonTag::Start);
            spawn_button(parent, "Exit", MenuButtonTag::Exit);
        })
        .insert(MainMenuTag);
}

fn spawn_button(parent: &mut ChildBuilder, text: &str, tag: MenuButtonTag) {
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
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .with_children(|button| {
            button.spawn_bundle(TextBundle {
                text: Text::with_section(
                    text,
                    TextStyle {
                        // TODO: load font
                        font_size: 40.0,
                        color: Color::BLACK.into(),
                        ..Default::default()
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(tag);
}
