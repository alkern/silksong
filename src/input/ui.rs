use crate::input::picker::SelectedItem;
use crate::state::{GameState, MinimalGameState};
use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SetupGameObjects), setup)
            .add_systems(
                Update,
                handle_item_switch_input.run_if(in_state(MinimalGameState::Running)),
            );
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

            parent.spawn(how_to_sentence("\n\n"));
            parent.spawn(how_to_sentence("Press Space to start/stop\n\n"));
            parent.spawn(how_to_sentence(
                "Press Ctrl to select which object to place\n\n",
            ));
            parent.spawn(how_to_sentence(
                "Use left mouse to place the selected object\n\n",
            ));
            parent.spawn(how_to_sentence("Use right mouse to delete an object\n\n"));
            parent.spawn(how_to_sentence("Press Backspace to delete all objects\n\n"));
        });
}

fn how_to_sentence(content: &str) -> impl Bundle {
    (
        Name::new("How To Textbox"),
        Node {
            // position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            padding: UiRect::all(Val::Px(4.0)),
            ..default()
        },
        Text::new(content),
        TextFont {
            font_size: 15.0,
            ..default()
        },
        TextColor(Color::BLACK),
    )
}

fn handle_item_switch_input(
    mut commands: Commands,
    mut ui: Query<(Entity, &SelectedItem, &mut Text)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.any_just_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
        if let Ok((entity, item, mut text)) = ui.single_mut() {
            let next_item = item.switch();
            // TODO string concatenation
            let mut new_text = "\n".to_string();
            new_text.push_str(next_item.name().as_str());

            text.0 = new_text;
            commands.entity(entity).insert(next_item);
        }
    }
}
