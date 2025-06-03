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

    pub fn deactivate(&mut self) {
        self.state = TriggerState::Inactive;
        self.size = 0.0;
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
pub struct UntriggeredObjects(pub Vec<(Entity, Position)>);

#[derive(Deref, Clone, Copy, Debug)]
pub struct Position(pub Vec2);
