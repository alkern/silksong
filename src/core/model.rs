use crate::visual::color::ColorPalette;
use bevy::prelude::*;

#[derive(Component)]
pub struct Note;

#[derive(Component, Default, Debug)]
#[require(TriggerSize, TriggerColor)]
pub struct Trigger;

#[derive(Component, Default, PartialEq, Debug, Deref)]
pub struct TriggerSize(f32);

impl TriggerSize {
    pub fn zero() -> Self {
        TriggerSize(0.0)
    }

    pub fn increment(&mut self, value: f32) {
        self.0 += value;
    }
}

#[derive(Component, Default, PartialEq, Debug)]
#[require(Trigger)]
pub enum TriggerState {
    #[default]
    Inactive,
    Active,
}

impl TriggerState {
    pub fn is_active(&self) -> bool {
        *self == TriggerState::Active
    }
}

#[derive(Component, Default, PartialEq, Debug)]
#[require(Trigger, TriggerState, TriggerSize)]
pub enum TriggerType {
    Main,
    #[default]
    Passive,
}

#[derive(Component, Default, PartialEq, Debug, Copy, Clone)]
pub struct TriggerColor(pub ColorPalette);

impl From<&TriggerColor> for Color {
    fn from(value: &TriggerColor) -> Self {
        value.0.as_rgba().into()
    }
}

#[derive(Component, Deref)]
pub struct UntriggeredObjects(pub Vec<Entity>);
