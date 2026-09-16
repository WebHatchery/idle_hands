//! Regression coverage for the tests module.

use super::*;
use crate::{state::AppState, ui::UiAction};
use macroquad::prelude::vec2;

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
