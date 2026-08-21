//! Cabinet completion and progress labels shared by each responsive shelf.

use crate::game_descriptor::descriptor;
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
        GameId::Snake => state.records.snake_best_score.is_some(),
        GameId::Breakout => state.records.breakout_best_score.is_some(),
        GameId::HigherLower => state.records.higher_lower_best_score.is_some(),
        GameId::KlondikeGolf => state.records.klondike_golf_best_moves.is_some(),
        GameId::Blackjack => state.records.blackjack_best_wins.is_some(),
        GameId::SpiderSolitaire => state.records.spider_solitaire_best_moves.is_some(),
        GameId::DungeonSweeper => state.records.dungeon_sweeper_best_moves.is_some(),
        GameId::Potion2048 => state.records.potion_2048_best_score.is_some(),
        GameId::TinyTowerDefence => state.records.tiny_tower_defence_best_wave.is_some(),
        GameId::OneRoomRoguelike => state.records.one_room_roguelike_best_score.is_some(),
        GameId::DailyDungeon => state.records.daily_dungeon_best_score.is_some(),
        GameId::DotsBoxes => state.records.dots_boxes_best_score.is_some(),
        GameId::Sokoban => state.records.sokoban_best_moves.is_some(),
        GameId::Mancala => state.records.mancala_best_score.is_some(),
        GameId::Hanoi => state.records.hanoi_best_moves.is_some(),
        GameId::NumberMatch => state.records.number_match_best_moves.is_some(),
        GameId::FloodIt => state.records.flood_it_best_moves.is_some(),
        GameId::ColorSort => state.records.color_sort_best_moves.is_some(),
        GameId::Battleship => state.records.battleship_best_moves.is_some(),
        GameId::WordGrid => state.records.word_grid_best_moves.is_some(),
        GameId::PipeLoop => state.records.pipe_loop_best_moves.is_some(),
        GameId::MazeWalk => state.records.maze_walk_best_moves.is_some(),
        GameId::MatchThree => state.records.match_three_best_score.is_some(),
        GameId::Pyramid => state.records.pyramid_best_moves.is_some(),
        GameId::TriPeaks => state.records.tri_peaks_best_moves.is_some(),
        GameId::Nim => state.records.nim_best_moves.is_some(),
        GameId::WordLadder => state.records.word_ladder_best_moves.is_some(),
        GameId::SpaceInvaders => state.records.space_invaders_best_score.is_some(),
        GameId::Asteroids => state.records.asteroids_best_score.is_some(),
        GameId::Frogger => state.records.frogger_best_score.is_some(),
        GameId::MunchMaze => state.records.munch_maze_best_score.is_some(),
        GameId::BlockStack => state.records.block_stack_best_score.is_some(),
        GameId::TerrainCannon => state.records.terrain_cannon_best_score.is_some(),
        GameId::FlingFury => state.records.fling_fury_best_score.is_some(),
        GameId::PaddleDuel => state.records.paddle_duel_best_score.is_some(),
        GameId::RiddleRoom => state.records.misc_best_moves[0].is_some(),
        GameId::PatternVault => state.records.misc_best_moves[1].is_some(),
        GameId::SumCircuit => state.records.misc_best_moves[2].is_some(),
        GameId::OrbitOrder => state.records.misc_best_moves[3].is_some(),
        GameId::WordForge => state.records.misc_best_moves[4].is_some(),
    };
    if complete {
        "COMPLETE"
    } else if has_progress(state, game) {
        "IN PROGRESS"
    } else {
        "PLAY NOW"
    }
}

pub fn matches_filter(state: &AppState, game: GameId, filter: u8) -> bool {
    match filter {
        1 => status(state, game) != "COMPLETE",
        2 => status(state, game) == "COMPLETE",
        3..=8 => category_filter(game) == filter,
        _ => true,
    }
}

pub const CATEGORY_FILTERS: [u8; 6] = [3, 4, 5, 6, 7, 8];

pub fn category_filter(game: GameId) -> u8 {
    descriptor(game).category.filter()
}

pub fn category_name(filter: u8) -> &'static str {
    crate::game_descriptor::ALL
        .iter()
        .find(|game| game.category.filter() == filter)
        .map_or("All Games", |game| game.category.label())
}

pub fn filter_count(state: &AppState, filter: u8) -> usize {
    GameId::ALL
        .iter()
        .filter(|game| matches_filter(state, **game, filter))
        .count()
}

