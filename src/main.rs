//! Idle Hands collection runtime, currently using the Phase 0 template harness.

use macroquad::prelude::*;
use macroquad_toolkit::capture;

mod data;
mod game;
mod minesweeper;
mod state;
mod sudoku;
mod sudoku_ui;
mod ui;

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
