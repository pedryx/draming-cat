use avian2d::prelude::*;
use bevy::prelude::*;
use rand::Rng;

use crate::{
    PausableSystems,
    audio::sound_effect_volume,
    game::{
        DestroyOnNewLevel, LevelNumber, LevelRestart, NewLevel, RandomSource, environment::ROAD_SIZE, guide::ChangeGuideText, player::Player
    },
    screens::Screen,
};

const SPAWN_Y_START: f32 = ROAD_SIZE.y / 3.0;
const SPAWN_Y_END: f32 = ROAD_SIZE.y - 400.0;
const ARROW_Z: f32 = 150.0;
const ARROW_SPEED: f32 = 400.0;

pub fn plugin(app: &mut App) {
    app.add_observer(spawn_arrow_spawner).add_systems(
        Update,
        (handle_arrow_spawning, destroy_out_of_map).in_set(PausableSystems),
    );
}

#[derive(Component)]
struct ArrowSpawner;

#[derive(Component)]
struct Arrow;

fn spawn_arrow_spawner(new_level: On<NewLevel>, mut commands: Commands) {
    if new_level.0 < 1 {
        return;
    }

    commands.spawn((
        Name::new("arrow spawner"),
        ArrowSpawner,
        DestroyOnNewLevel,
        DespawnOnExit(Screen::Gameplay),
    ));

    commands
        .spawn((
            Name::new("arrow guide trigger"),
            DestroyOnNewLevel,
            DespawnOnExit(Screen::Gameplay),
            Collider::rectangle(ROAD_SIZE.x, SPAWN_Y_END - SPAWN_Y_START),
            RigidBody::Static,
            Sensor,
            CollisionEventsEnabled,
            Transform::from_xyz(0.0, ROAD_SIZE.y / 2.0, 0.0),
        ))
        .observe(on_player_in_arrow_area);
}

fn handle_arrow_spawning(
    _: Single<&ArrowSpawner>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_number: Res<LevelNumber>,
    mut random_source: ResMut<RandomSource>,
) {
    if !random_source.0.random_bool(0.05) {
        return;
    }

    let handle = if level_number.0 >= 2 {
        asset_server.load("images/toast.png")
    } else {
        asset_server.load("images/arrow.png")
    };

    let collider = if level_number.0 >= 2 {
        Collider::rectangle(21.0, 21.0)
    } else {
        Collider::rectangle(32.0, 5.0)
    };

    commands
        .spawn((
            Name::new("arrow"),
            DestroyOnNewLevel,
            DespawnOnExit(Screen::Gameplay),
            Arrow,
            Sprite::from_image(handle),
            collider,
            Sensor,
            RigidBody::Kinematic,
            LinearVelocity(Vec2::NEG_X * ARROW_SPEED),
            CollisionEventsEnabled,
            Transform::from_xyz(
                ROAD_SIZE.x + 50.0,
                random_source.0.random_range(SPAWN_Y_START..=SPAWN_Y_END),
                ARROW_Z,
            )
            .with_scale(Vec2::splat(4.0).extend(1.0)),
        ))
        .observe(on_player_hit);
}

fn destroy_out_of_map(mut commands: Commands, arrows: Query<(Entity, &Transform), With<Arrow>>) {
    for (entity, transform) in arrows {
        if transform.translation.x < -ROAD_SIZE.x / 2.0 - 50.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn on_player_hit(
    event: On<CollisionStart>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Single<Entity, With<Player>>,
) {
    if event.collider2 != *player {
        return;
    }

    commands.spawn(sound_effect_volume(
        asset_server.load("audio/sound_effects/cat_hurt.wav"),
        0.4,
    ));
    commands.trigger(LevelRestart);
}

fn on_player_in_arrow_area(
    event: On<CollisionStart>,
    mut commands: Commands,
    level_number: Res<LevelNumber>,
    player: Single<Entity, With<Player>>,
) {
    if event.collider1 == *player && event.collider2 != *player {
        return;
    }
    commands.entity(event.observer()).despawn();

    let text = if level_number.0 >= 2 {
        "Oh no, avoid the toasts."
    } else {
        "Oh no, avoid the arrows."
    };
    commands.trigger(ChangeGuideText(String::from(text)));
}
