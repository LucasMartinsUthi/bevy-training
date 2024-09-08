use crate::player::Player;
use bevy::prelude::*;

pub mod health_bar;
pub mod player;

#[derive(Resource, Default)]
pub struct Game {
    pub player: Player,
    pub score: u32,
}
