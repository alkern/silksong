use crate::core::model::{
    Note, Trigger, TriggerColor, TriggerSize, TriggerState, TriggerType, UntriggeredObjects,
};
use crate::music::model::Scale;
use crate::state::GameState;
use bevy::prelude::*;
use bevy_svg::prelude::{Svg, Svg2d};
use std::cmp::Ordering;

pub struct CoreGamePlugin;

impl Plugin for CoreGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CoreAssets>()
            .add_event::<NotePlayedEvent>()
            .add_event::<TriggerActivatedEvent>()
            .add_event::<TriggerDeactivatedEvent>()
            .add_event::<ObjectTriggeredEvent>()
            .add_event::<AllPlayedEvent>()
            .add_observer(activate_trigger)
            .add_systems(
                Update,
                (
                    check_and_trigger_other,
                    draw_triggers,
                    handle_events_to_triggered_object,
                    handle_object_triggered,
                    check_all_played,
                    handle_all_played,
                )
                    .run_if(in_state(GameState::Execute))
                    .chain(),
            )
            .add_systems(Update, deactivate_trigger)
            .add_systems(OnEnter(GameState::Execute), enter_execution)
            .add_systems(OnExit(GameState::Execute), exit_execution);
    }
}

#[derive(Resource)]
pub struct LevelConfig {
    pub grow_factor: f32,
    pub scale: Box<dyn Scale>,
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
    pub source: Entity,
    pub note: Entity,
}

#[derive(Event, Debug, Copy, Clone)]
pub struct TriggerActivatedEvent {
    pub source: Option<Entity>,
    pub target: Entity,
}

#[derive(Event, Debug, Deref)]
struct TriggerDeactivatedEvent(Entity);

#[derive(Event, Debug)]
struct ObjectTriggeredEvent {
    source: Option<Entity>,
    object: Entity,
}

#[derive(Event, Debug)]
struct AllPlayedEvent;

fn handle_events_to_triggered_object(
    mut note_played: EventReader<NotePlayedEvent>,
    mut trigger_activated: EventReader<TriggerActivatedEvent>,
    mut object_triggered: EventWriter<ObjectTriggeredEvent>,
) {
    note_played.read().for_each(|ev| {
        object_triggered.write(ObjectTriggeredEvent {
            source: Some(ev.source),
            object: ev.note,
        });
    });
    trigger_activated.read().for_each(|ev| {
        object_triggered.write(ObjectTriggeredEvent {
            source: ev.source,
            object: ev.target,
        });
    });
}

/// Set the model data up for one execution. We keep some data in memory to simplify calculations.
fn enter_execution(
    triggers: Query<(Entity, &TriggerType)>,
    mut activate_triggers: EventWriter<TriggerActivatedEvent>,
    mut commands: Commands,
) {
    for (entity, trigger) in &triggers {
        if trigger == &TriggerType::Main {
            // TODO duplication
            let event = TriggerActivatedEvent {
                source: None,
                target: entity,
            };
            activate_triggers.write(event);
            commands.trigger(event);
        }
    }
}

/// Clear the game state after an execution.
fn exit_execution(
    triggers: Query<Entity, With<Trigger>>,
    mut events: EventWriter<TriggerDeactivatedEvent>,
) {
    for entity in triggers {
        events.write(TriggerDeactivatedEvent(entity));
    }
}

/// The core game logic: check if as trigger hits a note.
fn check_and_trigger_other(
    triggers: Query<(Entity, &mut TriggerSize, &TriggerState, &Transform)>,
    unplayed_objects: Query<&UntriggeredObjects>,
    notes: Query<&Note>,
    positions: Query<&Transform>,
    config: Res<LevelConfig>,
    time: Res<Time>,
    mut play_note_events: EventWriter<NotePlayedEvent>,
    mut activate_trigger_events: EventWriter<TriggerActivatedEvent>,
    mut commands: Commands,
) {
    for (trigger, mut size, trigger_state, trigger_position) in triggers {
        // update trigger state
        if !trigger_state.is_active() {
            continue;
        }
        size.increment(time.delta().as_secs_f32() * config.grow_factor);

        let Ok(unplayed_objects_of_trigger) = unplayed_objects.get(trigger) else {
            // only test when unplayed objects are present
            // should not happen, since a trigger is inactive in this condition
            continue;
        };

        for other in &unplayed_objects_of_trigger.0 {
            let Ok(position) = positions.get(*other) else {
                continue;
            };

            if position
                .translation
                .xy()
                .distance(trigger_position.translation.xy())
                < **size
            {
                // we can implement more types here, other trigger is always the fallback
                match notes.get(*other) {
                    // unplayed is a note
                    Ok(_) => {
                        play_note_events.write(NotePlayedEvent {
                            source: trigger,
                            note: *other,
                        });
                    }
                    _ => {
                        // unplayed is another trigger
                        //TODO duplication
                        let event = TriggerActivatedEvent {
                            source: Some(trigger),
                            target: *other,
                        };
                        commands.trigger(event);
                        activate_trigger_events.write(event);
                    }
                }
            } else {
                // since the objects are sorted relative to the trigger we can stop at the first one
                // which is too far away
                continue;
            }
        }
    }
}

