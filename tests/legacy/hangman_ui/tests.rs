//! Regression coverage for the tests module.

use idle_hands::testing::modules::hangman_ui::*;

#[test]
fn portrait_controls_leave_a_right_edge_margin() {
    idle_hands::testing::ui::with_portrait_layout(|| {
        let layout = layout();
        assert!(layout.new_game.right() <= idle_hands::testing::responsive_ui::WIDTH - 8.);
        assert!(layout.keyboard.right() <= idle_hands::testing::responsive_ui::WIDTH - 8.);
    });
}
