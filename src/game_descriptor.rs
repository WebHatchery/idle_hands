//! The single source of truth for cabinet metadata.

use crate::state::GameId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameCategory {
    Cards,
    Logic,
    Board,
    Word,
    Arcade,
    Misc,
}

impl GameCategory {
    pub const fn filter(self) -> u8 {
        match self {
            Self::Cards => 3,
            Self::Logic => 4,
            Self::Board => 5,
            Self::Word => 6,
            Self::Arcade => 7,
            Self::Misc => 8,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Cards => "Cards",
            Self::Logic => "Logic",
            Self::Board => "Board",
            Self::Word => "Word",
            Self::Arcade => "Arcade",
            Self::Misc => "Misc",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionStrategy {
    Record,
    RuntimeOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameDescriptor {
    pub id: GameId,
    pub index: usize,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub save_key: &'static str,
    pub category: GameCategory,
    pub active: bool,
    pub completion: CompletionStrategy,
    pub has_variants: bool,
}

pub const fn entry(
    id: GameId,
    index: usize,
    title: &'static str,
    subtitle: &'static str,
    save_key: &'static str,
    category: GameCategory,
    has_variants: bool,
) -> GameDescriptor {
    GameDescriptor {
        id,
        index,
        title,
        subtitle,
        save_key,
        category,
        active: true,
        completion: CompletionStrategy::Record,
        has_variants,
    }
}

pub const fn runtime_entry(
    id: GameId,
    index: usize,
    title: &'static str,
    subtitle: &'static str,
    save_key: &'static str,
    category: GameCategory,
    has_variants: bool,
) -> GameDescriptor {
    GameDescriptor {
        id,
        index,
        title,
        subtitle,
        save_key,
        category,
        active: true,
        completion: CompletionStrategy::RuntimeOnly,
        has_variants,
    }
}

mod catalog;
pub use catalog::{descriptor, is_demo_build, is_demo_game, ALL};
pub const DEMO_GAMES_PER_CATEGORY: usize = 5;
