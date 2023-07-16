use bevy::{prelude::*, window::PrimaryWindow};

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const RAINBOW_COLORS: [Color; 21] = [
    Color::rgb(1.0, 0.0, 0.0),     // Red
    Color::rgb(0.5, 0.0, 0.0),     // Dark Red
    Color::rgb(1.0, 0.5, 0.5),     // Light Red
    Color::rgb(1.0, 0.5, 0.0),     // Orange
    Color::rgb(0.5, 0.25, 0.0),    // Dark Orange
    Color::rgb(1.0, 0.75, 0.5),    // Light Orange
    Color::rgb(1.0, 1.0, 0.0),     // Yellow
    Color::rgb(0.5, 0.5, 0.0),     // Dark Yellow
    Color::rgb(1.0, 1.0, 0.5),     // Light Yellow
    Color::rgb(0.0, 1.0, 0.0),     // Green
    Color::rgb(0.0, 0.5, 0.0),     // Dark Green
    Color::rgb(0.5, 1.0, 0.5),     // Light Green
    Color::rgb(0.0, 0.0, 1.0),     // Blue
    Color::rgb(0.0, 0.0, 0.5),     // Dark Blue
    Color::rgb(0.5, 0.5, 1.0),     // Light Blue
    Color::rgb(0.75, 0.0, 1.0),    // Indigo
    Color::rgb(0.375, 0.0, 0.5),   // Dark Indigo
    Color::rgb(0.875, 0.5, 1.0),   // Light Indigo
    Color::rgb(1.0, 0.0, 1.0),     // Violet
    Color::rgb(0.5, 0.0, 0.5),     // Dark Violet
    Color::rgb(1.0, 0.5, 1.0),     // Light Violet
];


pub fn main_menu_style(window_query: Query<&Window, With<PrimaryWindow>>) -> Style {
    let window = window_query.get_single().unwrap();

    Style {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect {
            left: Val::Px((window.width() / 2.0) - 120.0),
            right: Val::Px(0.0),
            top: Val::Px(0.0),
            bottom: Val::Px(0.0),
        },
        ..default()
    }
}

pub fn button_style() -> Style {
    Style {
        justify_content: JustifyContent::Default,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn image_style() -> Style {
    Style {
        margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
        ..default()
    }
}

pub fn title_style() -> Style {
    Style {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::PURPLE,
    }
}

pub fn get_score_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}
