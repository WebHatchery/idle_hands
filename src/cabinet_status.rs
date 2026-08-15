//! Cabinet completion and progress labels shared by each responsive shelf.

use crate::minesweeper::MineStatus;
use crate::state::{AppState, GameId};
use macroquad::prelude::Color;

pub fn status(state: &AppState, game: GameId) -> &'static str {
    let complete = match game {
        GameId::Game2048 => state.records.best_2048 >= 2048,
        GameId::Minesweeper => state.records.minesweeper.iter().any(Option::is_some),
        GameId::Sudoku => state.records.sudoku.iter().any(Option::is_some),
        GameId::Nonogram => state.records.nonogram.iter().any(Option::is_some),
        GameId::Solitaire => state.records.solitaire_best_moves.is_some(),
        GameId::FreeCell => state.records.freecell_best_moves.is_some(),
        GameId::Yahtzee => state.records.fivefold_best_total > 0,
        GameId::Reversi => state.records.reversi_best_score > 0,
        GameId::LightsOut => state.records.lights_out_best_moves.is_some(),
        GameId::TicTacToe => state.records.tic_tac_toe_best_moves.is_some(),
        GameId::MemoryPairs => state.records.memory_pairs_best_moves.is_some(),
        GameId::SlidingPuzzle => state.records.sliding_puzzle_best_moves.is_some(),
        GameId::Mastermind => state.records.mastermind_best_rows.is_some(),
        GameId::Spider => state.records.spider_best_moves.is_some(),
        GameId::WordSearch => state.records.word_search_best_moves.is_some(),
        GameId::Hangman => state.records.hangman_best_moves.is_some(),
        GameId::ConnectFour => state.records.connect_four_best_moves.is_some(),
        GameId::Checkers => state.records.checkers_best_moves.is_some(),
        GameId::PegSolitaire => state.records.peg_solitaire_best_moves.is_some(),
        GameId::MahjongSolitaire => state.records.mahjong_solitaire_best_moves.is_some(),
    };
    if complete {
        "COMPLETE"
    } else if has_progress(state, game) {
        "IN PROGRESS"
    } else {
        "PLAY NOW"
    }
}

fn has_progress(state: &AppState, game: GameId) -> bool {
    match game {
        GameId::Game2048 => state.game.score > 0 || state.game.best > 0,
        GameId::Minesweeper => state.minesweeper.status != MineStatus::Ready,
        GameId::Sudoku => state.sudoku.moves > 0,
        GameId::Nonogram => state.nonogram.moves > 0,
        GameId::Solitaire => state.solitaire.moves > 0,
        GameId::FreeCell => state.freecell.moves > 0,
        GameId::Yahtzee => state.fivefold.roll_number > 0,
        GameId::Reversi => state.reversi.moves > 0,
        GameId::LightsOut => state.lights_out.moves > 0,
        GameId::TicTacToe => state.tic_tac_toe.moves > 0,
        GameId::MemoryPairs => state.memory_pairs.moves > 0,
        GameId::SlidingPuzzle => state.sliding_puzzle.moves > 0,
        GameId::Mastermind => state.mastermind.row > 0,
        GameId::Spider => state.spider.moves > 0,
        GameId::WordSearch => state.word_search.moves > 0,
        GameId::Hangman => state.hangman.moves > 0,
        GameId::ConnectFour => state.connect_four.moves > 0,
        GameId::Checkers => state.checkers.moves > 0,
        GameId::PegSolitaire => state.peg_solitaire.moves > 0,
        GameId::MahjongSolitaire => state.mahjong_solitaire.moves > 0,
    }
}

pub fn color(status: &str) -> Color {
    match status {
        "COMPLETE" => Color::new(0.55, 1., 0.72, 1.),
        _ => Color::new(0.98, 0.75, 0.30, 1.),
    }
}
