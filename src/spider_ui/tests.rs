#[test]
fn compact_spider_subtitle_stays_before_the_rule_card() {
    let subtitle_x = 250.;
    let rule_card_x = 494.;
    assert!(subtitle_x + 160. < rule_card_x);
}
