//! Regression coverage for the tests module.

use idle_hands::testing::modules::game_variant_ui::*;
use idle_hands::testing::state::{AppState, GameId, Screen};

#[test]
fn rule_card_is_touchable_in_every_layout() {
    let state = AppState {
        screen: Screen::Game(GameId::Game2048),
        ..Default::default()
    };

    idle_hands::testing::ui::with_desktop_layout(|| {
        assert!(clicks(&state, button_rect().center()));
    });
    idle_hands::testing::ui::with_portrait_layout(|| {
        assert!(clicks(&state, button_rect().center()));
    });
    idle_hands::testing::ui::with_compact_landscape_layout(|| {
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

    idle_hands::testing::ui::with_desktop_layout(|| {
        let panel = setup_panel_rect(GameId::Game2048);
        assert!(matches!(
            setup_clicks(&state, setup_option_rect(panel, 1).center()),
            Some(idle_hands::testing::ui::UiAction::Game2048Size(
                idle_hands::testing::game_2048::Game2048Size::Five
            ))
        ));
        assert!(matches!(
            setup_clicks(&state, setup_close_rect(panel).center()),
            Some(idle_hands::testing::ui::UiAction::ToggleGameSetup)
        ));
    });
}
