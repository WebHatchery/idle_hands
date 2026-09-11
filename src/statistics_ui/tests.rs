use super::*;
use crate::{state::Screen, ui};

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

#[test]
fn outside_statistics_controls_are_ignored() {
    let state = AppState {
        screen: Screen::Statistics,
        ..AppState::default()
    };
    ui::with_desktop_layout(|| assert!(clicks(&state, vec2(50., 50.)).is_empty()));
}

#[test]
fn high_contrast_statistics_accents_are_neutral() {
    let state = AppState {
        high_contrast: true,
        ..AppState::default()
    };

    assert_eq!(accent(&state), WHITE);
    assert_eq!(secondary(&state), WHITE);
}
