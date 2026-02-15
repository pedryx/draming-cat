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

fn spawn_goal(_: On<NewLevel>, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("goal"),
            DestroyOnNewLevel,
            DespawnOnExit(Screen::Gameplay),
            Sprite::from_image(asset_server.load("images/cat_bed.png")),
            Transform::from_xyz(0.0, ROAD_SIZE.y - 160.0, GOAL_Z)
                .with_scale(Vec2::splat(4.0).extend(1.0)),
            Collider::circle(20.0),
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
