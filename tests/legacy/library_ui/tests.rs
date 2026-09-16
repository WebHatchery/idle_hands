//! Regression coverage for the tests module.

use super::*;
use crate::state::AppState;
use crate::ui::UiAction;
use macroquad::prelude::vec2;

#[test]
fn desktop_rule_card_info_opens_drawer_details() {
    let actions = rules_clicks(&AppState::default(), vec2(360., 209.));

    assert!(matches!(actions.as_slice(), [UiAction::Inspect(0)]));
}
