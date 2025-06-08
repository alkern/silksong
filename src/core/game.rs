use crate::core::model::{
    Activator, ActivatorColor, ActivatorSize, ActivatorState, ActivatorType, InactivatedObjects,
    Note,
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
            .add_event::<ActivatorEnabledEvent>()
            .add_event::<ActivatorDisabledEvent>()
            .add_event::<ObjectActivatedEvent>()
            .add_event::<AllPlayedEvent>()
            .add_observer(activate_activator)
            .add_systems(
                Update,
                (
                    execute_activator_and_check_collisions,
                    draw_activator_sizes,
                    collect_activation_events,
                    handle_object_activated,
                    check_all_played,
                    handle_all_played,
                )
                    .run_if(in_state(GameState::Execute))
                    .chain(),
            )
            .add_systems(Update, disable_activator)
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
    pub activator_icon_pause: Handle<Svg>,
    pub activator_icon_play: Handle<Svg>,
}

impl FromWorld for CoreAssets {
    fn from_world(world: &mut World) -> Self {
        CoreAssets {
            note_icon: world.load_asset("icons/music-solid.svg"),
            activator_icon_pause: world.load_asset("icons/circle-pause-regular.svg"),
            activator_icon_play: world.load_asset("icons/circle-play-regular.svg"),
        }
    }
}

#[derive(Event, Debug)]
pub struct NotePlayedEvent {
    pub source: Entity,
    pub note: Entity,
}

#[derive(Event, Debug, Copy, Clone)]
pub struct ActivatorEnabledEvent {
    pub source: Option<Entity>,
    pub target: Entity,
}

#[derive(Event, Debug, Deref)]
struct ActivatorDisabledEvent(Entity);

#[derive(Event, Debug)]
struct ObjectActivatedEvent {
    source: Option<Entity>,
    object: Entity,
}

#[derive(Event, Debug)]
struct AllPlayedEvent;

/// Collects all events which represent an "activation" and map them to [`ObjectActivatedEvent`]s to
/// handle similarly.
fn collect_activation_events(
    mut note_played: EventReader<NotePlayedEvent>,
    mut activator_enabled: EventReader<ActivatorEnabledEvent>,
    mut object_activated: EventWriter<ObjectActivatedEvent>,
) {
    note_played.read().for_each(|ev| {
        object_activated.write(ObjectActivatedEvent {
            source: Some(ev.source),
            object: ev.note,
        });
    });
    activator_enabled.read().for_each(|ev| {
        object_activated.write(ObjectActivatedEvent {
            source: ev.source,
            object: ev.target,
        });
    });
}

/// Set the model data up for one execution. We keep some data in memory to simplify calculations.
fn enter_execution(
    activators: Query<(Entity, &ActivatorType)>,
    mut enabled_activators: EventWriter<ActivatorEnabledEvent>,
    mut commands: Commands,
) {
    for (entity, activator) in &activators {
        if activator == &ActivatorType::Main {
            // TODO duplication
            let event = ActivatorEnabledEvent {
                source: None,
                target: entity,
            };
            enabled_activators.write(event);
            commands.trigger(event);
        }
    }
}

/// Clear the game state after an execution.
fn exit_execution(
    activators: Query<Entity, With<Activator>>,
    mut events: EventWriter<ActivatorDisabledEvent>,
) {
    for entity in activators {
        events.write(ActivatorDisabledEvent(entity));
    }
}

/// The core game logic: increment activator and check for collisions
fn execute_activator_and_check_collisions(
    activators: Query<(Entity, &mut ActivatorSize, &ActivatorState, &Transform)>,
    unplayed_objects: Query<&InactivatedObjects>,
    notes: Query<&Note>,
    positions: Query<&Transform>,
    config: Res<LevelConfig>,
    time: Res<Time>,
    mut play_note_events: EventWriter<NotePlayedEvent>,
    mut enable_activator_events: EventWriter<ActivatorEnabledEvent>,
    mut commands: Commands,
) {
    for (activator, mut size, activator_state, activator_position) in activators {
        // grow enabled activator size
        if !activator_state.is_active() {
            continue;
        }
        size.increment(time.delta().as_secs_f32() * config.grow_factor);

        let Ok(unplayed_objects_of_activator) = unplayed_objects.get(activator) else {
            // only test when unplayed objects are present
            // should not happen, since an activator is disabled in this condition
            continue;
        };

        // check collisions
        for other in &unplayed_objects_of_activator.0 {
            let Ok(position) = positions.get(*other) else {
                continue;
            };

            if position
                .translation
                .xy()
                .distance(activator_position.translation.xy())
                < **size
            {
                // we can implement more types here, only activator cannot be matched with a query
                match notes.get(*other) {
                    // unplayed is a note
                    Ok(_) => {
                        play_note_events.write(NotePlayedEvent {
                            source: activator,
                            note: *other,
                        });
                    }
                    _ => {
                        // unplayed is another activator
                        //TODO duplication
                        let event = ActivatorEnabledEvent {
                            source: Some(activator),
                            target: *other,
                        };
                        commands.trigger(event);
                        enable_activator_events.write(event);
                    }
                }
            } else {
                // since the objects are sorted relative to the activator we can stop at the first
                // one which is too far away
                break;
            }
        }
    }
}

