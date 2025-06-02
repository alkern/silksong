mod core;
mod level;
mod math;
mod state;
mod visuals;
mod music;

use crate::core::CoreGamePlugin;
use crate::level::creative_mode::CreativeModePlugin;
use crate::state::GameStatePlugin;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use crate::visuals::VisualsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        // game plugins
        .add_plugins(GameStatePlugin)
        .add_plugins(CoreGamePlugin)
        .add_plugins(VisualsPlugin)
        // level plugins
        .add_plugins(CreativeModePlugin)
        // camera
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
