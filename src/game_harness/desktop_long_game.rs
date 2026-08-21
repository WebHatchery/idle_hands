//! Desktop-only long-session checks for every cabinet game.
//!
//! These tests stay at the rule/action boundary used by the desktop screen. They
//! deliberately avoid responsive layout code: the goal is to keep a real game
//! session alive long enough to expose late-game state and input regressions.

use serde::{de::DeserializeOwned, Serialize};

use crate::state::GameId;

mod arcade;
mod boards;
mod cards;
mod puzzles;

pub(super) const SESSION_SEED: u64 = 0xD35A_7A11_2026_0001;

pub(super) fn assert_serializable<T>(game: &T)
where
    T: Serialize + DeserializeOwned,
{
    let encoded = serde_json::to_value(game).expect("desktop session should serialize");
    let restored: T =
        serde_json::from_value(encoded.clone()).expect("desktop session should deserialize");
    assert_eq!(
        encoded,
        serde_json::to_value(restored).expect("restored desktop session should serialize")
    );
}

pub(super) fn run(game: GameId) {
    match game {
        GameId::Solitaire
        | GameId::FreeCell
        | GameId::Spider
        | GameId::KlondikeGolf
        | GameId::Blackjack
        | GameId::SpiderSolitaire
        | GameId::Pyramid
        | GameId::TriPeaks => cards::run(game),
        GameId::Sudoku
        | GameId::Minesweeper
        | GameId::Game2048
        | GameId::Nonogram
        | GameId::Yahtzee
        | GameId::LightsOut
        | GameId::Mastermind
        | GameId::MemoryPairs
        | GameId::SlidingPuzzle
        | GameId::Hangman
        | GameId::WordSearch
        | GameId::WordGrid
        | GameId::WordLadder
        | GameId::RiddleRoom
        | GameId::PatternVault
        | GameId::SumCircuit
        | GameId::OrbitOrder
        | GameId::WordForge => puzzles::run(game),
        GameId::Reversi
        | GameId::TicTacToe
        | GameId::ConnectFour
        | GameId::DungeonSweeper
        | GameId::Checkers
        | GameId::PegSolitaire
        | GameId::MahjongSolitaire
        | GameId::HigherLower
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
        | GameId::PipeLoop
        | GameId::MazeWalk
        | GameId::MatchThree
        | GameId::Nim
        | GameId::TinyTowerDefence => boards::run(game),
        GameId::Snake
        | GameId::Breakout
        | GameId::Potion2048
        | GameId::SpaceInvaders
        | GameId::Asteroids
        | GameId::Frogger
        | GameId::MunchMaze
        | GameId::BlockStack
        | GameId::TerrainCannon
        | GameId::FlingFury
        | GameId::PaddleDuel => arcade::run(game),
    }
}
