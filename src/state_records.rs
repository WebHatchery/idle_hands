//! Persisted best-result records for the collection.

use serde::{Deserialize, Serialize};

use crate::state::GameId;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CollectionRecords {
    #[serde(default)]
    pub elapsed_seconds: Vec<u32>,
    #[serde(default)]
    pub best_time_seconds: Vec<Option<u32>>,
    #[serde(default)]
    pub elapsed_remainder: f32,
    pub best_2048: u32,
    pub minesweeper: [Option<u32>; 4],
    pub sudoku: [Option<u32>; 3],
    pub nonogram: [Option<u32>; 3],
    pub solitaire_best_moves: Option<u32>,
    pub freecell_best_moves: Option<u32>,
    pub fivefold_best_total: u16,
    pub reversi_best_score: u8,
    #[serde(default)]
    pub lights_out_best_moves: Option<u16>,
    #[serde(default)]
    pub tic_tac_toe_best_moves: Option<u8>,
    #[serde(default)]
    pub memory_pairs_best_moves: Option<u16>,
    #[serde(default)]
    pub sliding_puzzle_best_moves: Option<u16>,
    #[serde(default)]
    pub mastermind_best_rows: Option<u8>,
    #[serde(default)]
    pub spider_best_moves: Option<u32>,
    #[serde(default)]
    pub word_search_best_moves: Option<u16>,
    #[serde(default)]
    pub hangman_best_moves: Option<u16>,
    #[serde(default)]
    pub connect_four_best_moves: Option<u8>,
    #[serde(default)]
    pub checkers_best_moves: Option<u16>,
    #[serde(default)]
    pub peg_solitaire_best_moves: Option<u16>,
    #[serde(default)]
    pub mahjong_solitaire_best_moves: Option<u16>,
    #[serde(default)]
    pub snake_best_score: Option<u16>,
    #[serde(default)]
    pub breakout_best_score: Option<u16>,
    #[serde(default)]
    pub higher_lower_best_score: Option<u16>,
    #[serde(default)]
    pub klondike_golf_best_moves: Option<u16>,
    #[serde(default)]
    pub blackjack_best_wins: Option<u16>,
    #[serde(default)]
    pub spider_solitaire_best_moves: Option<u32>,
    #[serde(default)]
    pub dungeon_sweeper_best_moves: Option<u16>,
    #[serde(default)]
    pub potion_2048_best_score: Option<u32>,
    #[serde(default)]
    pub tiny_tower_defence_best_wave: Option<u8>,
    #[serde(default)]
    pub one_room_roguelike_best_score: Option<u32>,
    #[serde(default)]
    pub daily_dungeon_best_score: Option<u32>,
    #[serde(default)]
    pub dots_boxes_best_score: Option<u8>,
    #[serde(default)]
    pub sokoban_best_moves: Option<u16>,
    #[serde(default)]
    pub mancala_best_score: Option<u8>,
    #[serde(default)]
    pub hanoi_best_moves: Option<u16>,
    #[serde(default)]
    pub number_match_best_moves: Option<u16>,
    #[serde(default)]
    pub flood_it_best_moves: Option<u16>,
    #[serde(default)]
    pub color_sort_best_moves: Option<u16>,
    #[serde(default)]
    pub battleship_best_moves: Option<u16>,
    #[serde(default)]
    pub word_grid_best_moves: Option<u8>,
    #[serde(default)]
    pub pipe_loop_best_moves: Option<u16>,
    #[serde(default)]
    pub maze_walk_best_moves: Option<u16>,
    #[serde(default)]
    pub match_three_best_score: Option<u16>,
    #[serde(default)]
    pub pyramid_best_moves: Option<u16>,
    #[serde(default)]
    pub tri_peaks_best_moves: Option<u16>,
    #[serde(default)]
    pub nim_best_moves: Option<u16>,
    #[serde(default)]
    pub word_ladder_best_moves: Option<u16>,
    #[serde(default)]
    pub space_invaders_best_score: Option<u16>,
    #[serde(default)]
    pub asteroids_best_score: Option<u16>,
    #[serde(default)]
    pub frogger_best_score: Option<u16>,
    #[serde(default)]
    pub munch_maze_best_score: Option<u32>,
    #[serde(default)]
    pub block_stack_best_score: Option<u32>,
    #[serde(default)]
    pub terrain_cannon_best_score: Option<u32>,
    #[serde(default)]
    pub fling_fury_best_score: Option<u32>,
    #[serde(default)]
    pub paddle_duel_best_score: Option<u32>,
    #[serde(default)]
    pub misc_best_moves: [Option<u16>; 5],
}

impl CollectionRecords {
    pub fn ensure_time_slots(&mut self) {
        self.elapsed_seconds.resize(GameId::ALL.len(), 0);
        self.best_time_seconds.resize(GameId::ALL.len(), None);
    }

    pub fn record_time(&mut self, game_index: usize) {
        self.ensure_time_slots();
        if game_index >= self.elapsed_seconds.len() {
            return;
        }
        let elapsed = self.elapsed_seconds[game_index];
        self.best_time_seconds[game_index] =
            Some(self.best_time_seconds[game_index].map_or(elapsed, |best| best.min(elapsed)));
    }

    pub fn reset_time(&mut self, game_index: usize) {
        self.ensure_time_slots();
        if let Some(elapsed) = self.elapsed_seconds.get_mut(game_index) {
            *elapsed = 0;
        }
    }

    pub fn current_time(&self, game_index: usize) -> u32 {
        self.elapsed_seconds.get(game_index).copied().unwrap_or(0)
    }

    pub fn best_time(&self, game_index: usize) -> Option<u32> {
        self.best_time_seconds.get(game_index).copied().flatten()
    }
}

#[cfg(test)]
mod tests;
