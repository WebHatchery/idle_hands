use super::*;

#[test]
fn desktop_library_keeps_the_last_page_inside_the_viewport() {
    assert_eq!(crate::cabinet_data::page_start(60, 0, 44), 0);
    assert_eq!(crate::cabinet_data::page_start(60, 44, 44), 16);
    assert_eq!(crate::cabinet_data::page_start(60, 59, 44), 16);
    assert_eq!(crate::cabinet_data::page_start(20, 19, 44), 0);
}

#[test]
fn desktop_sort_control_is_a_touch_target_in_the_library_header() {
    let state = AppState {
        cabinet_filter: 9,
        ..Default::default()
    };

    assert!(matches!(
        clicks(&state, vec2(800., 60.)).as_slice(),
        [UiAction::CabinetSort]
    ));
}

#[test]
fn desktop_cards_expose_a_favorite_touch_zone() {
    let state = AppState {
        cabinet_filter: 3,
        ..Default::default()
    };

    assert!(matches!(
        clicks(&state, vec2(460., 140.)).as_slice(),
        [UiAction::ToggleFavorite(_)]
    ));
}
