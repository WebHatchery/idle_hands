use crate::state::{AppState, GameId, Screen};
use crate::ui::UiAction;

use super::{make, ResultInfo, ResultKind};

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

fn result<T>(
    state: &AppState,
    game: &T,
    kind: ResultKind,
    explanation: impl Into<String>,
    stats: String,
    primary_action: UiAction,
    primary_label: &'static str,
    secondary_action: UiAction,
    secondary_label: &'static str,
) -> Option<ResultInfo> {
    Some(make(
        state,
        game,
        kind,
        explanation,
        stats,
        primary_action,
        primary_label,
        secondary_action,
        secondary_label,
    ))
}
