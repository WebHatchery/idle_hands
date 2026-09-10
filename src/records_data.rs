//! Shared record rows used by the responsive records shelves.

use crate::state::{AppState, GameId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordRow {
    pub category: u8,
    pub game: GameId,
    pub label: &'static str,
    pub score: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShelfSummary {
    pub games: usize,
    pub rows: usize,
    pub completed: usize,
}

pub const FILTERS: [u8; 7] = [0, 3, 4, 5, 6, 7, 8];

pub fn filter_label(filter: u8) -> &'static str {
    match filter {
        0 => "ALL",
        3..=8 => crate::cabinet_status::category_name(filter),
        _ => "ALL",
    }
}

pub fn next_filter(filter: u8) -> u8 {
    let current = FILTERS
        .iter()
        .position(|candidate| *candidate == filter)
        .unwrap_or(0);
    FILTERS[(current + 1) % FILTERS.len()]
}

pub fn summary(state: &AppState, filter: u8) -> ShelfSummary {
    let filter = if FILTERS.contains(&filter) { filter } else { 0 };
    let progress = if filter == 0 {
        crate::cabinet_status::collection_progress(state)
    } else {
        crate::cabinet_status::category_progress(state, filter)
    };
    ShelfSummary {
        games: progress.total,
        rows: rows(state, filter).len(),
        completed: progress.completed,
    }
}

pub fn summary_label(state: &AppState, filter: u8) -> String {
    let summary = summary(state, filter);
    format!(
        "{}G / {}R  ·  {} DONE",
        summary.games, summary.rows, summary.completed
    )
}

pub fn rows(state: &AppState, filter: u8) -> Vec<RecordRow> {
    let mut rows = Vec::new();
    add(
        &mut rows,
        GameId::Game2048,
        "2048 best",
        state.records.best_2048.to_string(),
    );
    for (label, score) in [
        ("Mines beginner", value(state.records.minesweeper[0])),
        ("Mines intermediate", value(state.records.minesweeper[1])),
        ("Mines expert", value(state.records.minesweeper[2])),
        ("Mines custom", value(state.records.minesweeper[3])),
    ] {
        add(&mut rows, GameId::Minesweeper, label, score);
    }
    for (label, score) in [
        ("Sudoku easy", value(state.records.sudoku[0])),
        ("Sudoku medium", value(state.records.sudoku[1])),
        ("Sudoku hard", value(state.records.sudoku[2])),
    ] {
        add(&mut rows, GameId::Sudoku, label, score);
    }
    for (label, score) in [
        ("Nonogram 5x5", value(state.records.nonogram[0])),
        ("Nonogram 10x10", value(state.records.nonogram[1])),
        ("Nonogram 15x15", value(state.records.nonogram[2])),
    ] {
        add(&mut rows, GameId::Nonogram, label, score);
    }
    add(
        &mut rows,
        GameId::Solitaire,
        "Solitaire best moves",
        value(state.records.solitaire_best_moves),
    );
    add(
        &mut rows,
        GameId::FreeCell,
        "FreeCell best moves",
        value(state.records.freecell_best_moves),
    );
    add(
        &mut rows,
        GameId::Yahtzee,
        "Fivefold best total",
        state.records.fivefold_best_total.to_string(),
    );
    add(
        &mut rows,
        GameId::Reversi,
        "Reversi best score",
        state.records.reversi_best_score.to_string(),
    );
    add(
        &mut rows,
        GameId::LightsOut,
        "Lights Out best moves",
        value(state.records.lights_out_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::TicTacToe,
        "Tic-Tac-Toe best moves",
        value(state.records.tic_tac_toe_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::MemoryPairs,
        "Memory best moves",
        value(state.records.memory_pairs_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::SlidingPuzzle,
        "Sliding Puzzle best moves",
        value(state.records.sliding_puzzle_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::Mastermind,
        "Mastermind best guesses",
        value(state.records.mastermind_best_rows.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::Spider,
        "Spider best moves",
        value(state.records.spider_best_moves),
    );
    add(
        &mut rows,
        GameId::WordSearch,
        "Word Search best moves",
        value(state.records.word_search_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::Hangman,
        "Hangman best moves",
        value(state.records.hangman_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::ConnectFour,
        "Connect Four best moves",
        value(state.records.connect_four_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::Checkers,
        "Checkers best moves",
        value(state.records.checkers_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::PegSolitaire,
        "Peg Solitaire best moves",
        value(state.records.peg_solitaire_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::MahjongSolitaire,
        "Mahjong Solitaire best moves",
        value(state.records.mahjong_solitaire_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::Snake,
        "Snake best score",
        value(state.records.snake_best_score.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::Breakout,
        "Breakout best score",
        value(state.records.breakout_best_score.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::HigherLower,
        "Higher or Lower best score",
        value(state.records.higher_lower_best_score.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::KlondikeGolf,
        "Klondike Golf best moves",
        value(state.records.klondike_golf_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::Blackjack,
        "Blackjack best wins",
        value(state.records.blackjack_best_wins.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::SpiderSolitaire,
        "Spider Solitaire best moves",
        value(state.records.spider_solitaire_best_moves),
    );
    add(
        &mut rows,
        GameId::DungeonSweeper,
        "Dungeon best",
        value(state.records.dungeon_sweeper_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::Potion2048,
        "Potion 2048 best",
        value(state.records.potion_2048_best_score),
    );
    add(
        &mut rows,
        GameId::TinyTowerDefence,
        "Tower Defence wave",
        value(state.records.tiny_tower_defence_best_wave.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::OneRoomRoguelike,
        "Room Roguelike best",
        value(state.records.one_room_roguelike_best_score),
    );
    add(
        &mut rows,
        GameId::DailyDungeon,
        "Daily Dungeon best",
        value(state.records.daily_dungeon_best_score),
    );
    add(
        &mut rows,
        GameId::DotsBoxes,
        "Dots & Boxes best",
        value(state.records.dots_boxes_best_score.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::Sokoban,
        "Sokoban best moves",
        value(state.records.sokoban_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::Mancala,
        "Mancala best stones",
        value(state.records.mancala_best_score.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::Hanoi,
        "Hanoi best moves",
        value(state.records.hanoi_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::NumberMatch,
        "Number Match best",
        value(state.records.number_match_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::FloodIt,
        "Flood It best",
        value(state.records.flood_it_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::ColorSort,
        "Color Sort best",
        value(state.records.color_sort_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::Battleship,
        "Battleship best",
        value(state.records.battleship_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::WordGrid,
        "Word Grid best",
        value(state.records.word_grid_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::PipeLoop,
        "Pipe Loop best",
        value(state.records.pipe_loop_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::MazeWalk,
        "Maze Walk best",
        value(state.records.maze_walk_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::MatchThree,
        "Match Three best",
        value(state.records.match_three_best_score.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::Pyramid,
        "Pyramid best moves",
        value(state.records.pyramid_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::TriPeaks,
        "TriPeaks best moves",
        value(state.records.tri_peaks_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::Nim,
        "Nim best moves",
        value(state.records.nim_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::WordLadder,
        "Word Ladder best",
        value(state.records.word_ladder_best_moves.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::SpaceInvaders,
        "Space Invaders best",
        value(state.records.space_invaders_best_score.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::Asteroids,
        "Asteroids best",
        value(state.records.asteroids_best_score.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::Frogger,
        "Frogger best",
        value(state.records.frogger_best_score.map(u32::from)),
    );
    add(
        &mut rows,
        GameId::MunchMaze,
        "Munch Maze best",
        value(state.records.munch_maze_best_score),
    );
    add(
        &mut rows,
        GameId::BlockStack,
        "Block Stack best",
        value(state.records.block_stack_best_score),
    );
    add(
        &mut rows,
        GameId::TerrainCannon,
        "Terrain Cannon best",
        value(state.records.terrain_cannon_best_score),
    );
    add(
        &mut rows,
        GameId::FlingFury,
        "Fling Fury best",
        value(state.records.fling_fury_best_score),
    );
    add(
        &mut rows,
        GameId::PaddleDuel,
        "Paddle Duel best",
        value(state.records.paddle_duel_best_score),
    );
    for (game, label, index) in [
        (GameId::RiddleRoom, "Riddle Room best", 0),
        (GameId::PatternVault, "Pattern Vault best", 1),
        (GameId::SumCircuit, "Sum Circuit best", 2),
        (GameId::OrbitOrder, "Orbit Order best", 3),
        (GameId::WordForge, "Word Forge best", 4),
    ] {
        add(
            &mut rows,
            game,
            label,
            value(state.records.misc_best_moves[index].map(u32::from)),
        );
    }
    rows.into_iter()
        .filter(|row| filter == 0 || row.category == filter)
        .collect()
}

fn add(rows: &mut Vec<RecordRow>, game: GameId, label: &'static str, score: String) {
    rows.push(RecordRow {
        category: crate::cabinet_status::category_filter(game),
        game,
        label,
        score,
    });
}

fn value(value: Option<u32>) -> String {
    value.map_or_else(|| "—".into(), |number| number.to_string())
}

#[cfg(test)]
mod tests;
