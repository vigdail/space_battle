use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};

struct AudioState {
    sfx: ChannelState,
}

impl AudioState {
    fn new() -> Self {
        Self {
            sfx: ChannelState::new("sfx".into()),
        }
    }
}

struct ChannelState {
    channel: AudioChannel,
    volume: f32,
}

impl ChannelState {
    fn new(key: String) -> Self {
        Self {
            channel: AudioChannel::new(key),
            volume: 1.0,
        }
    }
}

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AudioState::new())
            .add_plugin(AudioPlugin)
            .add_system(button_sounds)
            .add_system(change_volume);
    }
}

fn button_sounds(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_state: Res<AudioState>,
    interaction: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in interaction.iter() {
        match *interaction {
            Interaction::Hovered => {
                audio.play_in_channel(
                    asset_server.get_handle("sounds/button_hover.wav"),
                    &audio_state.sfx.channel,
                );
            }
            _ => {}
        }
    }
}

fn change_volume(
    input: Res<Input<KeyCode>>,
    mut audio_state: ResMut<AudioState>,
    audio: Res<Audio>,
) {
    if input.just_pressed(KeyCode::Up) {
        audio_state.sfx.volume = (audio_state.sfx.volume + 0.1).clamp(0.0, 1.0);
    }
    if input.just_pressed(KeyCode::Down) {
        audio_state.sfx.volume = (audio_state.sfx.volume - 0.1).clamp(0.0, 1.0);
    }

    if audio_state.is_changed() {
        audio.set_volume_in_channel(audio_state.sfx.volume, &audio_state.sfx.channel);
        println!("Volume changed: {}", audio_state.sfx.volume);
    }
}
