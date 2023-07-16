use bevy::prelude::*;
use rand::{Rng, rngs::StdRng, SeedableRng}; 

#[derive(Resource)]
pub struct Enemies {
    pub value: u32,
}
impl Default for Enemies {
    fn default() -> Enemies {
        Enemies { value: 4 }
    }
}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}
impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

#[derive(Resource)]
pub struct SpawnEnemyTimer {
    pub timer: Timer,
}
impl Default for SpawnEnemyTimer {
    fn default() -> SpawnEnemyTimer {
        SpawnEnemyTimer {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct FpsTracker {
    pub enabled: bool,
    pub fps: u32,
    pub frame_time: f32,
    pub frame_count: u32,
}

impl Default for FpsTracker {
    fn default() -> Self {
        Self {
            enabled: true,
            fps: 0,
            frame_time: 0.0,
            frame_count: 0,
        }
    }
}

impl FpsTracker {
    pub fn update(&mut self, time: Res<Time>) {
        self.frame_time += time.delta_seconds();
        self.frame_count += 1;

        if self.frame_time >= 0.5 {
            self.fps = self.frame_count;
            self.frame_time -= 1.0;
            self.frame_count = 0;
        }
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Game,
}

#[derive(Resource)]
pub struct FirstGame {
    pub value: bool,
}
impl Default for FirstGame {
    fn default() -> FirstGame {
        FirstGame { value: true }
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum Invincible {
    #[default]
    Off,
    On,
}


#[derive(Resource)]
pub struct SpawnInvinciTimer {
    pub timer: Timer,
    pub rng: StdRng,
}

impl Default for SpawnInvinciTimer {
    fn default() -> Self {
        let mut rng = StdRng::from_entropy();
        let random_time = rng.gen_range(0.0..240.0);

        SpawnInvinciTimer {
            timer: Timer::from_seconds(random_time, TimerMode::Repeating),
            rng,
        }
    }
}

#[derive(Resource)]
pub struct InvinciDurationTimer {
    pub timer: Timer,
}
impl Default for InvinciDurationTimer {
    fn default() -> InvinciDurationTimer {
        InvinciDurationTimer {
            timer: Timer::from_seconds(30.0, TimerMode::Repeating),
        }
    }
}