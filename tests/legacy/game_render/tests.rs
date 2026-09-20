//! Regression coverage for the tests module.

use super::*;

fn disjoint(first: Rect, second: Rect) -> bool {
    first.right() <= second.x
        || second.right() <= first.x
        || first.bottom() <= second.y
        || second.bottom() <= first.y
}

fn assert_header_lanes_are_disjoint() {
    let replay = if crate::ui::is_compact_landscape() {
        crate::mobile_tutorial_ui::replay_rect(true)
    } else if crate::ui::is_portrait() {
        crate::mobile_tutorial_ui::replay_rect(false)
    } else {
        crate::tutorial_ui::REPLAY_RECT
    };
    let rule = crate::game_variant_ui::button_rect();
    assert!(disjoint(rule, replay), "rule card overlaps tutorial replay");
}

#[test]
fn portrait_header_lanes_leave_tutorial_visible() {
    crate::ui::with_portrait_layout(assert_header_lanes_are_disjoint);
}

#[test]
fn desktop_header_lanes_leave_tutorial_visible() {
    crate::ui::with_desktop_layout(assert_header_lanes_are_disjoint);
}
