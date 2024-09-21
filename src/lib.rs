// use crate::player::Player;
use bevy::prelude::*;
use bevy::time::Stopwatch;

pub mod health_bar;
// pub mod player;
pub mod weapon;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Default, Component)]
pub struct DamageTimer(pub Stopwatch);

#[derive(Resource, Default)]
pub struct Game {
    pub player: Player,
    pub score: u32,
}

pub const ENEMY_HIT_TICK_SECS: f32 = 0.5;

#[derive(Component)]
pub struct EnemyHitTimer {
    pub timer: Timer,
}

impl Default for EnemyHitTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_HIT_TICK_SECS, TimerMode::Once),
        }
    }
}
