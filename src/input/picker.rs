use crate::core::game::CoreAssets;
use crate::core::model::Note;
use crate::state::{GameState, MinimalGameState};
use bevy::color::palettes::basic::WHITE;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy_svg::prelude::{Origin, Svg2d};

pub(super) struct PickerPlugin;

impl Plugin for PickerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SetupGameObjects), setup)
            .add_systems(
                Update,
                (handle_item_switch_input, handle_item_placement)
                    .run_if(in_state(MinimalGameState::Running)),
            );
    }
}

#[derive(Component, PartialEq, Debug)]
enum SelectedItem {
    Trigger,
    Note,
}

impl SelectedItem {
    fn switch(&self) -> SelectedItem {
        match self {
            SelectedItem::Trigger => SelectedItem::Note,
            SelectedItem::Note => SelectedItem::Trigger,
        }
    }

    fn name(&self) -> String {
        match self {
            SelectedItem::Trigger => "Trigger".to_string(),
            SelectedItem::Note => "Node".to_string(),
        }
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Picker UI"),
            Node {
                // position_type: PositionType::Absolute,
                display: Display::Grid,
                width: Val::Percent(5.0),
                height: Val::Percent(5.0),
                margin: UiRect::all(Val::Px(25.0)),
                align_self: AlignSelf::Stretch,
                justify_self: JustifySelf::Stretch,
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                align_content: AlignContent::FlexStart,
                ..default()
            },
            BackgroundColor(WHITE.into()),
            // BorderColor(RED.into()),
            BorderRadius::all(Val::Px(10.0)),
            Outline {
                width: Val::Px(6.),
                offset: Val::Px(6.),
                color: Color::WHITE,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Current Item Node"),
                Node {
                    // position_type: PositionType::Absolute,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    padding: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                SelectedItem::Trigger,
                Text::new("Trigger"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));
        });
}

fn handle_item_switch_input(
    mut commands: Commands,
    mut ui: Query<(Entity, &SelectedItem, &mut Text)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.any_just_pressed([KeyCode::KeyW, KeyCode::KeyS]) {
        if let Ok((entity, item, mut text)) = ui.single_mut() {
            let next_item = item.switch();
            text.0 = next_item.name();
            commands.entity(entity).insert(next_item);
        }
    }
}

fn handle_item_placement(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    selected_item: Query<&SelectedItem>,
    assets: Res<CoreAssets>,
) {
    for event in mouse_button_input_events.read() {
        if event.button == MouseButton::Left {
            // calculation taken from https://bevy-cheatbook.github.io/cookbook/cursor2world.html
            let (camera, camera_transform) = camera.single().expect("Camera must exist");

            if let Some(world_position) = window
                .get(event.window)
                .expect("Window must exist")
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
                .map(|ray| ray.origin.truncate())
            {
                let item = selected_item.single().expect("SelectedItem must exist");

                match item {
                    SelectedItem::Trigger => {
                        commands.spawn((
                            Name::new(item.name()),
                            crate::core::model::Trigger::default(),
                            Transform::from_translation(world_position.extend(0.0))
                                .with_scale(Vec3::splat(0.05)),
                            Svg2d(assets.trigger_icon_play.clone()),
                            Origin::Center,
                        ));
                    }
                    SelectedItem::Note => {
                        commands.spawn((
                            Name::new(item.name()),
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
    }
}
