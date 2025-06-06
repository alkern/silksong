use crate::visual::particles::ParticlePlugin;
use crate::visual::shader::ShaderPlugin;
use bevy::prelude::*;

pub mod color;
pub mod particles;
mod shader;

pub struct VisualPlugin;

impl Plugin for VisualPlugin {
    #[cfg(target_family = "wasm")]
    fn build(&self, app: &mut App) {
        app.add_plugins((ShaderPlugin));
    }

    #[cfg(not(target_family = "wasm"))]
    fn build(&self, app: &mut App) {
        app.add_plugins((ShaderPlugin, ParticlePlugin));
    }
}
