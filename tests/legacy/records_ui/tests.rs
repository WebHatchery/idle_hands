//! Regression coverage for the tests module.

use super::*;
use crate::state::AppState;
use crate::ui::UiAction;
use macroquad::prelude::vec2;

#[test]
fn desktop_shelf_button_cycles_from_all_to_cards() {
    let state = AppState::default();
    let actions = records_clicks(&state, vec2(1000., 228.));

    assert!(matches!(actions.as_slice(), [UiAction::RecordsFilter(3)]));
}

#[test]
fn desktop_statistics_button_opens_the_statistics_shelf() {
    let actions = records_clicks(&AppState::default(), vec2(1000., 280.));

    assert!(matches!(actions.as_slice(), [UiAction::Statistics]));
}
