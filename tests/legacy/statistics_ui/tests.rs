//! Regression coverage for the tests module.

use idle_hands::testing::modules::statistics_ui::*;
use idle_hands::testing::{state::Screen, ui};

#[test]
fn back_button_routes_from_every_layout() {
    let state = AppState {
        screen: Screen::Statistics,
        ..AppState::default()
    };
    ui::with_desktop_layout(|| {
        assert!(matches!(
            ui::actions_at(&state, vec2(940., 600.)).as_slice(),
            [UiAction::Records]
        ));
    });
    ui::with_compact_landscape_layout(|| {
        assert!(matches!(
            ui::actions_at(&state, vec2(700., 330.)).as_slice(),
            [UiAction::Records]
        ));
    });
    ui::with_portrait_layout(|| {
        assert!(matches!(
            ui::actions_at(&state, vec2(20., 720.)).as_slice(),
            [UiAction::Records]
        ));
    });
}
