//! Regression coverage for the tests module.

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
fn desktop_continue_card_routes_through_the_visible_action() {
    let state = AppState::default();

    assert!(matches!(
        clicks(&state, CONTINUE.center()).as_slice(),
        [UiAction::ContinueGame]
    ));
}

#[test]
fn desktop_cards_expose_a_favorite_touch_zone() {
    let state = AppState {
        cabinet_filter: 3,
        ..Default::default()
    };
    let page = crate::cabinet_data::page(&state, crate::cabinet_data::DESKTOP_PAGE_SIZE);
    let index = page
        .games
        .iter()
        .position(|game| crate::cabinet_status::is_available(*game))
        .expect("the demo keeps one card drawer playable");
    let rect = library_rect(index);

    assert!(matches!(
        clicks(&state, vec2(rect.right() - 10., rect.y + 10.)).as_slice(),
        [UiAction::ToggleFavorite(_)]
    ));
}

#[test]
fn desktop_cards_expose_a_drawer_info_touch_zone() {
    let state = AppState {
        cabinet_filter: 9,
        ..Default::default()
    };
    let rect = library_rect(0);

    assert!(matches!(
        clicks(&state, vec2(rect.right() - 72., rect.y + 10.)).as_slice(),
        [UiAction::Inspect(_)]
    ));
}
