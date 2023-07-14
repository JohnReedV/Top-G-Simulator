#[allow(unused_variables)]
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_ENEMIES: usize = 4;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_player, spawn_enemies))
        .add_systems(
            Update,
            (
                player_movement,
                window_border_movement,
                enemy_movement,
                confine_enemy_to_window,
            ),
        )
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle { ..default() });

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let width = (window.width() / 2.0) - (ENEMY_SIZE / 2.0);
    let height = (window.height() / 2.0) - (ENEMY_SIZE / 2.0);

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = (rand::random::<f32>() * width * 2.0) - width;
        let random_y = (rand::random::<f32>() * height * 2.0) - height;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(rand::random::<f32>(), rand::random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }

        if direction.length() > 0.0 {
            direction = direction.normalize()
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &mut Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn confine_enemy_to_window(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
) {
    let window = window_query.get_single().unwrap();
    let half_window_width = window.width() / 2.0;
    let half_window_height = window.height() / 2.0;
    let half_enemy_size = ENEMY_SIZE / 2.0;

    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let enemy_x = transform.translation.x;
        let enemy_y = transform.translation.y;

        if enemy_x + half_enemy_size > half_window_width {
            enemy.direction.x *= -1.0;
            transform.translation.x = half_window_width - half_enemy_size;
        }
        if enemy_y + half_enemy_size > half_window_height {
            enemy.direction.y *= -1.0;
            transform.translation.y = half_window_height - half_enemy_size;
        }
        if enemy_x - half_enemy_size < -half_window_width {
            enemy.direction.x *= -1.0;
            transform.translation.x = -half_window_width + half_enemy_size;
        }
        if enemy_y - half_enemy_size < -half_window_height {
            enemy.direction.y *= -1.0;
            transform.translation.y = -half_window_height + half_enemy_size;
        }
    }
}

pub fn window_border_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let player_x = transform.translation.x;
        let player_y = transform.translation.y;
        let window = window_query.get_single().unwrap();
        let half_window_width = window.width() / 2.0;
        let half_window_height = window.height() / 2.0;
        let half_player_size = PLAYER_SIZE / 2.0;

        if player_x + half_player_size > half_window_width {
            transform.translation.x = half_window_width - half_player_size;
        }
        if player_y + half_player_size > half_window_height {
            transform.translation.y = half_window_height - half_player_size;
        }
        if player_x - half_player_size < -half_window_width {
            transform.translation.x = -half_window_width + half_player_size;
        }
        if player_y - half_player_size < -half_window_height {
            transform.translation.y = -half_window_height + half_player_size;
        }
    }
}
