pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use bevy::prelude::*;
use events::*;
use systems::*;
use resources::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<SpawnEnemyTimer>()
        .init_resource::<Enemies>()
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_player)
        .add_systems(
            Update,
            (
                spawn_enemies,
                player_movement,
                window_border_movement,
                enemy_movement,
                confine_enemy_to_window,
                detect_collision,
                spawn_stars,
                collect_stars,
                update_score,
                tick_enemy_timer,
                exit_game,
            ),
        )
        .run();
}