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
