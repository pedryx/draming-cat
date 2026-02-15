use bevy::{
    core_pipeline::{
        core_2d::graph::Node2d,
        fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin},
    },
    prelude::*,
    render::{
        extract_component::ExtractComponent,
        render_graph::{InternedRenderLabel, RenderLabel},
        render_resource::ShaderType,
    },
    shader::ShaderRef,
};

use crate::{PausableSystems, game::player::Player};

const EFFECT_DURATION: f32 = 1.5;

pub fn plugin(app: &mut App) {
    app.add_plugins(FullscreenMaterialPlugin::<GlitchEffect>::default())
        .add_observer(spawn_glitch_effect)
        .add_systems(Update, update_glitch_effect.in_set(PausableSystems));
}

#[derive(Event)]
pub struct SpawnGlitchEffect;

#[derive(Component, ShaderType, Default, Copy, Clone, ExtractComponent)]
pub struct GlitchEffect {
    progress: f32,
}

#[derive(Component)]
struct GlitchEffectMarker;

impl FullscreenMaterial for GlitchEffect {
    fn fragment_shader() -> ShaderRef {
        "shaders/glitch.wgsl".into()
    }

    fn node_edges() -> Vec<InternedRenderLabel> {
        vec![
            Node2d::Tonemapping.intern(),
            Self::node_label().intern(),
            Node2d::EndMainPassPostProcessing.intern(),
        ]
    }
}

fn spawn_glitch_effect(_: On<SpawnGlitchEffect>, mut commands: Commands) {
    commands.spawn(GlitchEffectMarker);
}

fn update_glitch_effect(
    mut commands: Commands,
    time: Res<Time>,
    mut effect: Single<&mut GlitchEffect>,
    marker: Single<Entity, With<GlitchEffectMarker>>,
    mut player: Single<&mut Player>,
) {
    effect.progress += time.delta_secs() / EFFECT_DURATION;
    player.disable_movement = true;

    if effect.progress > 1.0 {
        effect.progress = 0.0;
        commands.entity(*marker).despawn();
        player.disable_movement = false;
    }
}
