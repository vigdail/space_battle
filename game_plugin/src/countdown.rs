use std::time::Duration;

use bevy::prelude::*;

use crate::states::GameState;

struct Countdown(pub Timer);

impl Countdown {
    pub fn from_seconds(seconds: f32) -> Self {
        Self(Timer::new(Duration::from_secs_f32(seconds), false))
    }
}

pub struct CountdownPlugin;

impl Plugin for CountdownPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Countdown).with_system(setup_countdown))
            .add_system_set(
                SystemSet::on_update(GameState::Countdown).with_system(track_countdown),
            );
    }
}

fn setup_countdown(mut commands: Commands) {
    commands.insert_resource(Countdown::from_seconds(3.0));
}

fn track_countdown(
    time: Res<Time>,
    mut countdown: ResMut<Countdown>,
    mut state: ResMut<State<GameState>>,
) {
    if countdown.0.tick(time.delta()).just_finished() {
        state
            .set(GameState::Gameplay)
            .expect("Unable to change state to Gameplay");
    }
}
