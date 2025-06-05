use crate::core::game::CoreAssets;
use crate::core::model::{Note, TriggerType};
use crate::state::GameState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy_svg::prelude::{Origin, Svg2d};
use std::ops::Add;

pub(super) struct PickerPlugin;

impl Plugin for PickerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaceObjectEvent>()
            .add_event::<DeleteObjectEvent>()
            .add_systems(
                Update,
                (handle_mouse_input, place_object, delete_object, clear)
                    .run_if(in_state(GameState::Build)),
            );
    }
}

#[derive(Component)]
struct ManuallyPlaced;

#[derive(Component, PartialEq, Debug, Copy, Clone)]
pub(super) enum SelectedItem {
    Trigger,
    Note,
}

impl SelectedItem {
    pub(super) fn switch(&self) -> SelectedItem {
        match self {
            SelectedItem::Trigger => SelectedItem::Note,
            SelectedItem::Note => SelectedItem::Trigger,
        }
    }

    pub(super) fn name(&self) -> String {
        match self {
            SelectedItem::Trigger => "Trigger".to_string(),
            SelectedItem::Note => "Note".to_string(),
        }
    }
}

#[derive(Component, PartialEq, Deref, Debug)]
struct InputTimer(Timer);

impl InputTimer {
    fn new() -> Self {
        InputTimer(Timer::from_seconds(0.15, TimerMode::Once))
    }
}

#[derive(Event, Debug)]
struct PlaceObjectEvent(Vec2);

#[derive(Event, Debug)]
struct DeleteObjectEvent(Vec2);

fn handle_mouse_input(
    // input backoff
    time: Res<Time>,
    mut timer: Query<(Entity, &mut InputTimer)>,
    // calculate mouse position
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    // for interaction
    mut commands: Commands,
    mut place_object: EventWriter<PlaceObjectEvent>,
    mut delete_object: EventWriter<DeleteObjectEvent>,
) {
    // check timer, if the last interaction has been some frames ago
    if let Ok((entity, mut timer)) = timer.single_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            commands.get_entity(entity).unwrap().despawn();
        } else {
            return;
        }
    }

    // always execute just one event as another check besides the timer
    let mut events = Vec::new();
    for event in mouse_button_input_events.read() {
        events.push(event);
    }

    if events.is_empty() {
        return;
    }
    let event = events[0];

    // handle the input
    if let Ok(position) = cursor_to_world(window, camera, event.window) {
        match event.button {
            MouseButton::Left => {
                place_object.write(PlaceObjectEvent(position));
            }
            MouseButton::Right => {
                delete_object.write(DeleteObjectEvent(position));
            }
            _ => {}
        }
    }

    // start timer again
    commands.spawn(InputTimer::new());
}

fn delete_object(
    mut events: EventReader<DeleteObjectEvent>,
    mut commands: Commands,
    objects: Query<(Entity, &Transform), With<ManuallyPlaced>>,
    main_trigger: Query<&TriggerType>,
) {
    for event in events.read() {
        for object in &objects {
            if object.1.translation.xy().distance(event.0) < 10.0 {
                if let Ok(trigger) = main_trigger.get(object.0) {
                    if trigger == &TriggerType::Main {
                        // main trigger cannot be removed
                        continue;
                    }
                }

                commands.entity(object.0).despawn();
            }
        }
    }
}

fn place_object(
    mut events: EventReader<PlaceObjectEvent>,
    mut commands: Commands,
    selected_item: Query<&SelectedItem>,
    assets: Res<CoreAssets>,
) {
    for event in events.read() {
        let world_position = event.0;
        let item = selected_item.single().expect("SelectedItem must exist");

        match item {
            SelectedItem::Trigger => {
                commands.spawn((
                    Name::new(item.name().add(" manual")),
                    ManuallyPlaced,
                    TriggerType::Passive,
                    Transform::from_translation(world_position.extend(0.0))
                        .with_scale(Vec3::splat(0.05)),
                    Svg2d(assets.trigger_icon_play.clone()),
                    Origin::Center,
                ));
            }
            SelectedItem::Note => {
                commands.spawn((
                    Name::new(item.name()),
                    ManuallyPlaced,
                    Note,
                    Transform::from_translation(world_position.extend(0.0))
                        .with_scale(Vec3::splat(0.025)),
                    Svg2d(assets.note_icon.clone()),
                    Origin::Center,
                ));
            }
        }
    }
}

fn cursor_to_world(
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Entity,
) -> Result<Vec2> {
    // calculation taken from https://bevy-cheatbook.github.io/cookbook/cursor2world.html
    let (camera, camera_transform) = camera.single()?;

    windows
        .get(window)
        .expect("Window must exist")
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
        .ok_or(BevyError::from(
            "could not calculate world position for mouse input",
        ))
}

fn clear(
    mut commands: Commands,
    entities: Query<Entity, With<ManuallyPlaced>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Backspace) {
        for entity in entities.iter() {
            commands.entity(entity).despawn();
        }
    }
}
