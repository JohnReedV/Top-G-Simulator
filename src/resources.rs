use bevy::prelude::*;

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
            timer: Timer::from_seconds(10.0, TimerMode::Repeating),
        }
    }
}