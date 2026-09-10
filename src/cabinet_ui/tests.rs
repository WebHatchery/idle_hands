use super::*;

#[test]
fn desktop_library_keeps_the_last_page_inside_the_viewport() {
    assert_eq!(library_page_start(60, 0), 0);
    assert_eq!(library_page_start(60, 44), 16);
    assert_eq!(library_page_start(60, 59), 16);
    assert_eq!(library_page_start(20, 19), 0);
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
