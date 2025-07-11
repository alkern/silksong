use bevy::prelude::*;

/// State for the application.
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    /// Setup of the whole application
    // Setup,
    // Menu,
    #[default]
    /// In game state for the player: [`GameState`]
    Game,
}

/// In-game state
#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[source(AppState=AppState::Game)]
#[allow(dead_code)]
pub enum GameState {
    /// Set up resources and assets
    #[default]
    SetupResources,
    /// Set up the game objects which may depend on the resources from [`GameState::SetupResources`]
    SetupGameObjects,
    /// game is running in the build mode: arranging notes and more
    Build,
    /// game is running in the execution mode: the built setup is played
    Execute,
    /// game is over
    Over,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MinimalGameState {
    #[default]
    Setup,
    Running,
}

impl ComputedStates for MinimalGameState {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            GameState::SetupResources => MinimalGameState::Setup,
            GameState::SetupGameObjects => MinimalGameState::Setup,
            GameState::Build => MinimalGameState::Running,
            GameState::Execute => MinimalGameState::Running,
            GameState::Over => MinimalGameState::Running,
        }
        .into()
    }
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_sub_state::<GameState>()
            .add_computed_state::<MinimalGameState>()
            .add_systems(
                PostUpdate,
                (
                    resources_are_setup.run_if(in_state(GameState::SetupResources)),
                    game_objects_are_setup.run_if(in_state(GameState::SetupGameObjects)),
                ),
            );
    }
}

/// Run this to move further after resources for a level are set up.
fn resources_are_setup(mut next: ResMut<NextState<GameState>>) {
    next.set(GameState::SetupGameObjects);
    info!("resources are setup");
}

/// Run this to move further after game objects for a level are set up.
fn game_objects_are_setup(mut next: ResMut<NextState<GameState>>) {
    next.set(GameState::Build);
    info!("game objects are setup");
}
