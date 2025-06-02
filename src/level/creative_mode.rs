//! # Creative Mode
//!
//! Free Mode in which every element can be placed and playing is possible at all times.
//! This is the goal for the game jam, a tutorial and puzzle mode would be even cooler, but not
//! possible in the time.

use crate::core::game::{CoreAssets, LevelConfig};
use crate::core::model::{Note, Trigger};
use crate::music::model::NaturalMinorScale;
use crate::state::GameState;
use bevy::prelude::*;
use bevy_svg::prelude::{Origin, Svg2d};

pub struct CreativeModePlugin;

impl Plugin for CreativeModePlugin {
    fn build(&self, app: &mut App) {
        app
            // set up level
            .add_computed_state::<CreativeModeState>()
            .add_systems(
                OnEnter(GameState::SetupResources),
                setup_config.run_if(in_state(CreativeModeState::Setup)),
            )
            .add_systems(
                OnEnter(GameState::SetupGameObjects),
                setup_entities.run_if(in_state(CreativeModeState::Setup)),
            );
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CreativeModeState {
    Off,
    #[default]
    Setup,
    On,
}

impl ComputedStates for CreativeModeState {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            GameState::SetupResources => CreativeModeState::Setup,
            GameState::SetupGameObjects => CreativeModeState::Setup,
            GameState::Build => CreativeModeState::On,
            GameState::Execute => CreativeModeState::On,
            GameState::Over => CreativeModeState::Off,
        }
        .into()
    }
}

fn setup_config(mut commands: Commands) {
    commands.insert_resource(LevelConfig {
        grow_factor: 100.0,
        scale: NaturalMinorScale::new(crate::music::model::Note::A),
    });
}

fn setup_entities(mut commands: Commands, core_assets: Res<CoreAssets>) {
    fn build_note(x: f32, y: f32, core_assets: &Res<CoreAssets>) -> impl Bundle {
        (
            Name::new("Note"),
            Note,
            Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(0.025)),
            Svg2d(core_assets.note_icon.clone()),
            Origin::Center,
        )
    }

    commands.spawn(build_note(0.0, 500.0, &core_assets));
    commands.spawn(build_note(101.0, 401.0, &core_assets));
    commands.spawn(build_note(202.0, 302.0, &core_assets));
    commands.spawn(build_note(303.0, 203.0, &core_assets));
    commands.spawn(build_note(404.0, 104.0, &core_assets));
    commands.spawn(build_note(505.0, 0.0, &core_assets));
    commands.spawn(build_note(406.0, -105.0, &core_assets));
    commands.spawn(build_note(307.0, -206.0, &core_assets));
    commands.spawn(build_note(208.0, -307.0, &core_assets));
    commands.spawn(build_note(109.0, -408.0, &core_assets));
    commands.spawn(build_note(0.0, -509.0, &core_assets));
    commands.spawn(build_note(-110.0, -410.0, &core_assets));
    commands.spawn(build_note(-211.0, -311.0, &core_assets));
    commands.spawn(build_note(-312.0, -212.0, &core_assets));
    commands.spawn(build_note(-413.0, -113.0, &core_assets));
    commands.spawn(build_note(-514.0, 0.0, &core_assets));
    commands.spawn(build_note(-415.0, 114.0, &core_assets));
    commands.spawn(build_note(-316.0, 215.0, &core_assets));
    commands.spawn(build_note(-217.0, 316.0, &core_assets));
    commands.spawn(build_note(-118.0, 417.0, &core_assets));

    commands.spawn((
        Name::new("Trigger"),
        Trigger::default(),
        Transform::default().with_scale(Vec3::splat(0.05)),
        Svg2d(core_assets.trigger_icon_play.clone()),
        Origin::Center,
    ));

    info!("demo entities setup")
}
