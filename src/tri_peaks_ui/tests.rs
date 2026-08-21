use super::*;

#[test]
fn compact_tri_peaks_status_is_short_enough_for_the_header_lane() {
    assert!(compact_status_text(TriPeaksStatus::Playing).len() <= 12);
}

#[test]
fn portrait_variant_controls_clear_the_shared_rule_card_and_tableau() {
    crate::ui::with_portrait_layout(|| {
        let layout = layout();
        let shared_rule_card = crate::game_variant_ui::button_rect();
        assert!(!layout.bridge.overlaps(&shared_rule_card));
        assert!(!layout.rule.overlaps(&shared_rule_card));
        assert!(!layout.bridge.overlaps(&layout.rule));
        assert!(layout.bridge.bottom() <= layout.card_rect(0).y);
        assert!(layout.rule.bottom() <= layout.card_rect(0).y);
    });
}
