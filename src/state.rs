use bevy::prelude::*;

/// State for the application.
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    /// Setup of the whole application
    #[default]
    Setup,
    Menu,
    /// In game state for the player: [`GameState`]
    Game,
}

/// In-game state
#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[source(AppState=AppState::Game)]
pub enum GameState {
    /// Set up resources and assets
    #[default]
    SetupResources,
    /// Set up the game objects which may depend on the resources from [`GameState::SetupResources`]
    SetupGameObjects,
    /// game is running
    Prepare,
    /// game is paused
    Play,
    /// game is over
    Over,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>().add_sub_state::<GameState>();
    }
}