/// Visualize the size of each activator.
fn draw_activator_sizes(
    mut gizmos: Gizmos,
    activators: Query<(&ActivatorState, &ActivatorSize, &ActivatorColor, &Transform)>,
) {
    for (state, size, color, transform) in &activators {
        if !state.is_active() {
            continue;
        }

        let position = Isometry2d::from_translation(transform.translation.xy());
        gizmos.circle_2d(position, **size, color).resolution(64);
    }
}

/// If an activator hits an object it is removed from the list of unplayed objects for the activator
fn handle_object_activated(
    mut object_activated_event: EventReader<ObjectActivatedEvent>,
    mut all_inactive: Query<&mut InactivatedObjects>,
) {
    for event in object_activated_event.read() {
        if let Some(activator) = event.source {
            // source is only None if it is the MainActivator on enter_execution...

            if let Ok(mut inactive) = all_inactive.get_mut(activator) {
                inactive.0.retain(|it| it != &event.object);
            }
        }
    }
}

/// After all notes are played the execution is done.
fn check_all_played(
    mut notes: Query<(Entity, &InactivatedObjects)>,
    mut events: EventWriter<ActivatorDisabledEvent>,
    mut all_played_events: EventWriter<AllPlayedEvent>,
) {
    let mut all_done = true;
    for (entity, notes) in &mut notes {
        if notes.0.is_empty() {
            events.write(ActivatorDisabledEvent(entity));
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

fn activate_activator(
    cause: Trigger<ActivatorEnabledEvent>,
    activators: Query<Entity, With<Activator>>,
    notes: Query<Entity, With<Note>>,
    positions: Query<&Transform>,
    assets: Res<CoreAssets>,
    mut commands: Commands,
    state: Res<State<GameState>>,
) {
    // additional safety check: since events can arrive up to two updates later, this is sometimes
    // run unnecessarily
    if *state != GameState::Execute {
        return;
    }

    // collect all objects in a list template
    let mut inactive: Vec<Entity> = Vec::new();
    for object in &notes {
        inactive.push(object);
    }
    for object in &activators {
        inactive.push(object);
    }

    let activator = cause.target;
    let Ok(mut target) = commands.get_entity(activator) else {
        return;
    };
    let activator_position = positions
        .get(activator)
        .expect("sort unplayed objects for activator: activator must have a position");

    // add all other objects to the unplayed objects list for this activator.
    // this list is sorted by distance to the activator.
    let mut result: Vec<Entity> = inactive.clone();
    result.retain(|it| it != &activator);
    result.sort_by(|e1, e2| {
        distance_for_sort(activator_position.translation.xy(), e1, e2, &positions)
    });

    // enable the activator
    target
        .insert(ActivatorState::Enabled)
        .insert(ActivatorSize::zero())
        .insert(Svg2d(assets.activator_icon_pause.clone()))
        .insert(InactivatedObjects(result));
}

/// Compare the two objects by its distance to an activator.
fn distance_for_sort(
    activator_position: Vec2,
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

    pos1.distance(activator_position)
        .total_cmp(&pos2.distance(activator_position))
}

fn disable_activator(
    mut events: EventReader<ActivatorDisabledEvent>,
    assets: Res<CoreAssets>,
    mut commands: Commands,
) {
    for event in events.read() {
        commands
            .get_entity(event.0)
            .expect("activator should exist")
            .remove::<InactivatedObjects>()
            .insert(ActivatorState::Disabled)
            .insert(ActivatorSize::zero())
            .insert(Svg2d(assets.activator_icon_play.clone()));
    }
}
