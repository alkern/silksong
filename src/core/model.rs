use bevy::prelude::*;

#[derive(Component)]
pub struct Note;

#[derive(Component, Default, Debug)]
pub struct Trigger {
    pub size: f32,
}

#[derive(Component, Deref)]
pub struct UnplayedNotes(pub Vec<Entity>);
