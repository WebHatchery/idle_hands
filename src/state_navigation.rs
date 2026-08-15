//! Shared navigation enums kept separate from the larger persisted state.

use crate::state::GameId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Cabinet,
    Game(GameId),
    Help,
    Records,
    Rules,
    Credits,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
