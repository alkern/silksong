use crate::core::model::{Note, Trigger, TriggerState, TriggerType, UnplayedNotes};
use crate::music::model::{NaturalMinorScale, Scale};
use crate::state::GameState;
use bevy::color::palettes::css::BLUE_VIOLET;
use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_svg::prelude::{Svg, Svg2d};

pub struct CoreGamePlugin;

impl Plugin for CoreGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CoreAssets>()
            .add_event::<NotePlayedEvent>()
            .add_event::<TriggerActivatedEvent>()
            .add_systems(
                Update,
                (
                    activate_trigger,
                    check_and_play_notes::<NaturalMinorScale>,
                    draw_triggers,
                    handle_note_played,
                    check_all_played,
                )
                    .run_if(in_state(GameState::Execute))
                    .chain(),
            )
            .add_systems(OnEnter(GameState::Execute), (enter_execution,))
            .add_systems(
                OnExit(GameState::Execute),
                (exit_execution, update_trigger_icon_to_play_exit_execution),
            );
    }
}

#[derive(Resource)]
pub struct LevelConfig<T>
where
    T: Scale,
{
    pub grow_factor: f32,
    pub scale: T,
}

#[derive(Resource)]
pub struct CoreAssets {
    pub note_icon: Handle<Svg>,
    pub trigger_icon_pause: Handle<Svg>,
    pub trigger_icon_play: Handle<Svg>,
}

impl FromWorld for CoreAssets {
    fn from_world(world: &mut World) -> Self {
        CoreAssets {
            note_icon: world.load_asset("icons/music-solid.svg"),
            trigger_icon_pause: world.load_asset("icons/circle-pause-regular.svg"),
            trigger_icon_play: world.load_asset("icons/circle-play-regular.svg"),
        }
    }
}

#[derive(Event, Debug)]
pub struct NotePlayedEvent {
    pub note: Entity,
    pub trigger: Entity,
}

#[derive(Event, Debug)]
pub struct TriggerActivatedEvent {
    pub trigger: Entity,
}

/// Set the model data up for one execution. We keep some data in memory to simplify calculations.
fn enter_execution(
    triggers: Query<(Entity, &Trigger)>,
    notes: Query<(Entity, &Note)>,
    mut commands: Commands,
    mut activate_triggers: EventWriter<TriggerActivatedEvent>,
) {
    for (entity, trigger) in &triggers {
        if trigger.trigger_type == TriggerType::Main {
            activate_triggers.write(TriggerActivatedEvent { trigger: entity });
        }

        let Ok(mut trigger) = commands.get_entity(entity) else {
            continue;
        };

        let x = notes.into_iter().map(|(entity, _)| entity).collect();
        trigger.insert(UnplayedNotes(x));
    }
}

/// Clear the game state after an execution.
fn exit_execution(mut triggers: Query<(Entity, &mut Trigger)>, mut commands: Commands) {
    for (entity, mut trigger) in &mut triggers {
        trigger.size = 0.0;
        trigger.state = TriggerState::Inactive;

        let Ok(mut trigger) = commands.get_entity(entity) else {
            continue;
        };
        trigger.remove::<UnplayedNotes>();
    }
}

/// The core game logic: check if as trigger hits a note.
fn check_and_play_notes<T>(
    triggers: Query<(Entity, &mut Trigger, &Transform)>,
    unplayed_objects: Query<&UnplayedNotes>,
    notes: Query<(&Note, &Transform)>,
    config: Res<LevelConfig<T>>,
    time: Res<Time>,
    mut play_note_events: EventWriter<NotePlayedEvent>,
) where
    T: Scale,
{
    for (entity, mut trigger, trigger_position) in triggers {
        // update trigger state
        if trigger.state == TriggerState::Inactive {
            continue;
        }
        trigger.size += time.delta().as_secs_f32() * config.grow_factor;

        // check if any notes should be played this frame
        let Ok(unplayed_notes_of_trigger) = unplayed_objects.get(entity) else {
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

        // check if any other triggers are hit
    }
}

/// Visualize the size of each trigger.
fn draw_triggers(mut gizmos: Gizmos, triggers: Query<(&Trigger, &Transform)>) {
    for (trigger, transform) in &triggers {
        let position = Isometry2d::from_translation(transform.translation.xy());

        gizmos
            .circle_2d(position, trigger.size, BLUE_VIOLET)
            .resolution(64);
    }
}

/// If a trigger hits a note, this note is played and removed from the triggers list of unplayed notes.
fn handle_note_played(
    mut note_played_events: EventReader<NotePlayedEvent>,
    mut notes: Query<&mut UnplayedNotes>,
) {
    // TODO refactor this whole thing
    let mut to_remove: HashMap<Entity, Vec<Entity>> = HashMap::new();
    for event in note_played_events.read() {
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
            if !notes_to_remove.contains(note) {
                notes_to_keep.push(*note);
            }
        }

        notes.0 = notes_to_keep;
    }
}

/// After all notes are played the execution is done.
fn check_all_played(
    notes: Query<&UnplayedNotes>,
    // mut next_state: ResMut<NextState<GameState>>
) {
    for note in notes.iter() {
        if !note.0.is_empty() {
            return;
        }
    }
    //TODO stop execution

    // next_state.set(GameState::Over);
}

/// Set all trigger icons to play when leaving execution.
fn update_trigger_icon_to_play_exit_execution(
    assets: Res<CoreAssets>,
    trigger: Query<(Entity, &Trigger)>,
    mut commands: Commands,
) {
    for (entity, _) in &trigger {
        update_icon(assets.trigger_icon_play.clone(), entity, &mut commands);
    }
}

fn activate_trigger(
    mut events: EventReader<TriggerActivatedEvent>,
    mut query: Query<&mut Trigger>,
    assets: Res<CoreAssets>,
    mut commands: Commands,
) {
    for event in events.read() {
        if let Ok(mut trigger) = query.get_mut(event.trigger) {
            trigger.state = TriggerState::Active;
            update_icon(
                assets.trigger_icon_pause.clone(),
                event.trigger,
                &mut commands,
            );
        }
    }
}

/// Helper to set an icon on a trigger.
fn update_icon(asset: Handle<Svg>, trigger: Entity, commands: &mut Commands) {
    let Ok(mut trigger) = commands.get_entity(trigger) else {
        return;
    };
    trigger.insert(Svg2d(asset));
}
