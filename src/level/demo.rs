use crate::core::{CoreAssets, LevelConfig, Note, Trigger};
use crate::state::GameState;
use bevy::prelude::*;

pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app
            // set up level
            .add_computed_state::<DemoLevelState>()
            .add_systems(
                OnEnter(GameState::SetupResources),
                setup_config.run_if(in_state(DemoLevelState::Setup)),
            )
            .add_systems(
                OnEnter(GameState::SetupGameObjects),
                setup_entities.run_if(in_state(DemoLevelState::Setup)),
            );
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DemoLevelState {
    Off,
    #[default]
    Setup,
    On,
}

impl ComputedStates for DemoLevelState {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            GameState::SetupResources => DemoLevelState::Setup,
            GameState::SetupGameObjects => DemoLevelState::Setup,
            GameState::Build => DemoLevelState::On,
            GameState::Execute => DemoLevelState::On,
            GameState::Over => DemoLevelState::Off,
        }
        .into()
    }
}

fn setup_config(mut commands: Commands) {
    commands.insert_resource(LevelConfig { grow_factor: 100.0 });
}

fn setup_entities(mut commands: Commands, core_assets: Res<CoreAssets>) {
    commands.spawn((
        Name::new("First Demo Note"),
        Note,
        Transform::from_xyz(50.0, 50.0, 0.0),
        Mesh2d::from(core_assets.note_form.clone()),
        MeshMaterial2d(core_assets.note_material.clone()),
    ));

    commands.spawn((
        Name::new("First Demo Player"),
        Trigger::default(),
        Transform::default(),
        Mesh2d::from(core_assets.trigger_form.clone()),
        MeshMaterial2d(core_assets.trigger_material.clone()),
    ));

    info!("demo entities setup")
}
