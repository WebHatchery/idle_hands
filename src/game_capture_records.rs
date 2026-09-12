//! Deterministic record fixtures used by the screenshot capture harness.

use crate::{progression, state::AppState};

pub(super) fn apply(state: &mut AppState, scene: &str) {
    match scene {
        "records_progress" => {
            state.records.best_2048 = 2048;
            state.records.solitaire_best_moves = Some(42);
            state.records.word_ladder_best_moves = Some(5);
        }
        "records_cards" => {
            state.records.solitaire_best_moves = Some(42);
            state.records.freecell_best_moves = Some(31);
            state.records.memory_pairs_best_moves = Some(18);
            state.records.spider_best_moves = Some(24);
            state.records_filter = 3;
        }
        "records_arcade" => {
            state.records.snake_best_score = Some(120);
            state.records.breakout_best_score = Some(86);
            state.records.space_invaders_best_score = Some(320);
            state.records.asteroids_best_score = Some(140);
            state.records.frogger_best_score = Some(7);
            state.records.munch_maze_best_score = Some(55);
            state.records.block_stack_best_score = Some(41);
            state.records.terrain_cannon_best_score = Some(62);
            state.records.fling_fury_best_score = Some(73);
            state.records.paddle_duel_best_score = Some(48);
            state.records_filter = 7;
        }
        _ => return,
    }
    let _ = progression::sync_with_content(
        &mut state.achievements,
        &mut state.stamps,
        &state.records,
        &state.content,
    );
}

#[cfg(test)]
#[path = "game_capture_records/tests.rs"]
mod tests;
