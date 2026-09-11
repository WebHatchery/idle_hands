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
fn portrait_continue_card_routes_through_the_visible_action() {
    let state = AppState::default();

    assert!(matches!(
        clicks(&state, CONTINUE.center()).as_slice(),
        [UiAction::ContinueGame]
    ));
}

#[test]
fn portrait_cards_expose_a_favorite_touch_zone() {
    let state = AppState {
        cabinet_filter: 3,
        ..Default::default()
    };
    let page = crate::cabinet_data::page(&state, crate::cabinet_data::PORTRAIT_PAGE_SIZE);
    let index = page
        .games
        .iter()
        .position(|game| crate::cabinet_status::is_available(*game))
        .expect("the demo keeps one card drawer playable");
    let rect = game_rect(index);

    assert!(matches!(
        clicks(&state, vec2(rect.right() - 10., rect.y + 10.)).as_slice(),
        [UiAction::ToggleFavorite(_)]
    ));
}
