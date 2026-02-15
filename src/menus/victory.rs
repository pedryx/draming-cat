use bevy::prelude::*;

use crate::{screens::Screen, theme::widget};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Victory), spawn);
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("CGCG You Woke Up!"),
        GlobalZIndex(2),
        DespawnOnExit(Screen::Victory),
        children![
            widget::header("CGCG You Woke Up!"),
            widget::button("Restart", on_restart_click),
        ],
    ));
}

fn on_restart_click(_: On<Pointer<Click>>, mut screen: ResMut<NextState<Screen>>) {
    screen.set(Screen::Gameplay);
}
