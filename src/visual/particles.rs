use crate::state::GameState;
use crate::visual::color::ColorPalette;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub(super) struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ParticleEffects>()
            .add_systems(Update, handle_timers.run_if(in_state(GameState::Execute)))
            .add_systems(OnExit(GameState::Execute), cleanup);
    }
}

#[derive(Component)]
pub struct ParticleTimer(Timer);

impl ParticleTimer {
    pub fn new() -> Self {
        ParticleTimer(Timer::from_seconds(3.0, TimerMode::Once))
    }
}

fn handle_timers(
    mut timers: Query<(Entity, &mut ParticleTimer)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (e, mut timer) in &mut timers {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            commands.entity(e).despawn();
        }
    }
}

fn cleanup(timers: Query<Entity, With<ParticleTimer>>, mut commands: Commands) {
    for e in &timers {
        commands.entity(e).despawn()
    }
}

#[derive(Resource)]
pub struct ParticleEffects {
    map: HashMap<ColorPalette, Handle<EffectAsset>>,
}

impl ParticleEffects {
    pub fn get(&self, color: &ColorPalette) -> Handle<EffectAsset> {
        self.map
            .get(color)
            .expect("all colors must have an effect")
            .clone()
    }
}

impl FromWorld for ParticleEffects {
    fn from_world(world: &mut World) -> Self {
        let mut map = HashMap::new();

        for color in ColorPalette::enumerate() {
            map.insert(color, world.add_asset(build_effect(&color)));
        }

        ParticleEffects { map }
    }
}

fn build_effect(color: &ColorPalette) -> EffectAsset {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::from(color));
    gradient.add_key(1.0, Vec4::ZERO);

    // Create a new expression module
    let mut module = Module::default();

    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(50.0),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(6.0),
    };

    let lifetime = module.lit(1.5);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let size = module.lit(Vec2::new(2.0, 2.0));
    let init_size = SetAttributeModifier::new(Attribute::SIZE2, size);

    EffectAsset::new(512, SpawnerSettings::once(500.0.into()), module)
        .with_name(format!("Effect {:?}", color))
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .init(init_size)
        .render(ColorOverLifetimeModifier {
            gradient,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::RGBA,
        })
}
