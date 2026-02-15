use avian2d::prelude::*;
use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

use crate::{
    PausableSystems,
    audio::sound_effect_complex,
    game::{DestroyOnNewLevel, NewLevel, RandomSource, animation::SpriteAnimation},
    screens::Screen,
};

const PLAYER_SCALE: f32 = 2.0;
const PLAYER_Z: f32 = 100.0;
const PLAYER_MOVEMENT_SPEED: f32 = 1000.0;
const PLAYER_ROTATION_SPEED: f32 = 8.0;
const PLAYER_WALK_SOUND_PERIOD: f32 = 0.25;

pub fn plugin(app: &mut App) {
    app.add_observer(spawn_player)
        .add_systems(PreUpdate, read_keyboard_input)
        .add_systems(
            Update,
            (
                apply_linear_velocity,
                apply_angular_velocity,
                update_animation,
                play_walking_sound,
            )
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
pub struct Player {
    movement_direction: Vec2,
    walking_sound_timer: Timer,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            movement_direction: Vec2::ZERO,
            walking_sound_timer: Timer::from_seconds(
                PLAYER_WALK_SOUND_PERIOD,
                TimerMode::Repeating,
            ),
        }
    }
}

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
        Player::default(),
        Sprite::from_atlas_image(
            asset_server.load("images/player_cat.png"),
            TextureAtlas { layout, index: 0 },
        ),
        SpriteAnimation::new(6.0, true),
        RigidBody::Dynamic,
        Collider::capsule(7.5, 35.0),
        Transform::from_xyz(0.0, 300.0, PLAYER_Z).with_scale(Vec3::splat(PLAYER_SCALE)),
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

fn read_keyboard_input(keyboard: Res<ButtonInput<KeyCode>>, mut player: Single<&mut Player>) {
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

    player.movement_direction = direction.normalize_or_zero();
}

fn apply_linear_velocity(player: Single<(&Player, &mut LinearVelocity)>) {
    let (player, mut velocity) = player.into_inner();

    velocity.0 = player.movement_direction * PLAYER_MOVEMENT_SPEED;
}

fn apply_angular_velocity(player: Single<(&Player, &Transform, &mut AngularVelocity)>) {
    let (player, transform, mut velocity) = player.into_inner();

    if player.movement_direction == Vec2::ZERO {
        velocity.0 = 0.0;
        return;
    }

    let target_rotation = player.movement_direction.to_angle() - PI / 2.0;
    let current_rotation = transform.rotation.to_euler(EulerRot::XYZ).2;

    let delta = (target_rotation - current_rotation + PI).rem_euclid(2.0 * PI) - PI;
    velocity.0 = delta * PLAYER_ROTATION_SPEED;
}

fn update_animation(player: Single<(&Player, &mut SpriteAnimation)>) {
    let (player, mut animation) = player.into_inner();

    animation.paused = player.movement_direction == Vec2::ZERO;
}

fn play_walking_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut random_source: ResMut<RandomSource>,
    mut player: Single<&mut Player>,
) {
    if player.movement_direction == Vec2::ZERO {
        return;
    }

    player.walking_sound_timer.tick(time.delta());
    if !player.walking_sound_timer.just_finished() {
        return;
    }

    let volume = random_source.0.random_range(0.12..=0.3);
    let speed = random_source.0.random_range(0.25..=2.0);

    let handle = asset_server.load("audio/sound_effects/steps.wav");
    commands.spawn(sound_effect_complex(handle, volume, speed));
}
