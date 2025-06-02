//! Technical implementation of musical stuff

use crate::state::{GameState, MinimalGameState};
use bevy::prelude::*;
use std::time::Duration;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BackgroundAudioAssets>()
            .add_systems(
                Update,
                setup_audio_objects.run_if(in_state(GameState::SetupGameObjects)),
            )
            .add_systems(OnEnter(MinimalGameState::Running), initial)
            .add_systems(
                Update,
                background.run_if(in_state(MinimalGameState::Running)),
            );
    }
}

#[derive(Resource)]
struct BackgroundAudioAssets {
    strings_1: Handle<AudioSource>,
    strings_2: Handle<AudioSource>,
}

impl FromWorld for BackgroundAudioAssets {
    fn from_world(world: &mut World) -> Self {
        BackgroundAudioAssets {
            strings_1: world.load_asset("audio/strings_Am_1_I_iv_VI_v.wav"),
            strings_2: world.load_asset("audio/strings_Am_2_I_iidim_v_VII.wav"),
        }
    }
}

/// Simple wrapper to count repetitions for the background strings.
struct BackgroundRepetition(u8);

impl BackgroundRepetition {
    fn new() -> BackgroundRepetition {
        BackgroundRepetition(0)
    }

    fn get_repeat(&self) -> u8 {
        self.0
    }

    fn proceed(&mut self) {
        self.0 = (self.0 + 1) % 4;
    }
}

#[derive(Component)]
struct BackgroundTimer(Timer, BackgroundRepetition);

impl BackgroundTimer {
    /// Constructs a new timer with the given duration in seconds.
    fn new() -> Self {
        BackgroundTimer(
            Timer::from_seconds(15.0, TimerMode::Repeating),
            BackgroundRepetition::new(),
        )
    }

    fn tick(&mut self, duration: Duration) -> Option<u8> {
        self.0.tick(duration);

        if self.0.just_finished() {
            info!("timer just finished");
            self.1.proceed();
            return Some(self.1.get_repeat());
        }
        None
    }
}

fn setup_audio_objects(mut commands: Commands) {
    commands.spawn((Name::new("Background Audio Timer"), BackgroundTimer::new()));
}

fn initial(mut commands: Commands, assets: Res<BackgroundAudioAssets>) {
    commands.spawn((
        Name::new("Background Audio"),
        AudioPlayer(assets.strings_1.clone()),
        PlaybackSettings::DESPAWN,
    ));
}

fn background(
    mut timer: Query<&mut BackgroundTimer>,
    time: Res<Time>,
    assets: Res<BackgroundAudioAssets>,
    mut commands: Commands,
) {
    let mut timer = timer.single_mut().expect("Background timer must exist");

    let tick = timer.tick(time.delta());

    match tick {
        None => {}
        Some(3) => {
            commands.spawn((
                Name::new("Background Audio"),
                AudioPlayer(assets.strings_2.clone()),
                PlaybackSettings::DESPAWN,
            ));
        }
        Some(_) => {
            commands.spawn((
                Name::new("Background Audio"),
                AudioPlayer(assets.strings_1.clone()),
                PlaybackSettings::DESPAWN,
            ));
        }
    }
}
