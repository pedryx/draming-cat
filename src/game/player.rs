use avian2d::prelude::*;
use bevy::prelude::*;
use std::f32::consts::PI;

use crate::{PausableSystems, game::{DestroyOnNewLevel, NewLevel, animation::SpriteAnimation}, screens::Screen};

const PLAYER_SCALE: f32 = 2.0;
const PLAYER_Z: f32 = 100.0;
const PLAYER_SPEED: f32 = 1024.0;

pub fn plugin(app: &mut App) {
    app.add_observer(spawn_player)
        .add_systems(
            Update,
            handle_movement
                .run_if(in_state(Screen::Gameplay))
                .in_set(PausableSystems),
        )
        .add_systems(
            PostUpdate,
            update_follow_camera
                .run_if(in_state(Screen::Gameplay))
                .in_set(PausableSystems),
        );
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    _: On<NewLevel>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 4, 1, None, None);
    let layout = layouts.add(layout);

    commands.spawn((
        Name::new("player"),
        Player,
        Sprite::from_atlas_image(
            asset_server.load("images/player_cat.png"),
            TextureAtlas { layout, index: 0 },
        ),
        SpriteAnimation::new(6.0, true),
        RigidBody::Dynamic,
        Collider::capsule(7.5, 35.0),
        Transform::from_xyz(0.0, 80.0, PLAYER_Z).with_scale(Vec3::splat(PLAYER_SCALE)),
        DestroyOnNewLevel,
        DespawnOnExit(Screen::Gameplay),
    ));
}

fn update_follow_camera(
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    camera.translation = player.translation.with_z(camera.translation.z);
}

fn handle_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Single<
        (
            &Transform,
            &mut LinearVelocity,
            &mut AngularVelocity,
            &mut SpriteAnimation,
        ),
        With<Player>,
    >,
) {
    let (transform, mut linear, mut angular, mut animation) = player.into_inner();

    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    let direction = direction.normalize_or_zero();

    linear.0 = direction * PLAYER_SPEED;

    if direction != Vec2::ZERO {
        let target_rotation = direction.to_angle() - PI / 2.0;
        let current_rotation = transform.rotation.to_euler(EulerRot::XYZ).2;

        let mut delta = target_rotation - current_rotation;
        if delta > PI {
            delta -= 2.0 * PI;
        } else if delta < -PI {
            delta += 2.0 * PI;
        }

        angular.0 = delta * 10.0;

        animation.paused = false;
    } else {
        angular.0 = 0.0;
        animation.paused = true;
    }
}
