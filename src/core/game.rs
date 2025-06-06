use crate::core::model::{
    Note, Trigger, TriggerColor, TriggerSize, TriggerState, TriggerType, UntriggeredObjects,
};
use crate::music::model::{NaturalMinorScale, Scale};
use crate::state::GameState;
use bevy::prelude::*;
use bevy_svg::prelude::{Svg, Svg2d};

pub struct CoreGamePlugin;

impl Plugin for CoreGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CoreAssets>()
            .add_event::<NotePlayedEvent>()
            .add_event::<TriggerActivatedEvent>()
            .add_event::<TriggerDeactivatedEvent>()
            .add_event::<ObjectTriggeredEvent>()
            .add_systems(
                Update,
                (
                    check_and_trigger_other::<NaturalMinorScale>,
                    draw_triggers,
                    handle_events_to_triggered_object,
                    handle_object_triggered,
                    check_all_played,
                )
                    .run_if(in_state(GameState::Execute))
                    .chain(),
            )
            .add_systems(Update, (activate_trigger, deactivate_trigger).chain())
            .add_systems(OnEnter(GameState::Execute), enter_execution)
            .add_systems(OnExit(GameState::Execute), exit_execution);
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
    pub source: Entity,
    pub note: Entity,
}

#[derive(Event, Debug)]
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
) {
    for (entity, trigger) in &triggers {
        if trigger == &TriggerType::Main {
            activate_triggers.write(TriggerActivatedEvent {
                source: None,
                target: entity,
            });
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
fn check_and_trigger_other<T>(
    triggers: Query<(Entity, &mut TriggerSize, &TriggerState, &Transform)>,
    unplayed_objects: Query<&UntriggeredObjects>,
    notes: Query<&Note>,
    positions: Query<&Transform>,
    config: Res<LevelConfig<T>>,
    time: Res<Time>,
    mut play_note_events: EventWriter<NotePlayedEvent>,
    mut activate_trigger_events: EventWriter<TriggerActivatedEvent>,
) where
    T: Scale,
{
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
                        activate_trigger_events.write(TriggerActivatedEvent {
                            source: Some(trigger),
                            target: *other,
                        });
                    }
                }
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
    // mut next_state: ResMut<NextState<GameState>>
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
        //TODO stop execution
        info!("All played");
        // next_state.set(GameState::Over);
    }
}

fn activate_trigger(
    mut events: EventReader<TriggerActivatedEvent>,
    triggers: Query<Entity, With<Trigger>>,
    notes: Query<Entity, With<Note>>,
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

    for event in events.read() {
        let trigger = event.target;
        let Ok(mut target) = commands.get_entity(trigger) else {
            continue;
        };
        // add all other objects to triggers unplayed objects list
        let mut result: Vec<Entity> = untriggered.clone();
        result.retain(|it| it != &trigger);

        // activate the trigger
        target
            .insert(TriggerState::Active)
            .insert(TriggerSize::zero())
            .insert(Svg2d(assets.trigger_icon_pause.clone()))
            .insert(UntriggeredObjects(result));

        // build effect
    }
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
