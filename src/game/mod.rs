use bevy::prelude::*;
use rand::{SeedableRng, rngs::SmallRng};

use crate::{game::guide::ChangeGuideText, screens::Screen};

mod animation;
mod environment;
mod goal;
mod guide;
mod player;

const RANDOM_SOURCE_SEED: u64 = 0xDEAD_C0DE;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        environment::plugin,
        goal::plugin,
        guide::plugin,
        player::plugin,
    ))
    .init_resource::<LevelNumber>()
    .insert_resource(RandomSource(SmallRng::seed_from_u64(RANDOM_SOURCE_SEED)))
    .add_systems(OnEnter(Screen::Gameplay), trigger_first_level)
    .add_observer(destroy_on_new_level);
}

#[derive(Event)]
pub struct GameOver;

#[derive(Event)]
pub struct NewLevel(pub usize);

#[derive(Resource, Default)]
pub struct LevelNumber(pub usize);

#[derive(Resource)]
pub struct RandomSource(pub SmallRng);

#[derive(Component)]
pub struct DestroyOnNewLevel;

fn trigger_first_level(mut commands: Commands) {
    commands.trigger(NewLevel(0));
}

fn destroy_on_new_level(
    _: On<NewLevel>,
    mut commands: Commands,
    query: Query<Entity, With<DestroyOnNewLevel>>,
) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}
