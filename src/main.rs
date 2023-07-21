#![windows_subsystem = "windows"]

use windows::{
    core::{Result, PCWSTR},
    Win32::{
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{LoadImageW, IMAGE_ICON, LR_DEFAULTSIZE},
    },
};

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
    prelude::*,
    window::{PresentMode, WindowMode},
};

fn build_image() -> Result<()> {
    let _icon = unsafe {
        LoadImageW(
            GetModuleHandleW(None)?,
            PCWSTR(1 as _), // Value must match the `nameID` in the .rc script
            IMAGE_ICON,
            0,
            0,
            LR_DEFAULTSIZE,
        )
    }?;

    Ok(())
}

fn main() {

    let _ = build_image();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Top G Simulator".into(),
                resolution: (1920., 1080.).into(),
                mode: WindowMode::Fullscreen,
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<Score>()
        .init_resource::<SpawnEnemyTimer>()
        .init_resource::<Enemies>()
        .init_resource::<FpsTracker>()
        .init_resource::<FirstGame>()
        .init_resource::<SpawnInvinciTimer>()
        .init_resource::<InvinciDurationTimer>()
        .init_resource::<FixMenuTimer>()
        .init_resource::<MrProducerTimer>()
        .init_resource::<SpawnCoffeeTimer>()
        .add_state::<GameState>()
        .add_state::<Invincible>()
        .add_state::<MrProducerState>()
        .add_event::<GameStart>()
        .add_event::<GameOver>()
        .add_systems(Startup, (spawn_camera, setup_cursor))
        .add_systems(OnEnter(GameState::Menu), (spawn_main_menu, toggle_cursor))
        .add_systems(OnExit(GameState::Menu), (spawn_player, toggle_cursor))
        .add_systems(OnEnter(GameState::Paused), (spawn_main_menu, toggle_cursor))
        .add_systems(OnExit(GameState::Paused), toggle_cursor)
        .add_systems(
            Update,
            (
                game_start_event.before(spawn_enemies),
                spawn_enemies,
                interact_with_play_button.run_if(not(in_state(GameState::Game))),
                interact_with_quit_button.run_if(not(in_state(GameState::Game))),
                window_border_movement.run_if(in_state(GameState::Game)),
                player_movement.run_if(in_state(GameState::Game)),
                enemy_movement.run_if(not(in_state(GameState::Paused))),
                confine_enemy_to_window,
                detect_collision.run_if(in_state(GameState::Game)),
                spawn_stars,
                collect_stars,
                update_score,
                pause_game.run_if(not(in_state(GameState::Menu))),
                fps_system,
                game_over_event_receiver,
                despawn_main_menu.run_if(in_state(GameState::Game)),
                update_player_colors.run_if(in_state(GameState::Game)),
                spawn_invincibility.run_if(in_state(GameState::Game)),
                collect_invincibility.run_if(in_state(GameState::Game)),
                draw_enemy_number,
            ),
        )
        .add_systems(
            Update,
            (
                tick_enemy_timer.run_if(in_state(GameState::Game)),
                fix_menu_first_game.run_if(in_state(GameState::Menu)),
                interact_with_sound_button.run_if(not(in_state(GameState::Game))),
                spawn_coffee.run_if(in_state(GameState::Game)),
                collect_coffee
                    .run_if(in_state(GameState::Game))
                    .before(spawn_stars),
                mr_producer,
            ),
        )
        .run();
}
