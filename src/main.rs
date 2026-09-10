//! Idle Hands collection runtime.

use macroquad::prelude::*;
use macroquad_toolkit::capture;

mod accessibility;
mod achievements_ui;
mod analytics;
mod asteroids;
mod asteroids_ui;
mod battleship;
mod battleship_ui;
mod blackjack;
mod blackjack_ui;
mod block_stack;
mod block_stack_ui;
mod breakout;
mod breakout_ui;
mod cabinet_status;
mod cabinet_ui;
mod capture_registry;
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
mod daily_archive_ui;
mod daily_challenge;
mod daily_dungeon;
mod daily_dungeon_ui;
mod data;
mod domain;
mod dots_boxes;
mod dots_boxes_ui;
mod dungeon_sweeper;
mod dungeon_sweeper_ui;
mod favorites_ui;
mod fivefold;
mod fivefold_ui;
mod fling_fury;
mod fling_fury_ui;
mod flood_it;
mod flood_it_ui;
mod freecell;
mod freecell_ui;
mod frogger;
mod frogger_ui;
mod game;
mod game_2048;
mod game_actions;
mod game_descriptor;
#[cfg(test)]
mod game_harness;
mod game_input;
mod game_result_ui;
mod game_store;
mod game_variant_ui;
mod game_variants;
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
mod mascots;
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
mod misc_games;
mod misc_ui;
mod mobile_tutorial_ui;
mod munch_maze;
mod munch_maze_ui;
mod nim;
mod nim_ui;
mod nonogram;
mod nonogram_ui;
mod number_match;
mod number_match_ui;
mod one_room_roguelike;
mod one_room_roguelike_ui;
mod paddle_duel;
mod paddle_duel_ui;
mod palette_ui;
mod peg_solitaire;
mod peg_solitaire_ui;
mod persistence_models;
mod pipe_loop;
mod pipe_loop_ui;
mod potion_2048;
mod potion_2048_ui;
mod progression;
mod pyramid;
mod pyramid_ui;
mod records_ui;
#[cfg(test)]
mod responsive_bounds;
mod responsive_cabinet;
mod responsive_cards;
mod responsive_fivefold;
mod responsive_landscape;
mod responsive_landscape_cabinet;
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
mod space_invaders;
mod space_invaders_ui;
mod spider;
mod spider_solitaire;
mod spider_solitaire_ui;
mod spider_ui;
mod state;
mod state_navigation;
mod state_records;
mod state_snapshots;
mod storefront;
mod sudoku;
mod sudoku_ui;
mod terrain_cannon;
mod terrain_cannon_ui;
mod theme;
mod tic_tac_toe;
mod tic_tac_toe_ui;
mod tiny_tower_defence;
mod tiny_tower_defence_ui;
mod tri_peaks;
mod tri_peaks_ui;
mod tutorial_ui;
mod ui;
mod ui_action;
mod ui_game_routes;
mod undo;
mod word_grid;
mod word_grid_ui;
mod word_ladder;
mod word_ladder_ui;
mod word_search;
mod word_search_ui;

use game::Game;

async fn prepare_webgl_font_atlas() {
    const ATLAS_GROWTH_SIZES: &[u16] = &[16, 24, 32, 48, 64, 84];
    let mut characters = Font::latin_character_list();
    characters.extend(" -+/$|'?_<>;=~&…—–×‹·".chars());
    characters.sort_unstable();
    characters.dedup();
    let sample: String = characters.into_iter().collect();

    // Cache enough representative glyphs to grow the shared atlas before any
    // real UI is batched. Growing it midway through a WebGL frame invalidates
    // the texture referenced by draw calls already queued for that frame.
    for size in ATLAS_GROWTH_SIZES {
        macroquad_toolkit::ui::measure_ui_text(&sample, None, *size, 1.0);
    }
    for size in ATLAS_GROWTH_SIZES {
        macroquad_toolkit::ui::draw_ui_text(&sample, -10_000.0, -10_000.0, f32::from(*size), WHITE);
    }
    next_frame().await;
}

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
    macroquad_toolkit::ui::ensure_default_ui_font().expect("bundled UI font should load");
    prepare_webgl_font_atlas().await;

    // Screenshot harness: when IDLE_HANDS_CAPTURE_PATH is set, render
    // deterministic frames, write a PNG, and exit.
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

    prevent_quit();
    loop {
        let dt = get_frame_time().min(0.1);
        game.update(dt);
        game.draw();
        if is_quit_requested() {
            game.end_analytics_session();
            break;
        }
        next_frame().await;
    }
}
