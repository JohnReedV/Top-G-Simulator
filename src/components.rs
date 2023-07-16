use bevy::prelude::*;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Star {}

#[derive(Component)]
pub struct FPS {}

#[derive(Component)]
pub struct ScoreComponent {}

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Exit,
}

#[derive(Component)]
pub struct MainMenu {}

#[derive(Component)]
pub struct PlayButton {}

#[derive(Component)]
pub struct QuitButton {}
