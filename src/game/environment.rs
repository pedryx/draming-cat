use std::f32::consts::PI;

use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
use rand::Rng;

use crate::{
    game::{AllAssets, DestroyOnNewLevel, NewLevel, RandomSource},
    screens::Screen,
};

pub const ROAD_SIZE: Vec2 = Vec2::new(1200.0, 8192.0);
const ROAD_Z: f32 = -100.0;

pub const WALL_Z: f32 = 200.0;
const WALL_THICKNESS: f32 = 4096.0;

const STONE_COUNT: usize = 200;
const STONE_SCALE: f32 = 2.0;
const STONE_Z: f32 = -50.0;
const MOVE_HINT_Z: f32 = -75.0;

pub fn plugin(app: &mut App) {
    app.add_observer(spawn_road).add_observer(spawn_walls);
}

fn spawn_road(
    _: On<NewLevel>,
    mut commands: Commands,
    assets: Res<AllAssets>,
    mut random_source: ResMut<RandomSource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let road_rectangle = Rectangle::from_size(ROAD_SIZE);

    commands.spawn((
        Name::new("road"),
        Mesh2d(meshes.add(road_rectangle)),
        MeshMaterial2d(materials.add(Color::linear_rgb(0.075, 0.522, 0.063))),
        Transform::from_xyz(0.0, ROAD_SIZE.y / 2.0, ROAD_Z),
        DestroyOnNewLevel,
        DespawnOnExit(Screen::Gameplay),
    ));

    commands.spawn((
            Name::new("move hint"),
            DestroyOnNewLevel,
            DespawnOnExit(Screen::Gameplay),
            Sprite::from_image(assets.move_hint.clone()),
            Transform::from_xyz(0.0, 500.0, MOVE_HINT_Z),
        ));

    for _ in 0..STONE_COUNT {
        let stone_index = random_source.0.random_range(1..=3);
        let image = assets.stones[stone_index].clone();

        let position = road_rectangle.sample_interior(&mut random_source.0);
        let rotation = random_source.0.random_range(0.0..2.0 * PI);
        let scale = STONE_SCALE * random_source.0.random_range(0.7..=1.3);

        let position = position + Vec2::Y * ROAD_SIZE.y / 2.0;

        commands.spawn((
            Name::new("stone"),
            DestroyOnNewLevel,
            Sprite::from_image(image),
            Transform::from_translation(position.extend(STONE_Z))
                .with_rotation(Quat::from_rotation_z(rotation))
                .with_scale(Vec2::splat(scale).extend(1.0)),
            DespawnOnExit(Screen::Gameplay),
        ));
    }
}

fn spawn_walls(
    _: On<NewLevel>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut spawn_wall = |pos: Vec2, size: Vec2| {
        commands.spawn((
            Name::new("wall"),
            Mesh2d(meshes.add(Rectangle::from_size(size))),
            MeshMaterial2d(materials.add(Color::BLACK)),
            Transform::from_translation((pos).extend(WALL_Z)),
            RigidBody::Static,
            Collider::rectangle(size.x, size.y),
            DestroyOnNewLevel,
            DespawnOnExit(Screen::Gameplay),
        ));
    };
    // Left Wall
    spawn_wall(
        Vec2::new(-ROAD_SIZE.x / 2.0 - WALL_THICKNESS / 2.0, ROAD_SIZE.y / 2.0),
        Vec2::new(WALL_THICKNESS, ROAD_SIZE.y),
    );
    // Right Wall
    spawn_wall(
        Vec2::new(ROAD_SIZE.x / 2.0 + WALL_THICKNESS / 2.0, ROAD_SIZE.y / 2.0),
        Vec2::new(WALL_THICKNESS, ROAD_SIZE.y),
    );
    // Bottom Wall
    spawn_wall(
        Vec2::new(0.0, -WALL_THICKNESS / 2.0),
        Vec2::new(ROAD_SIZE.x + WALL_THICKNESS * 2.0, WALL_THICKNESS),
    );
    // Top Wall
    spawn_wall(
        Vec2::new(0.0, ROAD_SIZE.y + WALL_THICKNESS / 2.0),
        Vec2::new(ROAD_SIZE.x + WALL_THICKNESS * 2.0, WALL_THICKNESS),
    );
}
