use crate::core::{LevelConfig, Note, NotePlayedEvent, Trigger};
use crate::math::calculate_scale_position_by_angle;
use crate::music::model::{NaturalMinorScale, Scale};
use crate::state::GameState;
use bevy::prelude::*;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_note_played::<NaturalMinorScale>.run_if(in_state(GameState::Execute)),
        );
    }
}

fn handle_note_played<T>(
    mut note_played_events: EventReader<NotePlayedEvent>,
    triggers: Query<(&Trigger, &Transform)>,
    notes: Query<(&Note, &Transform)>,
    level: Res<LevelConfig<T>>,
) where
    T: Scale,
{
    for event in note_played_events.read() {
        let Ok((_, trigger)) = triggers.get(event.trigger) else {
            continue;
        };
        let Ok((_, note)) = notes.get(event.note) else {
            continue;
        };

        let index = calculate_scale_position_by_angle(
            &trigger.translation.xy(),
            &note.translation.xy(),
            &level.scale,
        );
        let played = level.scale.get(index);

        info!("playing note {:?}", played);
    }
}
