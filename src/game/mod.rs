use bevy::prelude::*;
use rand::{SeedableRng, rngs::SmallRng};

use crate::{game::{glitch_effect::SpawnGlitchEffect, guide::ChangeGuideText}, screens::Screen};

mod animation;
mod arrows;
mod environment;
pub mod glitch_effect;
mod goal;
mod guide;
mod player;

const WAKE_UP_LEVEL: usize = 2;
const RANDOM_SOURCE_SEED: u64 = 0xDEAD_C0DE;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        environment::plugin,
        goal::plugin,
        guide::plugin,
        player::plugin,
        glitch_effect::plugin,
        arrows::plugin,
    ))
    .init_resource::<LevelNumber>()
    .insert_resource(RandomSource(SmallRng::seed_from_u64(RANDOM_SOURCE_SEED)))
    .add_systems(OnEnter(Screen::Gameplay), trigger_first_level)
    .add_observer(on_new_level)
    .add_observer(trigger_new_level_on_restart);
}

#[derive(Event)]
pub struct LevelRestart;

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

fn trigger_new_level_on_restart(
    _: On<LevelRestart>,
    mut commands: Commands,
    level_number: Res<LevelNumber>,
) {
    commands.trigger(NewLevel(level_number.0));
}

fn on_new_level(
    new_level: On<NewLevel>,
    mut commands: Commands,
    mut screen: ResMut<NextState<Screen>>,
    query: Query<Entity, With<DestroyOnNewLevel>>,
) {
    if new_level.0 == WAKE_UP_LEVEL {
        screen.set(Screen::Victory);
    }

    if new_level.0 > 0 {
        commands.trigger(SpawnGlitchEffect);
    }
    commands.trigger(ChangeGuideText(String::from("You are a cat, your goal is to reach your bed.")));

    for entity in query {
        commands.entity(entity).despawn();
    }
}
