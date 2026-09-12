//! Terminal result-content routing for completed cabinet games.

use super::ResultInfo;
use crate::state::{AppState, GameId, Screen};
macro_rules! result_entry {
    (
        $state:expr,
        $game:expr,
        $kind:expr,
        $explanation:expr,
        $stats:expr,
        $primary_action:expr,
        $primary_label:expr,
        $secondary_action:expr,
        $secondary_label:expr $(,)?
    ) => {{
        // The game expression remains part of the call shape so each entry can
        // keep its local game binding for the stats it formats. The named spec
        // owns the result contract that crosses the helper boundary.
        let _ = &$game;
        crate::game_result_ui::make(
            $state,
            crate::game_result_ui::ResultSpec {
                kind: $kind,
                explanation: ($explanation).into(),
                stats: $stats,
                primary_action: $primary_action,
                primary_label: $primary_label,
                secondary_action: $secondary_action,
                secondary_label: $secondary_label,
            },
        )
    }};
}

#[path = "game_result_entries/arcade.rs"]
mod arcade;
#[path = "game_result_entries/board.rs"]
mod board;
#[path = "game_result_entries/cards.rs"]
mod cards;
#[path = "game_result_entries/logic.rs"]
mod logic;
#[path = "game_result_entries/misc.rs"]
mod misc;
#[path = "game_result_entries/words.rs"]
mod words;

pub(super) fn info(state: &AppState) -> Option<ResultInfo> {
    let Screen::Game(game) = state.screen else {
        return None;
    };
    if game == GameId::FlingFury {
        return None;
    }
    match game {
        GameId::Game2048
        | GameId::Minesweeper
        | GameId::Sudoku
        | GameId::Nonogram
        | GameId::LightsOut
        | GameId::MemoryPairs
        | GameId::SlidingPuzzle
        | GameId::Mastermind
        | GameId::DungeonSweeper
        | GameId::Potion2048
        | GameId::NumberMatch
        | GameId::FloodIt
        | GameId::ColorSort
        | GameId::PipeLoop
        | GameId::MazeWalk
        | GameId::MatchThree => logic::info(state, game),
        GameId::Solitaire
        | GameId::FreeCell
        | GameId::Yahtzee
        | GameId::Spider
        | GameId::KlondikeGolf
        | GameId::Blackjack
        | GameId::SpiderSolitaire
        | GameId::Pyramid
        | GameId::TriPeaks => cards::info(state, game),
        GameId::Reversi
        | GameId::TicTacToe
        | GameId::ConnectFour
        | GameId::Checkers
        | GameId::PegSolitaire
        | GameId::MahjongSolitaire
        | GameId::Sokoban
        | GameId::Mancala
        | GameId::Hanoi
        | GameId::Battleship
        | GameId::Nim => board::info(state, game),
        GameId::Snake
        | GameId::Breakout
        | GameId::HigherLower
        | GameId::TinyTowerDefence
        | GameId::OneRoomRoguelike
        | GameId::DailyDungeon
        | GameId::DotsBoxes
        | GameId::SpaceInvaders
        | GameId::Asteroids
        | GameId::Frogger
        | GameId::MunchMaze
        | GameId::BlockStack
        | GameId::TerrainCannon
        | GameId::PaddleDuel => arcade::info(state, game),
        GameId::WordSearch | GameId::Hangman | GameId::WordGrid | GameId::WordLadder => {
            words::info(state, game)
        }
        GameId::RiddleRoom
        | GameId::PatternVault
        | GameId::SumCircuit
        | GameId::OrbitOrder
        | GameId::WordForge => misc::info(state, game),
        GameId::FlingFury => None,
    }
}
