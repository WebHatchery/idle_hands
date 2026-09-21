//! Regression coverage for the tests module.

use idle_hands::testing::modules::pyramid_ui::*;

#[test]
fn portrait_draw_rule_stays_inside_the_logical_width() {
    idle_hands::testing::ui::with_portrait_layout(|| {
        assert!(layout().draw_rule.right() <= idle_hands::testing::responsive_ui::WIDTH);
    });
}

#[test]
fn portrait_draw_rule_clears_the_shared_rule_card_and_tableau() {
    idle_hands::testing::ui::with_portrait_layout(|| {
        let layout = layout();
        assert!(!layout
            .draw_rule
            .overlaps(&idle_hands::testing::game_variant_ui::button_rect()));
        assert!(layout.draw_rule.bottom() <= layout.card_rect(0).y);
    });
}
