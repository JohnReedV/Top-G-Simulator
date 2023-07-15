use crate::components::*;
use crate::events::*;
use crate::resources::*;
use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_STARS: usize = 4;

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
    enemy_spawn_timer: ResMut<SpawnEnemyTimer>,
    mut number_of_enemies: ResMut<Enemies>,
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
) {
    let mut current_enemies: u32 = 0;
    for (_enemy_entity, _enemy_transform) in enemy_query.iter_mut() {
        current_enemies += 1;
    }

    if enemy_spawn_timer.timer.finished() {
        number_of_enemies.value += 1;
    }

    for _ in 0..(number_of_enemies.value - current_enemies) {
        let window = window_query.get_single().unwrap();
        let width = (window.width() / 2.0) - (ENEMY_SIZE / 2.0);
        let height = (window.height() / 2.0) - (ENEMY_SIZE / 2.0);

        let random_x = (random::<f32>() * width * 2.0) - width;
        let random_y = (random::<f32>() * height * 2.0) - height;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut star_query: Query<(Entity, &Transform), With<Star>>,
) {
    let mut current_stars: usize = 0;
    for (_star_entity, _star_transform) in star_query.iter_mut() {
        current_stars += 1;
    }

    for _ in 0..(NUMBER_OF_STARS - current_stars) {
        let window = window_query.get_single().unwrap();
        let width = (window.width() / 2.0) - (ENEMY_SIZE / 2.0);
        let height = (window.height() / 2.0) - (ENEMY_SIZE / 2.0);

        let random_x = (random::<f32>() * width * 2.0) - width;
        let random_y = (random::<f32>() * height * 2.0) - height;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn collect_stars(
    mut commands: Commands,
    mut star_query: Query<(Entity, &Transform), With<Star>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter_mut() {
            if is_collision(
                star_transform.translation.x,
                star_transform.translation.y,
                player_transform.translation.x,
                player_transform.translation.y,
            ) {
                let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
                let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");

                let sound_effect = if random::<f32>() > 0.5 {
                    sound_effect_1
                } else {
                    sound_effect_2
                };

                commands.spawn(AudioBundle {
                    source: sound_effect,
                    ..default()
                });
                commands.entity(star_entity).despawn();

                score.value += 1;
            }
        }
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

pub fn detect_collision(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        for (_enemy_entity, enemy_transform) in enemy_query.iter() {
            let player_x = player_transform.translation.x;
            let player_y = player_transform.translation.y;
            let enemy_x = enemy_transform.translation.x;
            let enemy_y = enemy_transform.translation.y;

            if is_collision(enemy_x, enemy_y, player_x, player_y) {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/explosionCrunch_000.ogg"),
                    ..default()
                });
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

fn is_collision(enemy_x: f32, enemy_y: f32, player_x: f32, player_y: f32) -> bool {
    return ((enemy_x - player_x).powi(2) + (enemy_y - player_y).powi(2)).sqrt() <= PLAYER_SIZE;
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

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

pub fn tick_enemy_timer(mut enemy_timer: ResMut<SpawnEnemyTimer>, time: Res<Time>) {
    enemy_timer.timer.tick(time.delta());
}

pub fn exit_game(keyboard_input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

pub fn fps_system(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut tracker: ResMut<FpsTracker>,
    fps_query: Query<Entity, With<FPS>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_released(KeyCode::F) {
        tracker.enabled = !tracker.enabled;
    }

    for fps_entity in fps_query.iter() {
        commands.entity(fps_entity).despawn();
    }

    if tracker.enabled {
        tracker.update(time);


        let window = window_query.get_single().unwrap();
        let padding = 30.0;
        let x = window.width() / 2.0 - padding;
        let y = window.height() / 2.0 - padding;

        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 30.0,
            color: Color::WHITE,
        };
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(tracker.fps.to_string(), text_style),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..default()
            },
            FPS {},
        ));
    }
}
