use crate::state::AppState;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin};
use bevy::window::WindowResized;

const SHADER_PATH: &str = "shaders/silk.wgsl";

pub(super) struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<SilkMaterial>::default())
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (update, handle_window_resize).run_if(in_state(AppState::Game)),
            );
    }
}

#[derive(Component)]
struct Shader;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
struct SilkMaterial {
    #[uniform(0)]
    time: f32,
    //TODO temp fix for wasm
    // In Device::create_render_pipeline, label = 'opaque_mesh2d_pipeline'
    // In the provided shader, the type given for group 2 binding 0 has a size of 4.
    // As the device does not support `DownlevelFlags::BUFFER_BINDINGS_NOT_16_BYTE_ALIGNED`,
    // the type must have a size that is a multiple of 16 bytes.
    #[uniform(0)]
    padding_1: f32,
    #[uniform(0)]
    padding_2: f32,
    #[uniform(0)]
    padding_3: f32,
}

impl Material2d for SilkMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SilkMaterial>>,
    windows: Query<&Window>,
) {
    let size = windows
        .single()
        .expect("window should exist at this point")
        .resolution
        .size();

    commands.spawn((
        Shader,
        MeshMaterial2d(materials.add(SilkMaterial::default())),
        Mesh2d(meshes.add(Rectangle::default())),
        Transform::from_scale(size.extend(0.0)),
    ));
}

fn update(time: Res<Time>, mut backgrounds: ResMut<Assets<SilkMaterial>>) {
    backgrounds.iter_mut().for_each(|material| {
        material.1.time += time.delta_secs() / 2.;
    })
}

fn handle_window_resize(
    mut events: EventReader<WindowResized>,
    mut shader: Query<&mut Transform, With<Shader>>,
) {
    for event in events.read() {
        shader
            .single_mut()
            .expect("there must be exactly one background shader")
            .scale = Vec3::new(event.width, event.height, 0.);
    }
}
