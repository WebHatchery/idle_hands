//! Regression coverage for the tests module.

use idle_hands::testing::modules::records_ui::*;
use idle_hands::testing::state::AppState;
use idle_hands::testing::ui::UiAction;
use macroquad::prelude::vec2;

#[test]
fn desktop_shelf_button_cycles_from_all_to_cards() {
    let state = AppState::default();
    let actions = records_clicks(&state, vec2(1000., 228.));

    assert!(matches!(actions.as_slice(), [UiAction::RecordsFilter(3)]));
}
