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
fn portrait_statistics_button_opens_the_statistics_shelf() {
    let actions = records_clicks(&AppState::default(), vec2(70., 145.));

    assert!(matches!(actions.as_slice(), [UiAction::Statistics]));
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

#[test]
fn portrait_rule_card_info_opens_drawer_details() {
    let actions = rules_clicks(&AppState::default(), vec2(318., 135.));

    assert!(matches!(actions.as_slice(), [UiAction::Inspect(0)]));
}

#[test]
fn portrait_scrolled_filtered_rule_card_keeps_its_canonical_game() {
    let state = AppState {
        rules_filter: 4,
        library_scroll: 1,
        ..AppState::default()
    };
    let actions = rules_clicks(&state, vec2(30., 120.));

    assert!(matches!(actions.as_slice(), [UiAction::Open(3)]));
}

#[test]
fn portrait_help_routes_all_visible_buttons() {
    assert!(matches!(
        help_clicks(vec2(50., 495.)).as_slice(),
        [UiAction::Tutorials]
    ));
    assert!(matches!(
        help_clicks(vec2(50., 550.)).as_slice(),
        [UiAction::Rules]
    ));
    assert!(matches!(
        help_clicks(vec2(160., 550.)).as_slice(),
        [UiAction::Credits]
    ));
    assert!(matches!(
        help_clicks(vec2(270., 550.)).as_slice(),
        [UiAction::Cabinet]
    ));
}

#[test]
fn portrait_credits_back_is_a_touch_target() {
    assert!(matches!(
        credits_clicks(vec2(50., 670.)).as_slice(),
        [UiAction::Cabinet]
    ));
}
