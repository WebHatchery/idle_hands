//! Idle Hands collection runtime, currently using the Phase 0 template harness.

use macroquad::prelude::*;
use macroquad_toolkit::capture;

mod accessibility;
mod achievements_ui;
mod battleship;
mod battleship_ui;
mod blackjack;
mod blackjack_ui;
mod breakout;
mod breakout_ui;
mod cabinet_art;
mod cabinet_status;
mod cabinet_ui;
mod card_hints;
mod card_render;
mod cards;
mod checkers;
mod checkers_ui;
mod color_sort;
mod color_sort_ui;
mod connect_four;
mod connect_four_ui;
mod cosmetics;
mod daily_dungeon;
mod daily_dungeon_ui;
mod data;
mod dots_boxes;
mod dots_boxes_ui;
mod dungeon_sweeper;
mod dungeon_sweeper_ui;
mod favorites_ui;
mod fivefold;
mod fivefold_ui;
mod flood_it;
mod flood_it_ui;
mod freecell;
mod freecell_ui;
mod game;
mod game_2048;
mod game_input;
mod grid;
mod hangman;
mod hangman_ui;
mod hanoi;
mod hanoi_ui;
mod higher_lower;
mod higher_lower_ui;
mod input;
mod klondike_golf;
mod klondike_golf_ui;
mod library_ui;
mod lights_out;
mod lights_out_ui;
mod mahjong_solitaire;
mod mahjong_solitaire_ui;
mod mancala;
mod mancala_ui;
mod mastermind;
mod mastermind_ui;
mod match_three;
mod match_three_ui;
mod maze_walk;
mod maze_walk_ui;
mod memory_pairs;
mod memory_pairs_ui;
mod minesweeper;
mod minesweeper_ui;
mod mobile_tutorial_ui;
mod nim;
mod nim_ui;
mod nonogram;
mod nonogram_ui;
mod number_match;
mod number_match_ui;
mod one_room_roguelike;
mod one_room_roguelike_ui;
mod palette_ui;
mod peg_solitaire;
mod peg_solitaire_ui;
mod pipe_loop;
mod pipe_loop_ui;
mod potion_2048;
mod potion_2048_ui;
mod progression;
mod pyramid;
mod pyramid_ui;
mod records_ui;
mod responsive_cards;
mod responsive_landscape;
mod responsive_landscape_cards;
mod responsive_landscape_games;
mod responsive_landscape_library;
mod responsive_library;
mod responsive_puzzles;
mod responsive_sudoku;
mod responsive_ui;
mod reversi;
mod reversi_ui;
mod settings_ui;
mod sliding_puzzle;
mod sliding_puzzle_ui;
mod snake;
mod snake_ui;
mod sokoban;
mod sokoban_ui;
mod solitaire;
mod solitaire_ui;
mod sound;
mod spider;
mod spider_solitaire;
mod spider_solitaire_ui;
mod spider_ui;
mod state;
mod state_navigation;
mod state_records;
mod state_snapshots;
mod sudoku;
mod sudoku_ui;
mod tic_tac_toe;
mod tic_tac_toe_ui;
mod tiny_tower_defence;
mod tiny_tower_defence_ui;
mod tri_peaks;
mod tri_peaks_ui;
mod tutorial_ui;
mod ui;
mod ui_action;
mod word_grid;
mod word_grid_ui;
mod word_ladder;
mod word_ladder_ui;
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
