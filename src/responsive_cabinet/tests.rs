use super::*;

#[test]
fn portrait_sort_control_is_a_touch_target_in_the_library_header() {
    let state = AppState {
        cabinet_filter: 9,
        ..Default::default()
    };

    assert!(matches!(
        clicks(&state, vec2(220., 30.)).as_slice(),
        [UiAction::CabinetSort]
    ));
}

#[test]
fn portrait_cards_expose_a_favorite_touch_zone() {
    let state = AppState {
        cabinet_filter: 3,
        ..Default::default()
    };

    assert!(matches!(
        clicks(&state, vec2(155., 120.)).as_slice(),
        [UiAction::ToggleFavorite(_)]
    ));
}
