use super::*;

#[test]
fn portrait_nim_primary_rows_stay_inside_the_logical_width() {
    crate::ui::with_portrait_layout(|| {
        assert!(heap_rect(2).right() <= crate::responsive_ui::WIDTH);
        assert!(take_rect(2).right() <= crate::responsive_ui::WIDTH);

        let (_, _, new_board, rule) = bottom_rects();
        assert!(new_board.right() <= crate::responsive_ui::WIDTH);
        assert!(rule.right() <= crate::responsive_ui::WIDTH);
    });
}
