use crate::input::picker::SelectedItem;
use crate::state::{GameState, MinimalGameState};
use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;
use std::fmt::Write;

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
    commands.spawn((
        Name::new("Picker UI"),
        Node {
            // position_type: PositionType::Absolute,
            display: Display::Grid,
            width: Val::Percent(20.0),
            height: Val::Percent(8.0),
            margin: UiRect::all(Val::Px(10.0)),
            align_self: AlignSelf::Stretch,
            justify_self: JustifySelf::Stretch,
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::FlexStart,
            ..default()
        },
        BackgroundColor(WHITE.into()),
        BorderRadius::all(Val::Px(10.0)),
        Outline {
            width: Val::Px(6.),
            offset: Val::Px(6.),
            color: Color::WHITE,
        },
        SelectedItem::Note,
        Text::new("Note"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::BLACK),
    ));
}

fn handle_item_switch_input(
    mut commands: Commands,
    mut ui: Query<(Entity, &SelectedItem, &mut Text)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.any_just_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
        if let Ok((entity, item, mut text)) = ui.single_mut() {
            let next_item = item.switch();
            let mut new_text = String::new();
            write!(new_text, "{}", next_item.name()).expect("string concatenation should work");
            text.0 = new_text;
            commands.entity(entity).insert(next_item);
        }
    }
}
