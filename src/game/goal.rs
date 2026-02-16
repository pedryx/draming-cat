use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    audio::sound_effect_volume,
    game::{DestroyOnNewLevel, LevelNumber, NewLevel, environment::ROAD_SIZE, player::Player},
    screens::Screen,
};

const GOAL_Z: f32 = 10.0;

pub fn plugin(app: &mut App) {
    app.add_observer(spawn_goal);
}

#[derive(Component)]
struct Goal;

fn spawn_goal(
    new_level: On<NewLevel>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 4, 1, None, None);
    let layout = layouts.add(layout);

    let handle = if new_level.0 >= 2 {
        asset_server.load("images/player_cat.png")
    } else {
        asset_server.load("images/cat_bed.png")
    };
    let collider = if new_level.0 >= 2 {
        Collider::capsule(7.5, 35.0)
    } else {
        Collider::circle(20.0)
    };

    commands
        .spawn((
            Name::new("goal"),
            DestroyOnNewLevel,
            DespawnOnExit(Screen::Gameplay),
            Sprite::from_atlas_image(handle, TextureAtlas { layout, index: 0 }),
            Transform::from_xyz(0.0, ROAD_SIZE.y - 160.0, GOAL_Z)
                .with_scale(Vec2::splat(4.0).extend(1.0)),
            collider,
            Sensor,
            CollisionEventsEnabled,
            Goal,
        ))
        .observe(on_player_reaches_goal);
}

fn on_player_reaches_goal(
    event: On<CollisionStart>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut level_number: ResMut<LevelNumber>,
    player: Single<Entity, With<Player>>,
) {
    if event.collider2 != *player {
        return;
    }

    level_number.0 += 1;
    commands.spawn(sound_effect_volume(
        asset_server.load("audio/sound_effects/goal_reached.wav"),
        0.5,
    ));
    commands.trigger(NewLevel(level_number.0));
}
