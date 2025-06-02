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
use std::f32::consts::PI;

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
    fn build_note(position: Vec2, core_assets: &Res<CoreAssets>) -> impl Bundle {
        (
            Name::new("Note"),
            Note,
            Transform::from_translation(position.extend(0.0)).with_scale(Vec3::splat(0.025)),
            Svg2d(core_assets.note_icon.clone()),
            Origin::Center,
        )
    }

    commands.spawn(build_note(Vec2::from_angle(0.0) * 50.0, &core_assets));
    commands.spawn(build_note(
        Vec2::from_angle(1. / 3. * PI) * 100.0,
        &core_assets,
    ));
    commands.spawn(build_note(
        Vec2::from_angle(2. / 3. * PI) * 150.0,
        &core_assets,
    ));
    commands.spawn(build_note(
        Vec2::from_angle(3. / 3. * PI) * 200.0,
        &core_assets,
    ));
    commands.spawn(build_note(
        Vec2::from_angle(4. / 3. * PI) * 250.0,
        &core_assets,
    ));
    commands.spawn(build_note(
        Vec2::from_angle(5. / 3. * PI) * 300.0,
        &core_assets,
    ));
    commands.spawn(build_note(
        Vec2::from_angle(6. / 3. * PI) * 350.0,
        &core_assets,
    ));

    commands.spawn((
        Name::new("Trigger"),
        Trigger::default(),
        Transform::default().with_scale(Vec3::splat(0.05)),
        Svg2d(core_assets.trigger_icon_play.clone()),
        Origin::Center,
    ));

    info!("demo entities setup")
}
