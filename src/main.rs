pub mod components;
pub mod events;
pub mod resources;
pub mod styles;
pub mod systems;
pub mod utils;

use events::*;
use resources::*;
use systems::*;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowMode},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Top G Simulator".into(),
                    resolution: (1920., 1080.).into(),
                    mode: WindowMode::Fullscreen,
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .init_resource::<Score>()
        .init_resource::<SpawnEnemyTimer>()
        .init_resource::<Enemies>()
        .init_resource::<FpsTracker>()
        .init_resource::<FirstGame>()
        .init_resource::<SpawnInvinciTimer>()
        .init_resource::<InvinciDurationTimer>()
        .init_resource::<FixMenuTimer>()
        .add_state::<GameState>()
        .add_state::<Invincible>()
        .add_event::<GameStart>()
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(OnEnter(GameState::Menu), spawn_main_menu)
        .add_systems(OnExit(GameState::Menu), spawn_player)
        .add_systems(
            Update,
            (
                interact_with_play_button.run_if(in_state(GameState::Menu)),
                interact_with_quit_button.run_if(in_state(GameState::Menu)),
                window_border_movement.run_if(in_state(GameState::Game)),
                spawn_enemies,
                player_movement.run_if(in_state(GameState::Game)),
                enemy_movement,
                confine_enemy_to_window,
                detect_collision.run_if(in_state(GameState::Game)),
                spawn_stars,
                collect_stars,
                update_score,
                exit_game,
                fps_system,
                game_over_event_receiver,
                despawn_main_menu.run_if(in_state(GameState::Game)),
                game_start_event,
                update_player_colors.run_if(in_state(GameState::Game)),
                spawn_invincibility.run_if(in_state(GameState::Game)),
                collect_invincibility.run_if(in_state(GameState::Game)),
            ),
        )
        .add_systems(
            Update,
            (
                tick_enemy_timer.run_if(in_state(GameState::Game)),
                tick_invinci_duration.run_if(in_state(Invincible::On)),
                disable_invincibility.run_if(in_state(Invincible::On)),
                fix_menu_first_game.run_if(in_state(GameState::Menu)),
            ),
        )
        .run();
}
