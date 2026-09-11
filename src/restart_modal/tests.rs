use super::*;

#[test]
fn variant_confirmation_names_the_rule_card_change() {
    let state = AppState {
        screen: Screen::Game(crate::state::GameId::Game2048),
        pending_restart: Some(UiAction::CycleGameVariant),
        ..Default::default()
    };

    assert_eq!(title(&state), "Change the rule card?");
    assert_eq!(
        detail(&state),
        "Current progress will be replaced by the next rule."
    );
}

#[test]
fn ordinary_restart_keeps_the_game_specific_title() {
    let state = AppState {
        screen: Screen::Game(crate::state::GameId::Game2048),
        ..Default::default()
    };

    assert_eq!(title(&state), "Start a new 2048?");
    assert_eq!(detail(&state), "Current progress will be replaced.");
}

#[test]
fn restart_modal_buttons_route_in_every_layout() {
    crate::ui::with_desktop_layout(|| {
        let layout = current_layout();
        assert!(matches!(
            clicks(layout.cancel.center()).as_slice(),
            [UiAction::Cancel]
        ));
        assert!(matches!(
            clicks(layout.start.center()).as_slice(),
            [UiAction::ConfirmRestart]
        ));
    });
    crate::ui::with_portrait_layout(|| {
        let layout = current_layout();
        assert!(matches!(
            clicks(layout.cancel.center()).as_slice(),
            [UiAction::Cancel]
        ));
        assert!(matches!(
            clicks(layout.start.center()).as_slice(),
            [UiAction::ConfirmRestart]
        ));
    });
    crate::ui::with_compact_landscape_layout(|| {
        let layout = current_layout();
        assert!(matches!(
            clicks(layout.cancel.center()).as_slice(),
            [UiAction::Cancel]
        ));
        assert!(matches!(
            clicks(layout.start.center()).as_slice(),
            [UiAction::ConfirmRestart]
        ));
    });
}
