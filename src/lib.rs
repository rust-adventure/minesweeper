use bevy::prelude::*;

pub mod assets;
pub mod board;
pub mod pick;

#[derive(
    Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States,
)]
pub enum GameState {
    #[default]
    Startup,
    Menu,
    Playing,
}
