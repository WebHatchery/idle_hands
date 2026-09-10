use super::*;
use crate::state::AppState;
use crate::ui::UiAction;
use macroquad::prelude::vec2;

#[test]
fn desktop_rules_shelf_button_cycles_from_all_to_cards() {
    let state = AppState::default();
    let actions = rules_clicks(&state, vec2(1000., 120.));

    assert!(matches!(actions.as_slice(), [UiAction::RulesFilter(3)]));
}

#[test]
fn desktop_rule_card_opens_the_canonical_game() {
    let state = AppState::default();
    let actions = rules_clicks(&state, vec2(170., 210.));

    assert!(matches!(actions.as_slice(), [UiAction::Open(0)]));
}
