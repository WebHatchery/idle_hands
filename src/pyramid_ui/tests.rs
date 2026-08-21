use super::*;

#[test]
fn portrait_draw_rule_stays_inside_the_logical_width() {
    crate::ui::with_portrait_layout(|| {
        assert!(layout().draw_rule.right() <= crate::responsive_ui::WIDTH);
    });
}
