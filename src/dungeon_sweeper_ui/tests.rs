#[test]
fn compact_long_title_stays_left_of_the_rule_card() {
    let title_x = 150.;
    let title_width_budget = 330.;
    let rule_card_x = 494.;
    let title_y = 30.;
    let status_y = 52.;

    assert!(title_x > 110.);
    assert!(title_x + title_width_budget < rule_card_x);
    assert!(status_y > title_y + 12.);
}
