use super::*;

#[test]
fn variant_confirmation_names_the_rule_card_change() {
    let state = AppState {
        screen: Screen::Game(crate::state::GameId::Game2048),
        pending_restart: Some(UiAction::CycleGameVariant),
        ..Default::default()
    };

    assert_eq!(title(&state), "Change the rule card?");
}

#[test]
fn ordinary_restart_keeps_the_game_specific_title() {
    let state = AppState {
        screen: Screen::Game(crate::state::GameId::Game2048),
        ..Default::default()
    };

    assert_eq!(title(&state), "Start a new 2048?");
}
