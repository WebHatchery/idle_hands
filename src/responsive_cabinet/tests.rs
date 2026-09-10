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
