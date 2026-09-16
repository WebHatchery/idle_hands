//! Regression coverage for the tests module.

use super::*;
use crate::state::AppState;
use crate::ui::UiAction;
use macroquad::prelude::vec2;

#[test]
fn portrait_scrolled_filtered_rule_card_keeps_its_canonical_game() {
    let state = AppState {
        rules_filter: 4,
        library_scroll: 1,
        ..AppState::default()
    };
    let actions = rules_clicks(&state, vec2(30., 120.));

    assert!(matches!(actions.as_slice(), [UiAction::Open(3)]));
}
