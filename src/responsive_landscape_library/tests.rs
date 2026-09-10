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
