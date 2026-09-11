use super::*;
use crate::{state::AppState, ui::UiAction};
use macroquad::prelude::vec2;

#[test]
fn portrait_settings_routes_shared_rows_and_accessibility_controls() {
    let state = AppState::default();

    assert!(matches!(
        settings_clicks(&state, vec2(30., 145.)).as_slice(),
        [UiAction::CycleCardBack]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(30., 195.)).as_slice(),
        [UiAction::CycleBoardTheme]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(30., 245.)).as_slice(),
        [UiAction::CycleSoundSet]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(30., 295.)).as_slice(),
        [UiAction::CycleCabinetDecoration]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(30., 475.)).as_slice(),
        [UiAction::ToggleHighContrast]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(195., 475.)).as_slice(),
        [UiAction::ToggleLargeText]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(195., 430.)).as_slice(),
        [UiAction::CycleSoundVolume]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(30., 380.)).as_slice(),
        [UiAction::ToggleSound]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(195., 380.)).as_slice(),
        [UiAction::ToggleMotion]
    ));
}

#[test]
fn portrait_settings_reset_modal_blocks_background_and_routes_buttons() {
    let state = AppState {
        confirm_reset: true,
        ..AppState::default()
    };

    assert!(settings_clicks(&state, vec2(30., 145.)).is_empty());
    assert!(matches!(
        settings_clicks(&state, vec2(50., 575.)).as_slice(),
        [UiAction::CancelResetData]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(210., 575.)).as_slice(),
        [UiAction::ConfirmResetData]
    ));
}
