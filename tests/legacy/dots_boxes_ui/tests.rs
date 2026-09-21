//! Regression coverage for the tests module.

use idle_hands::testing::modules::dots_boxes_ui::*;

#[test]
fn portrait_scoreline_uses_short_header_copy() {
    let status = portrait_score_status(0, 0, 0);

    assert_eq!(status, "Red 0  •  Blue 0  •  0 moves");
    assert!(status.chars().count() < 30);
}

#[test]
fn portrait_title_uses_a_narrow_header_size() {
    idle_hands::testing::ui::with_portrait_layout(|| assert_eq!(title_size(), 18.));
}
