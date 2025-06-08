use crate::core::game::{LevelConfig, NotePlayedEvent};
use crate::core::model::{Activator, Note};
use crate::math::calculate_scale_position_by_angle;
use crate::music::audio::PianoAudioAssets;
use bevy::audio::Volume;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActivePlayer>()
            .add_systems(Update, handle_note_played);
    }
}

#[derive(Resource, Default, Debug)]
struct ActivePlayer(HashMap<crate::music::model::Note, Entity>);

fn handle_note_played(
    mut note_played_events: EventReader<NotePlayedEvent>,
    activators: Query<(&Activator, &Transform)>,
    notes: Query<(&Note, &Transform)>,
    level: Res<LevelConfig>,
    piano: Res<PianoAudioAssets>,
    mut commands: Commands,
    mut active_player: ResMut<ActivePlayer>,
) {
    for event in note_played_events.read() {
        let Ok((_, activator)) = activators.get(event.source) else {
            continue;
        };
        let Ok((_, note)) = notes.get(event.note) else {
            continue;
        };

        // calculate the note from angle
        let index = calculate_scale_position_by_angle(
            &activator.translation.xy(),
            &note.translation.xy(),
            &*level.scale,
        );
        let played = level.scale.get(index);

        // if a player for the selected note exists, remove it
        match active_player.0.get(&played) {
            None => {}
            Some(id) => {
                if let Ok(mut e) = commands.get_entity(*id) {
                    e.insert(PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.0)));
                };
            }
        }

        // play note
        let id = commands
            .spawn((
                Name::new("Note"),
                AudioPlayer(piano.play(played)),
                PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.5)),
            ))
            .id();
        active_player.0.insert(played, id);
    }
}
