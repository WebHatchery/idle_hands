use super::*;

#[test]
fn compact_metrics_use_a_short_header_lane_copy() {
    let game = MiscGame::new(0x4D49_5343_0001, MiscKind::OrbitOrder);

    assert_eq!(metrics_text(&game, true), "R0 • S0 • M0");
    assert!(metrics_text(&game, true).chars().count() < metrics_text(&game, false).chars().count());
}

#[test]
fn portrait_misc_titles_and_metrics_use_the_header_lane() {
    let game = MiscGame::new(0x4D49_5343_0001, MiscKind::OrbitOrder);

    crate::ui::with_portrait_layout(|| {
        assert_eq!(title_size(), 21.);
        assert_eq!(metrics_text(&game, true), "R0 • S0 • M0");
    });
}
