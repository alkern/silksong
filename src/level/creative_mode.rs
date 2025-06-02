//! # Creative Mode
//!
//! Free Mode in which every element can be placed and playing is possible at all times.
//! This is the goal for the game jam, a tutorial and puzzle mode would be even cooler, but not
//! possible in the time.

use crate::core::{CoreAssets, LevelConfig, Note, Trigger};
use crate::music::model::NaturalMinorScale;
use crate::state::GameState;
use bevy::prelude::*;

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
            Transform::from_xyz(x, y, 0.0),
            Mesh2d::from(core_assets.note_form.clone()),
            MeshMaterial2d(core_assets.note_material.clone()),
        )
    }

    commands.spawn(build_note(0.0, 500.0, &core_assets));
    commands.spawn(build_note(100.0, 400.0, &core_assets));
    commands.spawn(build_note(200.0, 300.0, &core_assets));
    commands.spawn(build_note(300.0, 400.0, &core_assets));
    commands.spawn(build_note(400.0, 100.0, &core_assets));
    commands.spawn(build_note(500.0, 0.0, &core_assets));
    commands.spawn(build_note(400.0, -100.0, &core_assets));
    commands.spawn(build_note(300.0, -200.0, &core_assets));
    commands.spawn(build_note(200.0, -300.0, &core_assets));
    commands.spawn(build_note(100.0, -400.0, &core_assets));
    commands.spawn(build_note(0.0, -500.0, &core_assets));
    commands.spawn(build_note(-100.0, -400.0, &core_assets));
    commands.spawn(build_note(-200.0, -300.0, &core_assets));
    commands.spawn(build_note(-300.0, -200.0, &core_assets));
    commands.spawn(build_note(-400.0, -100.0, &core_assets));
    commands.spawn(build_note(-500.0, 0.0, &core_assets));
    commands.spawn(build_note(-400.0, 100.0, &core_assets));
    commands.spawn(build_note(-300.0, 200.0, &core_assets));
    commands.spawn(build_note(-200.0, 300.0, &core_assets));
    commands.spawn(build_note(-100.0, 400.0, &core_assets));

    commands.spawn((
        Name::new("First Demo Player"),
        Trigger::default(),
        Transform::default(),
        Mesh2d::from(core_assets.trigger_form.clone()),
        MeshMaterial2d(core_assets.trigger_material.clone()),
    ));

    info!("demo entities setup")
}
