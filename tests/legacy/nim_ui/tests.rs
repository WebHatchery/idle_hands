//! Regression coverage for the tests module.

use idle_hands::testing::modules::nim_ui::*;

#[test]
fn portrait_nim_primary_rows_stay_inside_the_logical_width() {
    idle_hands::testing::ui::with_portrait_layout(|| {
        assert!(heap_rect(2).right() <= idle_hands::testing::responsive_ui::WIDTH);
        assert!(take_rect(2).right() <= idle_hands::testing::responsive_ui::WIDTH);

        let (_, _, new_board, rule) = bottom_rects();
        assert!(new_board.right() <= idle_hands::testing::responsive_ui::WIDTH);
        assert!(rule.right() <= idle_hands::testing::responsive_ui::WIDTH);
    });
}
