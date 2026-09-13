//! Deterministic, non-playing hints for the two card games.

use crate::state::{AppState, GameId};

#[path = "card_hints/core_boards.rs"]
mod core_boards;
#[path = "card_hints/core_cards.rs"]
mod core_cards;
#[path = "card_hints_postlaunch.rs"]
mod postlaunch;

pub use core_boards::{
    checkers, connect_four, game_2048, hangman, is_hint, lights_out, mahjong_solitaire, mastermind,
    memory_pairs, minesweeper, nonogram, peg_solitaire, reversi, sliding_puzzle, sudoku,
    tic_tac_toe, word_ladder, word_search,
};
pub use core_cards::{
    fivefold, freecell, klondike_golf, nim, pyramid, solitaire, spider, spider_solitaire, tri_peaks,
};

pub use postlaunch::{
    battleship, blackjack, breakout, color_sort, daily_dungeon, dots_boxes, dungeon_sweeper,
    flood_it, hanoi, higher_lower, mancala, match_three, maze_walk, number_match,
    one_room_roguelike, pipe_loop, potion_2048, snake, sokoban, tiny_tower_defence, word_grid,
};

fn color_name(color: u8) -> &'static str {
    ["red", "amber", "green", "blue", "violet", "gold"][color as usize % 6]
}

pub(crate) fn authored_copy(state: &AppState, game: GameId, complete: bool) -> String {
    state
        .content
        .hints
        .get(game.key())
        .map(|copy| {
            if complete {
                copy.complete.clone()
            } else {
                copy.fallback.clone()
            }
        })
        .unwrap_or_else(|| "No hint is available for this drawer.".into())
}

#[cfg(test)]
#[path = "../tests/legacy/card_hints/tests.rs"]
mod tests;
