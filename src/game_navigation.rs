//! Cabinet-to-drawer navigation and recent-history ownership.

use super::Game;
use crate::state::{GameId, Screen};

impl Game {
    pub(super) fn open_game(&mut self, index: usize) {
        let Some(id) = GameId::ALL.get(index).copied() else {
            return;
        };
        self.state.favorites_view = false;
        self.state.recent_view = false;
        self.state.selected = index;
        if crate::cabinet_status::is_active(id) {
            self.state.recent_games.retain(|recent| *recent != id);
            self.state.recent_games.insert(0, id);
            self.state.recent_games.truncate(5);
            self.state.screen = Screen::Game(id);
            self.state.tutorial = (!matches!(
                id,
                GameId::LightsOut
                    | GameId::TicTacToe
                    | GameId::MemoryPairs
                    | GameId::SlidingPuzzle
                    | GameId::Mastermind
                    | GameId::Spider
                    | GameId::WordSearch
                    | GameId::Hangman
                    | GameId::ConnectFour
                    | GameId::Checkers
                    | GameId::PegSolitaire
                    | GameId::MahjongSolitaire
                    | GameId::Snake
                    | GameId::Breakout
                    | GameId::HigherLower
                    | GameId::KlondikeGolf
                    | GameId::Blackjack
                    | GameId::SpiderSolitaire
                    | GameId::DungeonSweeper
                    | GameId::Potion2048
                    | GameId::TinyTowerDefence
                    | GameId::OneRoomRoguelike
                    | GameId::DailyDungeon
                    | GameId::DotsBoxes
                    | GameId::Sokoban
                    | GameId::Mancala
                    | GameId::Hanoi
                    | GameId::NumberMatch
                    | GameId::FloodIt
                    | GameId::ColorSort
                    | GameId::Battleship
                    | GameId::WordGrid
                    | GameId::PipeLoop
                    | GameId::MazeWalk
                    | GameId::MatchThree
                    | GameId::Pyramid
                    | GameId::TriPeaks
                    | GameId::Nim
            ) && !self.state.tutorial_seen[id.index()])
            .then_some(id);
        } else {
            self.notifications
                .info(format!("{} is coming soon", id.title()));
        }
    }
}
