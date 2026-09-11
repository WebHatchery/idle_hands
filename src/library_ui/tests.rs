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

#[test]
fn desktop_rule_card_info_opens_drawer_details() {
    let actions = rules_clicks(&AppState::default(), vec2(360., 209.));

    assert!(matches!(actions.as_slice(), [UiAction::Inspect(0)]));
}

#[test]
fn desktop_rule_info_lane_stays_inside_the_rule_row() {
    let row = Rect::new(152., 193., 230., 32.);
    let info = rule_info_rect(row);

    assert!(info.x > row.x);
    assert!(info.right() <= row.right());
    assert!(info.bottom() <= row.bottom());
}

#[test]
fn desktop_credits_back_is_a_touch_target() {
    assert!(matches!(
        credits_clicks(vec2(1100., 660.)).as_slice(),
        [UiAction::Cabinet]
    ));
}
