//! Regression coverage for the tests module.

use idle_hands::testing::modules::spider_ui::*;
use idle_hands::testing::state::AppState;
use idle_hands::testing::ui::UiAction;
use macroquad::prelude::vec2;

#[test]
fn compact_spider_subtitle_stays_before_the_rule_card() {
    let subtitle_x = 250.;
    let rule_card_x = 494.;
    assert!(subtitle_x + 160. < rule_card_x);
}

#[test]
fn desktop_spider_routes_visible_controls_and_face_up_cards() {
    idle_hands::testing::ui::with_desktop_layout(|| {
        let state = AppState::default();
        let layout = layout();
        assert!(matches!(
            clicks(&state, vec2(10., 20.)).as_slice(),
            [UiAction::Cabinet]
        ));
        assert!(matches!(
            clicks(&state, layout.stock.center()).as_slice(),
            [UiAction::SpiderDeal]
        ));
        assert!(matches!(
            clicks(&state, layout.hint.center()).as_slice(),
            [UiAction::SpiderHint]
        ));
        assert!(matches!(
            clicks(&state, layout.undo.center()).as_slice(),
            [UiAction::SpiderUndo]
        ));
        assert!(matches!(
            clicks(&state, layout.new_game.center()).as_slice(),
            [UiAction::SpiderNew]
        ));

        let column = 0;
        let depth = state.games.spider.tableau[column].len() - 1;
        assert!(matches!(
            clicks(&state, layout.card_rect(column, depth).center()).as_slice(),
            [UiAction::SpiderSelect(0, _)]
        ));
    });
}

#[test]
fn compact_and_portrait_spider_keep_the_stock_and_action_row_clickable() {
    fn assert_targets() {
        let state = AppState::default();
        let layout = layout();
        assert!(matches!(
            clicks(&state, layout.stock.center()).as_slice(),
            [UiAction::SpiderDeal]
        ));
        assert!(matches!(
            clicks(&state, layout.hint.center()).as_slice(),
            [UiAction::SpiderHint]
        ));
        assert!(matches!(
            clicks(&state, layout.undo.center()).as_slice(),
            [UiAction::SpiderUndo]
        ));
        assert!(matches!(
            clicks(&state, layout.new_game.center()).as_slice(),
            [UiAction::SpiderNew]
        ));
    }

    idle_hands::testing::ui::with_compact_landscape_layout(assert_targets);
    idle_hands::testing::ui::with_portrait_layout(assert_targets);
}
