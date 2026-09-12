//! Idle Hands platform entry point.

use idle_hands::{game::Game, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use macroquad::prelude::*;
use macroquad_toolkit::capture;

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
        LOGICAL_WIDTH as i32,
        LOGICAL_HEIGHT as i32,
    )
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;
    if let Err(error) = macroquad_toolkit::ui::ensure_default_ui_font() {
        eprintln!("Idle Hands UI font could not load: {error}");
    }
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
        let frame_seconds = get_frame_time();
        game.note_frame_gap(frame_seconds);
        let dt = frame_seconds.min(0.1);
        game.update(dt);
        game.draw();
        if is_quit_requested() {
            game.end_analytics_session();
            break;
        }
        next_frame().await;
    }
}
