use crate::music::model::{NaturalMinorScale, Scale};
use crate::state::GameState;
use bevy::color::palettes::css::FUCHSIA;
use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;

pub struct CoreGamePlugin;

impl Plugin for CoreGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CoreAssets>()
            .add_event::<NotePlayedEvent>()
            .add_systems(
                Update,
                (
                    check_and_play_notes::<NaturalMinorScale>,
                    draw_triggers,
                    handle_note_played,
                    check_all_played,
                )
                    .run_if(in_state(GameState::Execute))
                    .chain(),
            )
            .add_systems(OnEnter(GameState::Execute), enter_execution)
            .add_systems(OnExit(GameState::Execute), exit_execution);
    }
}

#[derive(Resource)]
pub struct LevelConfig<T>
where
    T: Scale + Sync,
{
    pub grow_factor: f32,
    pub scale: T,
}

#[derive(Resource)]
pub struct CoreAssets {
    // TODO material active/inactive
    pub note_material: Handle<ColorMaterial>,
    pub note_form: Mesh2d,
    // TODO trigger icon
    pub trigger_material: Handle<ColorMaterial>,
    pub trigger_form: Mesh2d,
}

impl FromWorld for CoreAssets {
    fn from_world(world: &mut World) -> Self {
        CoreAssets {
            note_material: world.add_asset(ColorMaterial {
                color: Color::srgb(0.0, 0.1, 0.05),
                ..default()
            }),
            note_form: world.add_asset(Circle::new(10.0)).into(),
            trigger_material: world.add_asset(ColorMaterial {
                color: Color::srgb(0.6, 0.0, 0.0),
                ..default()
            }),
            trigger_form: world.add_asset(Circle::new(10.0)).into(),
        }
    }
}

#[derive(Component)]
pub struct Note;

#[derive(Component, Default, Debug)]
pub struct Trigger {
    size: f32,
}

#[derive(Component, Deref)]
struct UnplayedNotes(Vec<Entity>);

#[derive(Event, Debug)]
pub struct NotePlayedEvent {
    pub note: Entity,
    pub trigger: Entity,
}

fn enter_execution(
    triggers: Query<(Entity, &Trigger)>,
    notes: Query<(Entity, &Note)>,
    mut commands: Commands,
) {
    for (entity, _) in &triggers {
        let Ok(mut trigger) = commands.get_entity(entity) else {
            continue;
        };

        let x = notes.into_iter().map(|(entity, _)| entity).collect();
        trigger.insert(UnplayedNotes(x));
    }
}

fn exit_execution(mut triggers: Query<(Entity, &mut Trigger)>, mut commands: Commands) {
    for (entity, mut trigger) in &mut triggers {
        trigger.size = 0.0;

        let Ok(mut trigger) = commands.get_entity(entity) else {
            continue;
        };
        trigger.remove::<UnplayedNotes>();
    }
}

/// The core game logic: check if as trigger hits a note.
fn check_and_play_notes<T>(
    triggers: Query<(Entity, &mut Trigger, &Transform)>,
    unplayed_notes: Query<&UnplayedNotes>,
    notes: Query<(&Note, &Transform)>,
    config: Res<LevelConfig<T>>,
    time: Res<Time>,
    mut play_note_events: EventWriter<NotePlayedEvent>,
) where
    T: Scale + Send + Sync + 'static,
{
    for (entity, mut trigger, trigger_position) in triggers {
        trigger.size += time.delta().as_secs_f32() * config.grow_factor;

        let Ok(unplayed_notes_of_trigger) = unplayed_notes.get(entity) else {
            continue;
        };

        for unplayed_note in unplayed_notes_of_trigger.0.clone() {
            let Ok((_, position)) = notes.get(unplayed_note) else {
                continue;
            };

            if position
                .translation
                .xy()
                .distance(trigger_position.translation.xy())
                < trigger.size
            {
                play_note_events.write(NotePlayedEvent {
                    note: unplayed_note,
                    trigger: entity,
                });
            }
        }
    }
}

fn draw_triggers(mut gizmos: Gizmos, triggers: Query<&Trigger>) {
    for trigger in &triggers {
        gizmos.circle_2d(Isometry2d::IDENTITY, trigger.size, FUCHSIA);
    }
}

fn handle_note_played(
    mut note_played_events: EventReader<NotePlayedEvent>,
    mut notes: Query<&mut UnplayedNotes>,
) {
    // TODO refactor this whole thing
    let mut to_remove: HashMap<Entity, Vec<Entity>> = HashMap::new();
    for event in note_played_events.read() {
        info!("note {:?} played <3", event);

        match to_remove.get_mut(&event.trigger) {
            None => {
                to_remove.insert(event.trigger, vec![event.note]);
            }
            Some(values) => {
                values.add(event.note);
            }
        };
    }

    for (trigger, notes_to_remove) in to_remove {
        let Ok(mut notes) = notes.get_mut(trigger) else {
            continue;
        };

        let mut notes_to_keep = Vec::new();
        for note in &notes.0 {
            if !notes_to_remove.contains(&note) {
                notes_to_keep.push(note.clone());
            }
        }

        notes.0 = notes_to_keep;
    }
}

fn check_all_played(
    notes: Query<&UnplayedNotes>,
    // mut _: ResMut<NextState<GameState>>
) {
    for note in notes.iter() {
        if !note.0.is_empty() {
            return;
        }
    }
    //TODO when all is played return to another state after a short while
    // next_state.set(GameState::Over);
}