/// Visualize the size of each trigger.
fn draw_triggers(
    mut gizmos: Gizmos,
    triggers: Query<(&TriggerState, &TriggerSize, &TriggerColor, &Transform)>,
) {
    for (state, size, color, transform) in &triggers {
        if !state.is_active() {
            continue;
        }

        let position = Isometry2d::from_translation(transform.translation.xy());
        gizmos.circle_2d(position, **size, color).resolution(64);
    }
}

/// If a trigger hits an object it is removed from the triggers list of unplayed notes.
fn handle_object_triggered(
    mut object_triggered_event: EventReader<ObjectTriggeredEvent>,
    mut all_untriggered: Query<&mut UntriggeredObjects>,
) {
    for event in object_triggered_event.read() {
        if let Some(trigger) = event.source {
            // source is only None if it is the MainTrigger on enter_execution...

            if let Ok(mut untriggered) = all_untriggered.get_mut(trigger) {
                untriggered.0.retain(|it| it != &event.object);
            }
        }
    }
}

/// After all notes are played the execution is done.
fn check_all_played(
    mut notes: Query<(Entity, &UntriggeredObjects)>,
    mut events: EventWriter<TriggerDeactivatedEvent>,
    mut all_played_events: EventWriter<AllPlayedEvent>,
) {
    let mut all_done = true;
    for (entity, notes) in &mut notes {
        if notes.0.is_empty() {
            events.write(TriggerDeactivatedEvent(entity));
        } else {
            all_done = false;
        }
    }

    if all_done {
        all_played_events.write(AllPlayedEvent);
    }
}

fn handle_all_played(
    mut events: EventReader<AllPlayedEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in events.read() {
        next_state.set(GameState::Build);
    }
}

fn activate_trigger(
    cause: bevy::prelude::Trigger<TriggerActivatedEvent>,
    triggers: Query<Entity, With<Trigger>>,
    notes: Query<Entity, With<Note>>,
    positions: Query<&Transform>,
    assets: Res<CoreAssets>,
    mut commands: Commands,
    state: Res<State<GameState>>,
) {
    // additional safety check: since events can arrive up to two updates later, this is sometimes triggered
    if *state != GameState::Execute {
        return;
    }

    // collect all objects in a list template
    let mut untriggered: Vec<Entity> = Vec::new();
    for object in &notes {
        untriggered.push(object);
    }
    for object in &triggers {
        untriggered.push(object);
    }

    let trigger = cause.target;
    let Ok(mut target) = commands.get_entity(trigger) else {
        return;
    };
    let trigger_position = positions
        .get(trigger)
        .expect("sort unplayed objects for trigger: trigger must have a position");

    // add all other objects to triggers unplayed objects list
    // this list is sorted by distance to the trigger
    let mut result: Vec<Entity> = untriggered.clone();
    result.retain(|it| it != &trigger);
    result
        .sort_by(|e1, e2| distance_for_sort(trigger_position.translation.xy(), e1, e2, &positions));

    // activate the trigger
    target
        .insert(TriggerState::Active)
        .insert(TriggerSize::zero())
        .insert(Svg2d(assets.trigger_icon_pause.clone()))
        .insert(UntriggeredObjects(result));
}

/// Compare the two objects by its distance to a trigger.
fn distance_for_sort(
    trigger_position: Vec2,
    e1: &Entity,
    e2: &Entity,
    positions: &Query<&Transform>,
) -> Ordering {
    let pos1 = positions
        .get(*e1)
        .expect("object must have a position")
        .translation
        .xy();
    let pos2 = positions
        .get(*e2)
        .expect("object must have a position")
        .translation
        .xy();

    pos1.distance(trigger_position)
        .total_cmp(&pos2.distance(trigger_position))
}

fn deactivate_trigger(
    mut events: EventReader<TriggerDeactivatedEvent>,
    assets: Res<CoreAssets>,
    mut commands: Commands,
) {
    for event in events.read() {
        commands
            .get_entity(event.0)
            .expect("trigger should exist")
            .remove::<UntriggeredObjects>()
            .insert(TriggerState::Inactive)
            .insert(TriggerSize::zero())
            .insert(Svg2d(assets.trigger_icon_play.clone()));
    }
}
