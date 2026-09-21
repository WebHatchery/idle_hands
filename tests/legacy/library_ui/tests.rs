//! Regression coverage for the tests module.

use idle_hands::testing::modules::library_ui::*;
use idle_hands::testing::state::AppState;
use idle_hands::testing::ui::UiAction;
use macroquad::prelude::vec2;

#[test]
fn desktop_rule_card_info_opens_drawer_details() {
    let actions = rules_clicks(&AppState::default(), vec2(360., 209.));

    assert!(matches!(actions.as_slice(), [UiAction::Inspect(0)]));
}
