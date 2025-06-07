use crate::core::game::{LevelConfig, NotePlayedEvent};
use crate::core::model::{Note, Trigger};
use crate::math::calculate_scale_position_by_angle;
use crate::music::audio::PianoAudioAssets;
use bevy::prelude::*;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_note_played);
    }
}

fn handle_note_played(
    mut note_played_events: EventReader<NotePlayedEvent>,
    triggers: Query<(&Trigger, &Transform)>,
    notes: Query<(&Note, &Transform)>,
    level: Res<LevelConfig>,
    piano: Res<PianoAudioAssets>,
    mut commands: Commands,
) {
    for event in note_played_events.read() {
        let Ok((_, trigger)) = triggers.get(event.source) else {
            continue;
        };
        let Ok((_, note)) = notes.get(event.note) else {
            continue;
        };

        // calculate the note from angle
        let index = calculate_scale_position_by_angle(
            &trigger.translation.xy(),
            &note.translation.xy(),
            &*level.scale,
        );
        let played = level.scale.get(index);

        // play note
        commands.spawn((
            Name::new("Note"),
            AudioPlayer(piano.play(played)),
            PlaybackSettings::DESPAWN,
        ));
    }
}
