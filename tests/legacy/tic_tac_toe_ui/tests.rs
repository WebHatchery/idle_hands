//! Regression coverage for the tests module.

use super::*;

fn overlaps(left: Rect, right: Rect) -> bool {
    left.x < right.right()
        && right.x < left.right()
        && left.y < right.bottom()
        && right.y < left.bottom()
}

#[test]
fn desktop_layout_separates_header_status_variants_board_and_controls() {
    assert_layout_is_separated(layout_for(LayoutMode::Desktop));
}

#[test]
fn compact_landscape_layout_separates_header_status_variants_board_and_controls() {
    assert_layout_is_separated(layout_for(LayoutMode::CompactLandscape));
}

#[test]
fn compact_status_stays_before_the_rule_card() {
    let layout = layout_for(LayoutMode::CompactLandscape);
    assert!(layout.status.x + 220. < 494.);
    assert!(compact_status_text(TicTacToeStatus::Playing).len() < 30);
}

#[test]
fn portrait_layout_separates_header_status_variants_board_and_controls() {
    assert_layout_is_separated(layout_for(LayoutMode::Portrait));
}

fn assert_layout_is_separated(layout: Layout) {
    assert!(layout.header.y + 24. < layout.levels[0].y);
    assert!(layout.status.y + 8. < layout.levels[0].y);
    assert!(layout
        .levels
        .iter()
        .all(|level| !overlaps(*level, layout.board)));
    assert!(layout.levels.iter().all(|level| {
        !overlaps(*level, layout.new_board)
            && !overlaps(*level, layout.undo)
            && !overlaps(*level, layout.hint)
    }));
    assert!(!overlaps(layout.board, layout.new_board));
    assert!(!overlaps(layout.board, layout.undo));
    assert!(!overlaps(layout.board, layout.hint));
    assert!(!overlaps(layout.new_board, layout.undo));
    assert!(!overlaps(layout.new_board, layout.hint));
    assert!(!overlaps(layout.undo, layout.hint));
}
