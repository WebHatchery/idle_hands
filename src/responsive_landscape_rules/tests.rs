use super::*;
use crate::state::AppState;
use crate::ui::UiAction;
use macroquad::prelude::vec2;

#[test]
fn landscape_rules_shelf_button_cycles_from_all_to_cards() {
    let state = AppState::default();
    let actions = rules_clicks(&state, vec2(700., 20.));

    assert!(matches!(actions.as_slice(), [UiAction::RulesFilter(3)]));
}

#[test]
fn landscape_rule_card_opens_the_canonical_game() {
    let state = AppState::default();
    let actions = rules_clicks(&state, vec2(50., 90.));

    assert!(matches!(actions.as_slice(), [UiAction::Open(0)]));
}
