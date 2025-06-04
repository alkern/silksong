use crate::state::{GameState, MinimalGameState};
use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;

pub(super) struct PickerPlugin;

impl Plugin for PickerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SetupGameObjects), setup)
            .add_systems(
                Update,
                handle_input.run_if(in_state(MinimalGameState::Running)),
            );
    }
}

#[derive(Component, PartialEq, Debug)]
enum SelectedItem {
    Trigger,
    Node,
}

impl SelectedItem {
    fn switch(&self) -> SelectedItem {
        match self {
            SelectedItem::Trigger => SelectedItem::Node,
            SelectedItem::Node => SelectedItem::Trigger,
        }
    }

    fn name(&self) -> String {
        match self {
            SelectedItem::Trigger => "Trigger".to_string(),
            SelectedItem::Node => "Node".to_string(),
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

fn handle_input(
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
