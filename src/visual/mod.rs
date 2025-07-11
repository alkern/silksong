use crate::visual::shader::ShaderPlugin;
use bevy::prelude::*;

pub mod color;
mod shader;

pub struct VisualPlugin;

impl Plugin for VisualPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShaderPlugin);
    }
}
