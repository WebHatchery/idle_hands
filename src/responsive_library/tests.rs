use super::*;
use crate::state::AppState;
use crate::ui::UiAction;
use macroquad::prelude::vec2;

#[test]
fn portrait_shelf_button_cycles_from_all_to_cards() {
    let state = AppState::default();
    let actions = records_clicks(&state, vec2(270., 145.));

    assert!(matches!(actions.as_slice(), [UiAction::RecordsFilter(3)]));
}

#[test]
fn portrait_rules_shelf_button_cycles_from_all_to_cards() {
    let state = AppState::default();
    let actions = rules_clicks(&state, vec2(270., 45.));

    assert!(matches!(actions.as_slice(), [UiAction::RulesFilter(3)]));
}

#[test]
fn portrait_filtered_rule_card_opens_its_canonical_game() {
    let state = AppState {
        rules_filter: 6,
        ..AppState::default()
    };
    let actions = rules_clicks(&state, vec2(30., 120.));

    assert!(matches!(actions.as_slice(), [UiAction::Open(12)]));
}
