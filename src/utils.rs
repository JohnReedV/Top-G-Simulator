use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::*;
use crate::resources::*;
use crate::styles::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_STARS: usize = 4;

pub fn is_collision(enemy_x: f32, enemy_y: f32, player_x: f32, player_y: f32) -> bool {
    return ((enemy_x - player_x).powi(2) + (enemy_y - player_y).powi(2)).sqrt() <= PLAYER_SIZE;
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    score: &Res<Score>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: main_menu_style(window_query),
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // === Title ===
            parent
                .spawn(NodeBundle {
                    style: title_style(),
                    ..default()
                })
                .with_children(|parent| {
                    // Shadow Text
                    for &offset in &[
                        Vec2::new(-1.0, -1.0),
                        Vec2::new(1.0, -1.0),
                        Vec2::new(-1.0, 1.0),
                        Vec2::new(1.0, 1.0),
                    ] {
                        parent.spawn(TextBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                left: Val::Px(offset.x),
                                top: Val::Px(offset.y),
                                ..Default::default()
                            },
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Top G Simulator",
                                    get_shadow_text_style(&asset_server),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        });
                    }
                    // Main Text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Top G Simulator",
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // === Play Button ===
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style(),
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // === Quit Button ===
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style(),
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Quit",
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        format!("Score: ${}", score.value),
                        get_score_text_style(&asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        })
        .id();

    return main_menu_entity;
}

pub fn build_sound_button(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    window_query: &Query<&Window, With<PrimaryWindow>>,
) -> Entity {
    let window = window_query.single();
    let button_x = window.width() / 2.0;
    let button_y = -window.height() + 50.0;

    let button_bundle = ButtonBundle {
        style: mr_producer_button_style(&window),
        background_color: NORMAL_BUTTON_COLOR.into(),
        border_color: BorderColor(Color::BLACK),
        transform: Transform::from_xyz(button_x, button_y, 0.0),
        ..default()
    };

    let text_bundle = TextBundle {
        text: Text {
            sections: vec![TextSection::new(
                "Toggle Tune",
                get_button_text_style(&asset_server),
            )],
            alignment: TextAlignment::Center,
            ..default()
        },
        ..default()
    };

    return commands
        .spawn((button_bundle, SoundButton {}))
        .with_children(|parent| {
            parent.spawn(text_bundle);
        })
        .id();
}

