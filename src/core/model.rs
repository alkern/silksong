use crate::visual::color::ColorPalette;
use bevy::prelude::*;

#[derive(Component)]
pub struct Note;

#[derive(Component, Default, Debug)]
#[require(ActivatorSize, ActivatorColor)]
pub struct Activator;

#[derive(Component, Default, PartialEq, Debug, Deref)]
pub struct ActivatorSize(f32);

impl ActivatorSize {
    pub fn zero() -> Self {
        ActivatorSize(0.0)
    }

    pub fn increment(&mut self, value: f32) {
        self.0 += value;
    }
}

#[derive(Component, Default, PartialEq, Debug)]
#[require(Activator)]
pub enum ActivatorState {
    #[default]
    Disabled,
    Enabled,
}

impl ActivatorState {
    pub fn is_active(&self) -> bool {
        *self == ActivatorState::Enabled
    }
}

#[derive(Component, Default, PartialEq, Debug)]
#[require(Activator, ActivatorState, ActivatorSize)]
pub enum ActivatorType {
    Main,
    #[default]
    Passive,
}

#[derive(Component, Default, PartialEq, Debug, Copy, Clone)]
pub struct ActivatorColor(pub ColorPalette);

impl From<&ActivatorColor> for Color {
    fn from(value: &ActivatorColor) -> Self {
        value.0.as_rgba().into()
    }
}

#[derive(Component, Deref)]
pub struct InactivatedObjects(pub Vec<Entity>);
