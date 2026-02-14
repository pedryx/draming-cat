use bevy::prelude::*;

use crate::PausableSystems;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_animations.in_set(PausableSystems));
}

#[derive(Component)]
pub struct SpriteAnimation {
    timer: Timer,
    pub paused: bool,
}

impl SpriteAnimation {
    pub fn new(fps: f32, paused: bool) -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / fps, TimerMode::Repeating),
            paused,
        }
    }
}

fn update_animations(
    time: Res<Time>,
    layouts: Res<Assets<TextureAtlasLayout>>,
    query: Query<(&mut SpriteAnimation, &mut Sprite)>,
) {
    for (mut animation, mut sprite) in query {
        if animation.paused {
            continue;
        }

        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            let atlas = sprite.texture_atlas.as_mut().unwrap();
            atlas.index += 1;

            let layout = layouts.get(&atlas.layout).unwrap();
            if atlas.index >= layout.textures.len() {
                atlas.index = 0;
            }
        }
    }
}
