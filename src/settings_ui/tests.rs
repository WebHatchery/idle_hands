use super::*;
use crate::{state::AppState, ui::UiAction};
use macroquad::prelude::vec2;

#[test]
fn desktop_settings_routes_rows_and_accessibility_controls() {
    let state = AppState::default();

    assert!(matches!(
        settings_clicks(&state, vec2(240., 220.)).as_slice(),
        [UiAction::CycleCardBack]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(240., 270.)).as_slice(),
        [UiAction::CycleBoardTheme]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(240., 320.)).as_slice(),
        [UiAction::CycleSoundSet]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(240., 370.)).as_slice(),
        [UiAction::CycleCabinetDecoration]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(240., 475.)).as_slice(),
        [UiAction::ToggleHighContrast]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(590., 475.)).as_slice(),
        [UiAction::ToggleLargeText]
    ));
}

#[test]
fn desktop_settings_reset_modal_blocks_background_and_routes_buttons() {
    let state = AppState {
        confirm_reset: true,
        ..AppState::default()
    };

    assert!(settings_clicks(&state, vec2(240., 220.)).is_empty());
    assert!(matches!(
        settings_clicks(&state, vec2(450., 390.)).as_slice(),
        [UiAction::CancelResetData]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(670., 390.)).as_slice(),
        [UiAction::ConfirmResetData]
    ));
}
