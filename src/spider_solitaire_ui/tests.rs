use super::*;

#[test]
fn compact_spider_solitaire_subtitle_stays_before_the_rule_card() {
    let subtitle_x = std::hint::black_box(COMPACT_SUBTITLE_X);
    assert!(subtitle_x + 190. < 494.);
}
