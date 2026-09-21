//! Regression coverage for the tests module.

use idle_hands::testing::modules::cabinet_ui::*;

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
