//! Regression coverage for the tests module.

use idle_hands::testing::modules::responsive_landscape_cabinet::*;

#[test]
fn compact_cards_expose_a_drawer_info_touch_zone() {
    let state = AppState {
        cabinet_filter: 9,
        ..Default::default()
    };
    let rect = game_rect(0);

    assert!(matches!(
        clicks(&state, vec2(rect.right() - 64., rect.y + 10.)).as_slice(),
        [UiAction::Inspect(_)]
    ));
}
