//! Regression coverage for the tests module.

use super::*;

fn disjoint(first: Rect, second: Rect) -> bool {
    first.right() <= second.x
        || second.right() <= first.x
        || first.bottom() <= second.y
        || second.bottom() <= first.y
}

fn assert_header_lanes_are_disjoint() {
    let time = time_badge_rect();
    let replay = if crate::ui::is_compact_landscape() {
        crate::mobile_tutorial_ui::replay_rect(true)
    } else if crate::ui::is_portrait() {
        crate::mobile_tutorial_ui::replay_rect(false)
    } else {
        crate::tutorial_ui::REPLAY_RECT
    };
    let rule = crate::game_variant_ui::button_rect();
    assert!(
        disjoint(time, replay),
        "time badge overlaps tutorial replay"
    );
    assert!(disjoint(time, rule), "time badge overlaps the rule card");
}

#[test]
fn portrait_header_lanes_leave_tutorial_visible() {
    crate::ui::with_portrait_layout(assert_header_lanes_are_disjoint);
}

#[test]
fn compact_header_keeps_the_timer_out_of_the_tutorial_lane() {
    crate::ui::with_compact_landscape_layout(|| {
        assert!(!time_badge_is_visible());
    });
}

#[test]
fn desktop_header_lanes_leave_tutorial_visible() {
    crate::ui::with_desktop_layout(assert_header_lanes_are_disjoint);
}
