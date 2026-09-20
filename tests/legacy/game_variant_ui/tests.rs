//! Regression coverage for the tests module.

use super::*;
use crate::state::{AppState, GameId, Screen};

#[test]
fn rule_card_is_touchable_in_every_layout() {
    let state = AppState {
        screen: Screen::Game(GameId::Game2048),
        ..Default::default()
    };

    crate::ui::with_desktop_layout(|| {
        assert!(clicks(&state, button_rect().center()));
    });
    crate::ui::with_portrait_layout(|| {
        assert!(clicks(&state, button_rect().center()));
    });
    crate::ui::with_compact_landscape_layout(|| {
        assert!(clicks(&state, button_rect().center()));
    });
}

#[test]
fn setup_disclosure_keeps_board_choices_and_close_touchable() {
    let state = AppState {
        screen: Screen::Game(GameId::Game2048),
        game_setup_open: true,
        ..Default::default()
    };

    crate::ui::with_desktop_layout(|| {
        let panel = setup_panel_rect(GameId::Game2048);
        assert!(matches!(
            setup_clicks(&state, setup_option_rect(panel, 1).center()),
            Some(crate::ui::UiAction::Game2048Size(
                crate::game_2048::Game2048Size::Five
            ))
        ));
        assert!(matches!(
            setup_clicks(&state, setup_close_rect(panel).center()),
            Some(crate::ui::UiAction::ToggleGameSetup)
        ));
    });
}
