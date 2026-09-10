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
