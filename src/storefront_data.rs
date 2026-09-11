//! Shared game availability semantics for the cabinet and browse shelves.

use crate::{game_descriptor, state::GameId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameAvailability {
    Playable,
    DemoRestricted,
    ComingSoon,
}

impl GameAvailability {
    pub const fn is_playable(self) -> bool {
        matches!(self, Self::Playable)
    }

    pub const fn cabinet_label(self, compact: bool) -> &'static str {
        match self {
            Self::Playable => "",
            Self::DemoRestricted if compact => "FULL VERSION · ITCH.IO",
            Self::DemoRestricted => "FULL VERSION · BUY ON ITCH.IO",
            Self::ComingSoon => "COMING SOON",
        }
    }

    pub const fn browse_label(self) -> &'static str {
        match self {
            Self::Playable => "",
            Self::DemoRestricted => "FULL VERSION",
            Self::ComingSoon => "COMING SOON",
        }
    }

    pub const fn action_label(self) -> &'static str {
        match self {
            Self::Playable => "OPEN",
            Self::DemoRestricted => "FULL",
            Self::ComingSoon => "SOON",
        }
    }

    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Playable => "OPEN",
            Self::DemoRestricted => "FULL",
            Self::ComingSoon => "SOON",
        }
    }
}

pub fn availability_for_build(game: GameId, demo_build: bool) -> GameAvailability {
    let descriptor = game_descriptor::descriptor(game);
    if !descriptor.active {
        GameAvailability::ComingSoon
    } else if demo_build && !game_descriptor::is_demo_game(game) {
        GameAvailability::DemoRestricted
    } else {
        GameAvailability::Playable
    }
}

pub fn availability(game: GameId) -> GameAvailability {
    availability_for_build(game, game_descriptor::is_demo_build())
}

pub fn cabinet_label_for_build(game: GameId, demo_build: bool, compact: bool) -> &'static str {
    let availability = availability_for_build(game, demo_build);
    if availability.is_playable() {
        game_descriptor::descriptor(game).subtitle
    } else {
        availability.cabinet_label(compact)
    }
}

pub fn cabinet_label(game: GameId, compact: bool) -> &'static str {
    cabinet_label_for_build(game, game_descriptor::is_demo_build(), compact)
}

#[cfg(test)]
mod tests;