fn has_progress(state: &AppState, game: GameId) -> bool {
    match game {
        GameId::Game2048 => state.games.game.score > 0 || state.games.game.best > 0,
        GameId::Minesweeper => state.games.minesweeper.status != MineStatus::Ready,
        GameId::Sudoku => state.games.sudoku.moves > 0,
        GameId::Nonogram => state.games.nonogram.moves > 0,
        GameId::Solitaire => state.games.solitaire.moves > 0,
        GameId::FreeCell => state.games.freecell.moves > 0,
        GameId::Yahtzee => state.games.fivefold.roll_number > 0,
        GameId::Reversi => state.games.reversi.moves > 0,
        GameId::LightsOut => state.games.lights_out.moves > 0,
        GameId::TicTacToe => state.games.tic_tac_toe.moves > 0,
        GameId::MemoryPairs => state.games.memory_pairs.moves > 0,
        GameId::SlidingPuzzle => state.games.sliding_puzzle.moves > 0,
        GameId::Mastermind => state.games.mastermind.row > 0,
        GameId::Spider => state.games.spider.moves > 0,
        GameId::WordSearch => state.games.word_search.moves > 0,
        GameId::Hangman => state.games.hangman.moves > 0,
        GameId::ConnectFour => state.games.connect_four.moves > 0,
        GameId::Checkers => state.games.checkers.moves > 0,
        GameId::PegSolitaire => state.games.peg_solitaire.moves > 0,
        GameId::MahjongSolitaire => state.games.mahjong_solitaire.moves > 0,
        GameId::Snake => state.games.snake.moves > 0,
        GameId::Breakout => state.games.breakout.moves > 0,
        GameId::HigherLower => state.games.higher_lower.moves > 0,
        GameId::KlondikeGolf => state.games.klondike_golf.moves > 0,
        GameId::Blackjack => {
            state.games.blackjack.player.len() > 2
                || state.games.blackjack.status != crate::blackjack::BlackjackStatus::Playing
        }
        GameId::SpiderSolitaire => state.games.spider_solitaire.moves > 0,
        GameId::DungeonSweeper => state.games.dungeon_sweeper.moves > 0,
        GameId::Potion2048 => state.games.potion_2048.score > 0 || state.games.potion_2048.best > 0,
        GameId::TinyTowerDefence => {
            state.games.tiny_tower_defence.score > 0 || state.games.tiny_tower_defence.wave > 1
        }
        GameId::OneRoomRoguelike => {
            state.games.one_room_roguelike.score > 0 || state.games.one_room_roguelike.turns > 0
        }
        GameId::DailyDungeon => {
            state.games.daily_dungeon.score > 0 || state.games.daily_dungeon.moves > 0
        }
        GameId::DotsBoxes => state.games.dots_boxes.moves > 0,
        GameId::Sokoban => state.games.sokoban.moves > 0,
        GameId::Mancala => state.games.mancala.moves > 0,
        GameId::Hanoi => state.games.hanoi.moves > 0,
        GameId::NumberMatch => state.games.number_match.moves > 0,
        GameId::FloodIt => state.games.flood_it.moves > 0,
        GameId::ColorSort => state.games.color_sort.moves > 0,
        GameId::Battleship => state.games.battleship.moves > 0,
        GameId::WordGrid => state.games.word_grid.moves > 0,
        GameId::PipeLoop => state.games.pipe_loop.moves > 0,
        GameId::MazeWalk => state.games.maze_walk.moves > 0,
        GameId::MatchThree => state.games.match_three.moves > 0,
        GameId::Pyramid => state.games.pyramid.moves > 0,
        GameId::TriPeaks => state.games.tri_peaks.moves > 0,
        GameId::Nim => state.games.nim.moves > 0,
        GameId::WordLadder => state.games.word_ladder.moves > 0,
        GameId::SpaceInvaders => state.games.space_invaders.moves > 0,
        GameId::Asteroids => state.games.asteroids.moves > 0,
        GameId::Frogger => state.games.frogger.moves > 0,
        GameId::MunchMaze => state.games.munch_maze.moves > 0,
        GameId::BlockStack => state.games.block_stack.moves > 0,
        GameId::TerrainCannon => state.games.terrain_cannon.moves > 0,
        GameId::FlingFury => state.games.fling_fury.moves > 0,
        GameId::PaddleDuel => state.games.paddle_duel.moves > 0,
        GameId::RiddleRoom => state.games.riddle_room.moves > 0,
        GameId::PatternVault => state.games.pattern_vault.moves > 0,
        GameId::SumCircuit => state.games.sum_circuit.moves > 0,
        GameId::OrbitOrder => state.games.orbit_order.moves > 0,
        GameId::WordForge => state.games.word_forge.moves > 0,
    }
}

pub fn color(status: &str) -> Color {
    match status {
        "COMPLETE" => Color::new(0.55, 1., 0.72, 1.),
        _ => Color::new(0.98, 0.75, 0.30, 1.),
    }
}

pub fn is_active(game: GameId) -> bool {
    descriptor(game).active
}

#[cfg(test)]
mod tests;
