use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_guide);
}

#[derive(Component)]
pub struct GuideText;

fn spawn_guide(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            Pickable {
                should_block_lower: false,
                ..default()
            },
            DespawnOnExit(Screen::Gameplay),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("You're the red dot. Move with your mouse."),
                TextFont {
                    font: asset_server.load("fonts/Super Vanilla.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::linear_rgb(0.4, 0.4, 0.4)),
                TextLayout::new_with_justify(Justify::Center),
                Node {
                    position_type: PositionType::Relative,
                    top: Val::Vh(20.0),
                    ..default()
                },
                GuideText,
                Pickable {
                    should_block_lower: false,
                    ..default()
                },
            ));
        });
}
