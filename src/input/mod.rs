mod picker;
mod ui;

use crate::input::picker::{ManuallyPlaced, PickerPlugin};
use crate::input::ui::UiPlugin;
use crate::state::{AppState, GameState};
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PickerPlugin, UiPlugin))
            .add_systems(Update, close_on_esc)
            .add_systems(
                Update,
                handle_game_loop_input.run_if(in_state(AppState::Game)),
            );
    }
}

#[cfg(target_family = "wasm")]
fn close_on_esc() {
    // do nothing
}

#[cfg(not(target_family = "wasm"))]
fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

fn handle_game_loop_input(
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
    objects: Query<&ManuallyPlaced>,
) {
    if objects.iter().count() == 0 {
        // Nothing is placed yet, so we cannot execute.
        // Without this check the main trigger flickers in this case. Ugly!
        return;
    }

    if keys.just_pressed(KeyCode::Space) {
        match current_state.get() {
            GameState::SetupResources => {}
            GameState::SetupGameObjects => {}
            GameState::Build => next_state.set(GameState::Execute),
            GameState::Execute => next_state.set(GameState::Build),
            GameState::Over => {}
        }
    }
}
