use bevy::prelude::*;

#[derive(Component)]
pub struct Note;

#[derive(Component, Default, Debug)]
pub struct Trigger {
    pub size: f32,
    pub state: TriggerState,
}

impl Trigger {
    pub fn from_state(state: TriggerState) -> Trigger {
        Trigger { state, ..default() }
    }
}

#[derive(Default, PartialEq, Debug)]
pub enum TriggerState {
    Main,
    #[default]
    Inactive,
    Active,
}

#[derive(Component, Deref)]
pub struct UnplayedNotes(pub Vec<Entity>);
