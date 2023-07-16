pub mod components;
pub mod events;
pub mod resources;
pub mod styles;
pub mod systems;

use bevy::prelude::*;
use events::*;
use resources::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<SpawnEnemyTimer>()
        .init_resource::<Enemies>()
        .init_resource::<FpsTracker>()
        .init_resource::<FirstGame>()
        .add_state::<GameState>()
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
                tick_enemy_timer,
                exit_game,
                fps_system,
                game_over_event_receiver,
                despawn_main_menu.run_if(in_state(GameState::Game)),
                game_start_event,
            ),
        )
        .run();
}
