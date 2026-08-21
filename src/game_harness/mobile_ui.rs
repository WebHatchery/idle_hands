//! Responsive tap-route checks for every game in the cabinet.
//!
//! Desktop routes already have a host contract. These scans exercise the real
//! responsive dispatcher at the same logical sizes used by the capture matrix,
//! so a mobile-only layout cannot silently lose its visible controls.

use crate::state::{AppState, GameId, Screen};
use crate::ui::{self, UiAction};
use macroquad::prelude::vec2;

const COMPACT_WIDTH: i32 = 844;
const COMPACT_HEIGHT: i32 = 390;
const PORTRAIT_WIDTH: i32 = 360;
const PORTRAIT_HEIGHT: i32 = 780;
const SAMPLE_STEP: usize = 10;

fn assert_game_exposes_target(game: GameId, width: i32, height: i32, layout: &str) {
    let state = AppState {
        screen: Screen::Game(game),
        selected: game.index(),
        tutorial: None,
        ..AppState::default()
    };

    let has_target = (0..height as usize).step_by(SAMPLE_STEP).any(|y| {
        (0..width as usize).step_by(SAMPLE_STEP).any(|x| {
            ui::actions_at(&state, vec2(x as f32 + 5., y as f32 + 5.))
                .iter()
                .any(|action| !matches!(action, UiAction::Cabinet))
        })
    });

    assert!(
        has_target,
        "{} responsive {} surface did not expose a tap target",
        game.title(),
        layout
    );
}

#[test]
fn compact_landscape_exposes_a_tap_target_for_every_game() {
    ui::with_compact_landscape_layout(|| {
        for game in GameId::ALL {
            assert_game_exposes_target(game, COMPACT_WIDTH, COMPACT_HEIGHT, "compact landscape");
        }
    });
}

#[test]
fn portrait_exposes_a_tap_target_for_every_game() {
    ui::with_portrait_layout(|| {
        for game in GameId::ALL {
            assert_game_exposes_target(game, PORTRAIT_WIDTH, PORTRAIT_HEIGHT, "portrait");
        }
    });
}
