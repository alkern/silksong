use crate::core::model::{Note, Position, Trigger, TriggerState, TriggerType, UntriggeredObjects};
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
            .add_event::<TriggerDeactivatedEvent>()
            .add_event::<ObjectTriggeredEvent>()
            .add_systems(
                Update,
                (
                    activate_trigger,
                    deactivate_trigger,
                    check_and_play_notes::<NaturalMinorScale>,
                    draw_triggers,
                    handle_events_to_triggered_object,
                    handle_object_triggered,
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
    pub source: Entity,
    pub note: Entity,
}

#[derive(Event, Debug)]
struct TriggerActivatedEvent {
    source: Option<Entity>,
    trigger: Entity,
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
            object: ev.trigger,
        });
    });
}

/// Set the model data up for one execution. We keep some data in memory to simplify calculations.
fn enter_execution(
    triggers: Query<(Entity, &Trigger)>,
    mut activate_triggers: EventWriter<TriggerActivatedEvent>,
) {
    for (entity, trigger) in &triggers {
        if trigger.trigger_type == TriggerType::Main {
            activate_triggers.write(TriggerActivatedEvent {
                source: None,
                trigger: entity,
            });
        }
    }
}

/// Clear the game state after an execution.
fn exit_execution(
    triggers: Query<(Entity, &Trigger)>,
    mut events: EventWriter<TriggerDeactivatedEvent>,
) {
    for (entity, _) in triggers {
        events.write(TriggerDeactivatedEvent(entity));
    }
}

/// The core game logic: check if as trigger hits a note.
fn check_and_play_notes<T>(
    triggers: Query<(Entity, &mut Trigger, &Transform)>,
    unplayed_objects: Query<&UntriggeredObjects>,
    notes: Query<(&Note, &Transform)>,
    config: Res<LevelConfig<T>>,
    time: Res<Time>,
    mut play_note_events: EventWriter<NotePlayedEvent>,
    mut activate_trigger_events: EventWriter<TriggerActivatedEvent>,
) where
    T: Scale,
{
    for (entity, mut trigger, trigger_position) in triggers {
        // update trigger state
        if trigger.state == TriggerState::Inactive {
            continue;
        }
        trigger.size += time.delta().as_secs_f32() * config.grow_factor;

        let Ok(unplayed_objects_of_trigger) = unplayed_objects.get(entity) else {
            continue;
        };
        for unplayed in unplayed_objects_of_trigger.0.clone() {
            if unplayed.1.xy().distance(trigger_position.translation.xy()) < trigger.size {
                // we can implement more types here, other trigger is always the fallback
                match notes.get(unplayed.0) {
                    // unplayed is a note
                    Ok(_) => {
                        play_note_events.write(NotePlayedEvent {
                            source: entity,
                            note: unplayed.0,
                        });
                    }
                    _ => {
                        // unplayed is another trigger
                        activate_trigger_events.write(TriggerActivatedEvent {
                            source: Some(entity),
                            trigger: unplayed.0,
                        });
                    }
                }
            }
        }
    }
}

/// Visualize the size of each trigger.
fn draw_triggers(mut gizmos: Gizmos, triggers: Query<(&Trigger, &Transform)>) {
    for (trigger, transform) in &triggers {
        if trigger.state == TriggerState::Inactive {
            continue;
        }

        let position = Isometry2d::from_translation(transform.translation.xy());
        gizmos
            .circle_2d(position, trigger.size, BLUE_VIOLET)
            .resolution(64);
    }
}

/// If a trigger hits a note, this note is played and removed from the triggers list of unplayed notes.
fn handle_object_triggered(
    mut note_played_events: EventReader<ObjectTriggeredEvent>,
    mut all_untriggered: Query<&mut UntriggeredObjects>,
) {
    // TODO refactor this whole thing
    let mut to_remove: HashMap<Entity, Vec<Entity>> = HashMap::new();
    for event in note_played_events.read() {
        if let Some(trigger) = event.source {
            match to_remove.get_mut(&trigger) {
                None => {
                    to_remove.insert(trigger, vec![event.object]);
                }
                Some(values) => {
                    values.add(event.object);
                }
            };
        }
    }

    for (trigger, objects_to_remove) in to_remove {
        let Ok(mut untriggered) = all_untriggered.get_mut(trigger) else {
            continue;
        };

        let mut objects_to_keep = Vec::new();
        for note in &untriggered.0 {
            if !objects_to_remove.contains(&note.0) {
                objects_to_keep.push(*note);
            }
        }

        untriggered.0 = objects_to_keep;
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
    mut triggers: Query<(Entity, &mut Trigger, &Transform)>,
    notes: Query<(Entity, &Note, &Transform)>,
    assets: Res<CoreAssets>,
    mut commands: Commands,
) {
    // collect all objects in a list template
    let mut untriggered: Vec<(Entity, Position)> = Vec::new();
    for (entity, _, transform) in &notes {
        untriggered.push((entity, Position(transform.translation.xy())));
    }
    for (entity, _, transform) in &triggers {
        untriggered.push((entity, Position(transform.translation.xy())));
    }

    for event in events.read() {
        let entity = event.trigger;
        // activate the trigger
        if let Ok((_, mut trigger, _)) = triggers.get_mut(event.trigger) {
            trigger.state = TriggerState::Active;
            update_icon(
                assets.trigger_icon_pause.clone(),
                event.trigger,
                &mut commands,
            );
        }

        // add all other objects to this triggers unplayed objects list
        let Ok(mut trigger) = commands.get_entity(entity) else {
            continue;
        };
        let mut result: Vec<(Entity, Position)> = Vec::new();
        for x in untriggered.clone() {
            if x.0 != entity {
                result.push(x);
            }
        }
        trigger.insert(UntriggeredObjects(result));
    }
}

fn deactivate_trigger(
    mut events: EventReader<TriggerDeactivatedEvent>,
    mut query: Query<&mut Trigger>,
    assets: Res<CoreAssets>,
    mut commands: Commands,
) {
    for event in events.read() {
        if let Ok(mut trigger) = query.get_mut(**event) {
            trigger.deactivate();
            update_icon(assets.trigger_icon_play.clone(), **event, &mut commands);
            commands
                .get_entity(**event)
                .expect("trigger should exist")
                .remove::<UntriggeredObjects>();
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
