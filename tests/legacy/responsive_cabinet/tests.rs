//! Regression coverage for the tests module.

use super::*;

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

    let home = AppState::default();
    crate::ui::with_portrait_layout(|| {
        assert!(matches!(
            clicks(&home, vec2(52., 204.)).as_slice(),
            [UiAction::Recent]
        ));
        assert!(matches!(
            clicks(&home, vec2(180., 204.)).as_slice(),
            [UiAction::Open(index)] if *index == GameId::DailyDungeon.index()
        ));
    });
}
