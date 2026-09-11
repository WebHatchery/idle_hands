use super::*;

#[test]
fn compact_header_title_starts_after_the_cabinet_breadcrumb() {
    let breadcrumb_budget = 76.;
    let title_x = std::hint::black_box(COMPACT_HEADER_TITLE_X);
    assert!(title_x > breadcrumb_budget);
    assert!(title_x < 494.);
    let status_x = std::hint::black_box(COMPACT_HEADER_STATUS_X);
    assert!(status_x + 190. < 494.);
}

#[test]
fn desktop_help_routes_all_visible_buttons() {
    let state = crate::state::AppState {
        screen: crate::state::Screen::Help,
        ..Default::default()
    };

    with_desktop_layout(|| {
        assert!(matches!(
            actions_at(&state, vec2(650., 650.)).as_slice(),
            [UiAction::Rules]
        ));
        assert!(matches!(
            actions_at(&state, vec2(850., 650.)).as_slice(),
            [UiAction::Credits]
        ));
        assert!(matches!(
            actions_at(&state, vec2(1080., 650.)).as_slice(),
            [UiAction::Cabinet]
        ));
    });
}
