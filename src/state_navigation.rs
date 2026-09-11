//! Shared navigation enums kept separate from the larger persisted state.

use crate::state::GameId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Cabinet,
    Game(GameId),
    Help,
    Records,
    Statistics,
    Tutorials,
    Rules,
    Credits,
    Settings,
}

impl Screen {
    pub const fn game(self) -> Option<GameId> {
        match self {
            Self::Game(game) => Some(game),
            _ => None,
        }
    }

    pub const fn is_game(self) -> bool {
        matches!(self, Self::Game(_))
    }
}
