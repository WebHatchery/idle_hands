use super::*;

#[test]
fn portrait_draw_rule_stays_inside_the_logical_width() {
    crate::ui::with_portrait_layout(|| {
        assert!(layout().draw_rule.right() <= crate::responsive_ui::WIDTH);
    });
}

#[test]
fn portrait_draw_rule_clears_the_shared_rule_card_and_tableau() {
    crate::ui::with_portrait_layout(|| {
        let layout = layout();
        assert!(!layout
            .draw_rule
            .overlaps(&crate::game_variant_ui::button_rect()));
        assert!(layout.draw_rule.bottom() <= layout.card_rect(0).y);
    });
}
