mod core;
mod input;
mod level;
mod math;
mod music;
mod state;
mod visual;

use crate::core::game::CoreGamePlugin;
use crate::input::InputPlugin;
use crate::level::creative_mode::CreativeModePlugin;
use crate::music::audio::AudioPlugin;
use crate::music::game::MusicPlugin;
use crate::state::GameStatePlugin;
use crate::visual::VisualPlugin;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowMode;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics in web builds on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Silksong".into(),
                        name: Some("Silksong".into()),
                        mode: WindowMode::Fullscreen(
                            MonitorSelection::Primary,
                            VideoModeSelection::Current,
                        ),
                        ..default()
                    }),
                    ..default()
                }),
        )
        // external plugins
        .add_plugins(bevy_svg::prelude::SvgPlugin)
        // game plugins
        .add_plugins(AudioPlugin)
        .add_plugins(CoreGamePlugin)
        .add_plugins(GameStatePlugin)
        .add_plugins(InputPlugin)
        .add_plugins(MusicPlugin)
        .add_plugins(VisualPlugin)
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
