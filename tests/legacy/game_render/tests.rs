//! Regression coverage for the tests module.

use macroquad::prelude::*;

fn disjoint(first: Rect, second: Rect) -> bool {
    first.right() <= second.x
        || second.right() <= first.x
        || first.bottom() <= second.y
        || second.bottom() <= first.y
}

fn assert_header_lanes_are_disjoint() {
    let replay = if idle_hands::testing::ui::is_compact_landscape() {
        idle_hands::testing::mobile_tutorial_ui::replay_rect(true)
    } else if idle_hands::testing::ui::is_portrait() {
        idle_hands::testing::mobile_tutorial_ui::replay_rect(false)
    } else {
        idle_hands::testing::tutorial_ui::REPLAY_RECT
    };
    let rule = idle_hands::testing::game_variant_ui::button_rect();
    assert!(disjoint(rule, replay), "rule card overlaps tutorial replay");
}

#[test]
fn portrait_header_lanes_leave_tutorial_visible() {
    idle_hands::testing::ui::with_portrait_layout(assert_header_lanes_are_disjoint);
}

#[test]
fn desktop_header_lanes_leave_tutorial_visible() {
    idle_hands::testing::ui::with_desktop_layout(assert_header_lanes_are_disjoint);
}
