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

#[test]
fn landscape_rule_card_info_opens_drawer_details() {
    let actions = rules_clicks(&AppState::default(), vec2(366., 96.));

    assert!(matches!(actions.as_slice(), [UiAction::Inspect(0)]));
}

#[test]
fn landscape_rule_info_lane_stays_inside_the_rule_card() {
    let row = Rect::new(30., 68., 360., 56.);
    let info = rule_info_rect(row);

    assert!(info.x > row.x);
    assert!(info.right() <= row.right());
    assert!(info.bottom() <= row.bottom());
}
