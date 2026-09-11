use super::*;

#[test]
fn compact_sort_control_is_a_touch_target_in_the_library_header() {
    let state = AppState {
        cabinet_filter: 9,
        ..Default::default()
    };

    assert!(matches!(
        clicks(&state, vec2(720., 30.)).as_slice(),
        [UiAction::CabinetSort]
    ));
}

#[test]
fn compact_continue_card_routes_through_the_visible_action() {
    let state = AppState::default();

    assert!(matches!(
        clicks(&state, CONTINUE.center()).as_slice(),
        [UiAction::ContinueGame]
    ));
}

#[test]
fn compact_cards_expose_a_favorite_touch_zone() {
    let state = AppState {
        cabinet_filter: 9,
        ..Default::default()
    };

    assert!(matches!(
        clicks(&state, vec2(306., 82.)).as_slice(),
        [UiAction::ToggleFavorite(_)]
    ));
}
