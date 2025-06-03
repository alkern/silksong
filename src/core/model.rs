use bevy::prelude::*;

#[derive(Component)]
pub struct Note;

#[derive(Component, Default, Debug)]
pub struct Trigger {
    pub size: f32,
    pub state: TriggerState,
    pub trigger_type: TriggerType,
}

impl Trigger {
    pub fn main() -> Trigger {
        Trigger {
            trigger_type: TriggerType::Main,
            ..default()
        }
    }
}

#[derive(Default, PartialEq, Debug)]
pub enum TriggerState {
    #[default]
    Inactive,
    Active,
}

#[derive(Component, Default, PartialEq, Debug)]
pub enum TriggerType {
    Main,
    #[default]
    Passive,
}

#[derive(Component, Deref)]
pub struct UnplayedNotes(pub Vec<Entity>);
