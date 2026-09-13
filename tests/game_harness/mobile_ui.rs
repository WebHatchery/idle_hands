//! Responsive tap-route checks for every game in the cabinet.
//!
//! Desktop routes already have a host contract. These scans exercise the real
//! responsive dispatcher at the same logical sizes used by the capture matrix,
//! so a mobile-only layout cannot silently lose its visible controls.

use idle_hands::testing::{
    with_compact_landscape_layout, with_portrait_layout, AppState, GameId, Screen,
};
use idle_hands::ui::{self, UiAction};
use macroquad::prelude::vec2;
use std::{collections::HashSet, mem::discriminant};

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

    let mut action_kinds = HashSet::new();
    let mut has_recovery_action = false;
    for y in (0..height as usize).step_by(SAMPLE_STEP) {
        for x in (0..width as usize).step_by(SAMPLE_STEP) {
            for action in ui::actions_at(&state, vec2(x as f32 + 5., y as f32 + 5.)) {
                if !matches!(action, UiAction::Cabinet) {
                    action_kinds.insert(discriminant(&action));
                    has_recovery_action |= super::support::is_recovery_action(&action);
                }
            }
        }
    }

    assert!(
        !action_kinds.is_empty(),
        "{} responsive {} surface did not expose a tap target",
        game.title(),
        layout
    );
    assert!(
        action_kinds.len() >= 2,
        "{} responsive {} surface exposed fewer than two distinct game actions",
        game.title(),
        layout
    );
    assert!(
        has_recovery_action,
        "{} responsive {} surface did not expose a visible recovery action",
        game.title(),
        layout
    );
}

#[test]
fn compact_landscape_exposes_a_tap_target_for_every_game() {
    with_compact_landscape_layout(|| {
        for game in GameId::ALL {
            assert_game_exposes_target(game, COMPACT_WIDTH, COMPACT_HEIGHT, "compact landscape");
        }
    });
}

#[test]
fn portrait_exposes_a_tap_target_for_every_game() {
    with_portrait_layout(|| {
        for game in GameId::ALL {
            assert_game_exposes_target(game, PORTRAIT_WIDTH, PORTRAIT_HEIGHT, "portrait");
        }
    });
}
