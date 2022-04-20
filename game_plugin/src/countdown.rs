use std::time::Duration;

use bevy::prelude::*;

use crate::{loading::FontAssets, states::GameState};

#[derive(Component)]
struct CountdownUITag;

#[derive(Component)]
struct CountdownText;

struct CountdownTimer(pub Timer);

impl CountdownTimer {
    pub fn from_seconds(seconds: f32) -> Self {
        Self(Timer::new(Duration::from_secs_f32(seconds), false))
    }
}

pub struct CountdownPlugin;

impl Plugin for CountdownPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Countdown).with_system(setup_countdown))
            .add_system_set(SystemSet::on_update(GameState::Countdown).with_system(track_countdown))
            .add_system_set(
                SystemSet::on_exit(GameState::Countdown).with_system(despawn_countdown_ui),
            );
    }
}

fn setup_countdown(mut commands: Commands, fonts: Res<FontAssets>) {
    commands.insert_resource(CountdownTimer::from_seconds(3.0));

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "0",
                        TextStyle {
                            font: fonts.font.clone(),
                            font_size: 96.0,
                            color: Color::ORANGE_RED,
                        },
                        Default::default(),
                    ),
                    style: Style {
                        margin: Rect::all(Val::Auto),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CountdownText);
        })
        .insert(CountdownUITag);
}

fn despawn_countdown_ui(mut commands: Commands, ui: Query<Entity, With<CountdownUITag>>) {
    for entity in ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn track_countdown(
    time: Res<Time>,
    mut countdown: ResMut<CountdownTimer>,
    mut state: ResMut<State<GameState>>,
    mut text: Query<&mut Text, With<CountdownText>>,
) {
    if countdown.0.tick(time.delta()).just_finished() {
        state
            .set(GameState::Gameplay)
            .expect("Unable to change state to Gameplay");
    }
    if let Ok(mut text) = text.get_single_mut() {
        text.sections[0].value = (countdown.0.duration() - countdown.0.elapsed())
            .as_secs_f32()
            .ceil()
            .to_string();
    }
}
