use crate::state::GameState;
use bevy::prelude::*;
use std::ops::Deref;

pub struct CoreGamePlugin;

impl Plugin for CoreGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            check_and_play_notes.run_if(in_state(GameState::Play)),
        );
    }
}

#[derive(Resource)]
pub struct LevelConfig {
    grow_factor: f64,
}

impl Default for LevelConfig {
    fn default() -> Self {
        LevelConfig { grow_factor: 10.0 }
    }
}

#[derive(Component)]
pub struct Note;

#[derive(Component)]
pub enum NoteState {
    Ready,
    Finished,
}

#[derive(Component)]
pub struct Player {
    size: f64,
}

#[derive(Component, Deref)]
pub struct UnplayedNotes(Vec<Note>);

pub fn check_and_play_notes(
    mut players: Query<(&mut Player, &UnplayedNotes)>,
    config: Res<LevelConfig>,
    time: Res<Time>,
) {
    for (mut player, notes) in players {
        player.size += time.delta().as_secs_f64() * config.grow_factor;

        for _ in notes.deref() {
            // if distance to note is size play note
        }
    }
}
