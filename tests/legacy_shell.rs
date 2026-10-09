//! Retained public-API checks for content and persisted state integrity.

#[path = "legacy/content/tests.rs"]
mod content;
#[path = "legacy/data/tests.rs"]
mod data;
#[path = "legacy/lights_out/tests.rs"]
mod lights_out_solver;
#[path = "legacy/minesweeper/tests.rs"]
mod minesweeper_generation;
#[path = "legacy/nim/tests.rs"]
mod nim_solver;
#[path = "legacy/persistence_models/tests.rs"]
mod persistence_models;
#[path = "legacy/reversi/tests.rs"]
mod reversi_ai;
#[path = "legacy/game_result_ui/tests.rs"]
mod saved_terminal_state;
#[path = "legacy/sokoban/tests.rs"]
mod sokoban_pathfinding;
#[path = "legacy/state/tests.rs"]
mod state_snapshots;
#[path = "legacy/sudoku/tests.rs"]
mod sudoku_generation;
#[path = "legacy/misc_games/tests.rs"]
mod sum_circuit_generation;
