use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_guide)
        .add_observer(change_text);
}

#[derive(Component)]
struct GuideText;

#[derive(Event)]
pub struct ChangeGuideText(pub String);

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
                Text::new("You are a cat, your goal is to reach your bed."),
                TextFont {
                    font: asset_server.load("fonts/CantedFX Bold.otf"),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::linear_rgb(1.0, 1.0, 1.0)),
                TextLayout::new_with_justify(Justify::Center),
                Node {
                    position_type: PositionType::Relative,
                    top: Val::Vh(5.0),
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

fn change_text(event: On<ChangeGuideText>, mut guide: Single<&mut Text, With<GuideText>>) {
    guide.0 = event.0.clone();
}
