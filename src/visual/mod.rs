use crate::visual::particles::ParticlePlugin;
use crate::visual::shader::ShaderPlugin;
use bevy::prelude::*;

pub mod color;
pub mod particles;
mod shader;

pub struct VisualPlugin;

impl Plugin for VisualPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ShaderPlugin, ParticlePlugin));
    }
}
