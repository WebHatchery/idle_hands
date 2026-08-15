//! Idle Hands collection runtime, currently using the Phase 0 template harness.

use macroquad::prelude::*;
use macroquad_toolkit::capture;

mod accessibility;
mod cabinet_art;
mod card_hints;
mod card_render;
mod cards;
mod cosmetics;
mod data;
mod fivefold;
mod fivefold_ui;
mod freecell;
mod freecell_ui;
mod game;
mod game_input;
mod grid;
mod input;
mod library_ui;
mod lights_out;
mod lights_out_ui;
mod mastermind;
mod mastermind_ui;
mod memory_pairs;
mod memory_pairs_ui;
mod minesweeper;
mod minesweeper_ui;
mod nonogram;
mod nonogram_ui;
mod palette_ui;
mod progression;
mod records_ui;
mod responsive_cards;
mod responsive_landscape;
mod responsive_landscape_cards;
mod responsive_landscape_games;
mod responsive_landscape_library;
mod responsive_library;
mod responsive_puzzles;
mod responsive_ui;
mod reversi;
mod reversi_ui;
mod settings_ui;
mod sliding_puzzle;
mod sliding_puzzle_ui;
mod solitaire;
mod solitaire_ui;
mod sound;
mod spider;
mod spider_ui;
mod state;
mod sudoku;
mod sudoku_ui;
mod tic_tac_toe;
mod tic_tac_toe_ui;
mod tutorial_ui;
mod ui;
mod word_search;
mod word_search_ui;

use game::Game;

fn window_conf() -> Conf {
    capture::capture_window_conf(
        "IDLE_HANDS",
        "Idle Hands",
        ui::LOGICAL_WIDTH as i32,
        ui::LOGICAL_HEIGHT as i32,
    )
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    // Screenshot harness: when IDLE_HANDS_CAPTURE_PATH is set, render
    // deterministic frames, write a PNG, and exit. This Phase 0 harness has a
    // single boot state, so the capture just photographs
    // whatever the boot flow lands on.
    if let Some(configs) = capture::CaptureConfig::all_from_env("IDLE_HANDS") {
        for config in configs {
            game.begin_capture_scene(&config.scene);
            capture::run_capture_once(&config, |dt| {
                game.update(dt);
                game.draw();
            })
            .await;
        }
        return;
    }

    loop {
        let dt = get_frame_time().min(0.1);
        game.update(dt);
        game.draw();
        next_frame().await;
    }
}
