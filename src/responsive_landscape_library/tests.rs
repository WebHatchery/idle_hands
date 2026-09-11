use super::*;
use crate::state::AppState;
use crate::ui::UiAction;
use macroquad::prelude::vec2;

#[test]
fn landscape_shelf_button_cycles_from_all_to_cards() {
    let state = AppState::default();
    let actions = records_clicks(&state, vec2(380., 20.));

    assert!(matches!(actions.as_slice(), [UiAction::RecordsFilter(3)]));
}

#[test]
fn landscape_settings_routes_rows_and_accessibility_controls() {
    let state = AppState::default();

    assert!(matches!(
        settings_clicks(&state, vec2(50., 75.)).as_slice(),
        [UiAction::CycleCardBack]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(50., 123.)).as_slice(),
        [UiAction::CycleBoardTheme]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(50., 171.)).as_slice(),
        [UiAction::CycleSoundSet]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(50., 219.)).as_slice(),
        [UiAction::CycleCabinetDecoration]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(460., 180.)).as_slice(),
        [UiAction::ToggleHighContrast]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(640., 180.)).as_slice(),
        [UiAction::ToggleLargeText]
    ));
}

#[test]
fn landscape_settings_reset_modal_blocks_background_and_routes_buttons() {
    let state = AppState {
        confirm_reset: true,
        ..AppState::default()
    };

    assert!(settings_clicks(&state, vec2(50., 75.)).is_empty());
    assert!(matches!(
        settings_clicks(&state, vec2(300., 250.)).as_slice(),
        [UiAction::CancelResetData]
    ));
    assert!(matches!(
        settings_clicks(&state, vec2(470., 250.)).as_slice(),
        [UiAction::ConfirmResetData]
    ));
}

#[test]
fn landscape_help_routes_all_visible_buttons() {
    assert!(matches!(
        help_clicks(vec2(460., 300.)).as_slice(),
        [UiAction::Rules]
    ));
    assert!(matches!(
        help_clicks(vec2(580., 300.)).as_slice(),
        [UiAction::Credits]
    ));
    assert!(matches!(
        help_clicks(vec2(700., 300.)).as_slice(),
        [UiAction::Cabinet]
    ));
}
