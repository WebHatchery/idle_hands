//! Regression coverage for the tests module.

use idle_hands::testing::modules::ui::restart_modal::*;
use idle_hands::testing::{AppState, Screen, UiAction};

#[test]
fn variant_confirmation_names_the_rule_card_change() {
    let state = AppState {
        screen: Screen::Game(idle_hands::testing::state::GameId::Game2048),
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
fn restart_modal_buttons_route_in_every_layout() {
    idle_hands::testing::ui::with_desktop_layout(|| {
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
    idle_hands::testing::ui::with_portrait_layout(|| {
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
    idle_hands::testing::ui::with_compact_landscape_layout(|| {
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
