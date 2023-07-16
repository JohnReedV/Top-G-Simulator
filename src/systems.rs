use std::time::Duration;

use crate::components::*;
use crate::events::*;
use crate::resources::*;
use crate::styles::*;
use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_STARS: usize = 4;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            color_index: 14,
        },
    ));
}

pub fn spawn_rainbow_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: RAINBOW_COLORS[0],
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..Default::default()
        })
        .insert(Player {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            color_index: 0,
        });
}

pub fn update_player_colors(
    time: Res<Time>,
    mut query: Query<(&mut Player, &mut Sprite)>,
    invinci_state: Res<State<Invincible>>,
) {
    for (mut player, mut sprite) in query.iter_mut() {
        if *invinci_state.get() == Invincible::On {
            player.timer.tick(time.delta());
            if player.timer.finished() {
                player.color_index = (player.color_index + 1) % RAINBOW_COLORS.len();
                sprite.color = RAINBOW_COLORS[player.color_index];
            }
        } else {
            sprite.color = RAINBOW_COLORS[14]
        }
    }
}

pub fn game_start_event(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut score: ResMut<Score>,
    mut reader: EventReader<GameStart>,
    mut game_state: ResMut<NextState<GameState>>,
    mut number_of_enemies: ResMut<Enemies>,
) {
    if let Some(_game_start) = reader.iter().last() {
        for (enemy_entity, _enemy_transform) in enemy_query.iter_mut() {
            commands.entity(enemy_entity).despawn()
        }
        number_of_enemies.value = 3;
        score.value = 0;
        game_state.set(GameState::Game);
    }
}

pub fn game_over_event_receiver(
    mut reader: EventReader<GameOver>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Some(_game_over) = reader.iter().last() {
        game_state.set(GameState::Menu);
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: ResMut<SpawnEnemyTimer>,
    mut number_of_enemies: ResMut<Enemies>,
    mut enemy_query: Query<(&mut Transform, &mut Enemy), Without<Player>>,
    mut reader: EventReader<GameStart>,
    player_query: Query<&Transform, With<Player>>,
) {
    let mut current_enemies: u32 = 0;
    for (_enemy_entity, _enemy_transform) in enemy_query.iter_mut() {
        current_enemies += 1;
    }
    number_of_enemies.value = current_enemies;

    let mut game_start: bool = false;
    if let Some(_game_start) = reader.iter().last() {
        game_start = true
    }
    let iterations: i32;
    if game_start {
        iterations = 4
    } else {
        iterations = 1
    }

    if enemy_spawn_timer.timer.finished() || game_start {
        for _ in 0..iterations {
            let window = window_query.get_single().unwrap();
            let width = (window.width() / 2.0) - (ENEMY_SIZE / 2.0);
            let height = (window.height() / 2.0) - (ENEMY_SIZE / 2.0);

            let mut random_x = (random::<f32>() * width * 2.0) - width;
            let mut random_y = (random::<f32>() * height * 2.0) - height;

            if let Ok(player_transform) = player_query.get_single() {
                let player_x: f32 = player_transform.translation.x;
                let player_y: f32 = player_transform.translation.y;

                while is_collision(random_x, random_y, player_x, player_y) {
                    random_x = (random::<f32>() * width * 2.0) - width;
                    random_y = (random::<f32>() * height * 2.0) - height;
                }
            }

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

pub fn spawn_invincibility(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut invinci_spawn_timer: ResMut<SpawnInvinciTimer>,
    mut invinci_query: Query<Entity, With<Invinci>>,
    invinci_state: Res<State<Invincible>>,
    time: Res<Time>,
) {
    invinci_spawn_timer.timer.tick(time.delta());

    let mut invinci_exist: bool = false;
    for _invinci_entity in invinci_query.iter_mut() {
        invinci_exist = true;
    }

    if invinci_spawn_timer.timer.just_finished()
        && !invinci_exist
        && *invinci_state != Invincible::On
    {
        let window = window_query.get_single().unwrap();
        let width = (window.width() / 2.0) - (ENEMY_SIZE / 2.0);
        let height = (window.height() / 2.0) - (ENEMY_SIZE / 2.0);

        let random_x = (random::<f32>() * width * 2.0) - width;
        let random_y = (random::<f32>() * height * 2.0) - height;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/flower.png"),
                ..default()
            },
            Invinci {},
        ));

        let random_time = invinci_spawn_timer.rng.gen_range(0..240);
        invinci_spawn_timer
            .timer
            .set_duration(Duration::from_secs(random_time));
        invinci_spawn_timer.timer.reset();
    }
}

pub fn collect_invincibility(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut invinci_state: ResMut<NextState<Invincible>>,
    mut invinci_query: Query<(Entity, &Transform), With<Invinci>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (invinci_entity, invinci_transform) in invinci_query.iter_mut() {
            if is_collision(
                invinci_transform.translation.x,
                invinci_transform.translation.y,
                player_transform.translation.x,
                player_transform.translation.y,
            ) {
                invinci_state.set(Invincible::On);
                commands.entity(invinci_entity).despawn();
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/Invincibility.oga"),
                    ..default()
                });
            }
        }
    }
}

pub fn disable_invincibility(
    invinci_duration_timer: Res<InvinciDurationTimer>,
    mut invinci_state: ResMut<NextState<Invincible>>,
) {
    if invinci_duration_timer.timer.finished() {
        invinci_state.set(Invincible::Off)
    }
}

pub fn tick_invinci_duration(
    mut invinci_duration_timer: ResMut<InvinciDurationTimer>,
    time: Res<Time>,
) {
    invinci_duration_timer.timer.tick(time.delta());
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
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/laserLarge_000.ogg"),
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
    invinci_state: Res<State<Invincible>>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        for (_enemy_entity, enemy_transform) in enemy_query.iter() {
            let player_x = player_transform.translation.x;
            let player_y = player_transform.translation.y;
            let enemy_x = enemy_transform.translation.x;
            let enemy_y = enemy_transform.translation.y;

            if is_collision(enemy_x, enemy_y, player_x, player_y)
                && *invinci_state.get() == Invincible::Off
            {
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

pub fn update_score(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    score: Res<Score>,
    score_component_query: Query<Entity, With<ScoreComponent>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if score.is_changed() {
        for score_component_entity in score_component_query.iter() {
            commands.entity(score_component_entity).despawn();
        }

        let window = window_query.get_single().unwrap();
        let x = -window.width() / 2.0 + 50.0;
        let y = window.height() / 2.0 - 30.0;

        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 30.0,
            color: Color::WHITE,
        };
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(format!("Score: {}", score.value), text_style),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..default()
            },
            ScoreComponent {},
        ));
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
        let x = window.width() / 2.0 - 50.0;
        let y = window.height() / 2.0 - 30.0;

        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 30.0,
            color: Color::WHITE,
        };
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(format!("FPS: {}", tracker.fps), text_style),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..default()
            },
            FPS {},
        ));
    }
}

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut game_start_event_writer: EventWriter<GameStart>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                game_start_event_writer.send(GameStart {});
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    build_main_menu(&mut commands, &asset_server, &score, window_query);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
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
                    // Text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Ball Game",
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
                        format!("Score: {}", score.value),
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
