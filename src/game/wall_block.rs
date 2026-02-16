use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    game::{
        AllAssets, DestroyOnNewLevel, NewLevel,
        environment::{ROAD_SIZE, WALL_Z},
        guide::ChangeGuideText,
        player::Player,
    },
    screens::Screen,
};

pub fn plugin(app: &mut App) {
    app.add_observer(spawn);
}

#[derive(Component)]
struct WallBlock;

#[derive(Component)]
struct Key;

fn spawn(
    new_level: On<NewLevel>,
    mut commands: Commands,
    assets: Res<AllAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if new_level.0 < 2 {
        return;
    }

    let size = Vec2::new(ROAD_SIZE.x, 64.0);

    commands.spawn((
        Name::new("wall block"),
        DestroyOnNewLevel,
        DespawnOnExit(Screen::Gameplay),
        Mesh2d(meshes.add(Rectangle::from_size(size))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        RigidBody::Static,
        Collider::rectangle(size.x, size.y),
        Transform::from_xyz(0.0, ROAD_SIZE.y - 300.0, WALL_Z),
        WallBlock,
    ));
    commands.spawn((
        Name::new("door"),
        DestroyOnNewLevel,
        DespawnOnExit(Screen::Gameplay),
        Sprite::from_image(assets.door.clone()),
        Transform::from_xyz(0.0, ROAD_SIZE.y - 300.0, WALL_Z)
            .with_scale(Vec2::splat(2.0).extend(1.0)),
        WallBlock,
    ));
    commands
        .spawn((
            Name::new("door trigger"),
            DestroyOnNewLevel,
            DespawnOnExit(Screen::Gameplay),
            Transform::from_xyz(0.0, ROAD_SIZE.y - 500.0, WALL_Z),
            Collider::rectangle(ROAD_SIZE.x, 400.0),
            RigidBody::Static,
            Sensor,
            CollisionEventsEnabled,
        ))
        .observe(on_player_enters_trigger);
}

fn on_player_enters_trigger(
    event: On<CollisionStart>,
    mut commands: Commands,
    assets: Res<AllAssets>,
    player: Single<Entity, With<Player>>,
) {
    if event.collider1 != *player && event.collider2 != *player {
        return;
    }
    commands.entity(event.observer()).despawn();

    commands.trigger(ChangeGuideText(String::from(
        "Door, how audacious, now you need to return back for key.",
    )));

    commands
        .spawn((
            Name::new("key"),
            DestroyOnNewLevel,
            DespawnOnExit(Screen::Gameplay),
            Sprite::from_image(assets.key.clone()),
            Transform::from_xyz(0.0, 300.0, 75.0).with_scale(Vec2::splat(2.0).extend(1.0)),
            Collider::circle(64.0),
            RigidBody::Static,
            Sensor,
            CollisionEventsEnabled,
            Key,
        ))
        .observe(on_key_collected);
}

fn on_key_collected(
    event: On<CollisionStart>,
    mut commands: Commands,
    key: Single<Entity, With<Key>>,
    player: Single<Entity, (With<Player>, Without<WallBlock>)>,
    wall_blocks: Query<Entity, (With<WallBlock>, Without<Player>)>,
) {
    if event.collider1 != *player && event.collider2 != *player {
        return;
    }
    commands.entity(*key).despawn();

    commands.trigger(ChangeGuideText(String::from("And now up again...")));

    for entity in wall_blocks {
        commands.entity(entity).despawn();
    }
}
