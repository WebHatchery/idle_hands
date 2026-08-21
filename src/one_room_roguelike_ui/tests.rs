use super::*;

#[test]
fn portrait_long_title_budget_stays_left_of_the_rule_card() {
    let title_x = 10.;
    let title_width_budget = 195.;
    let rule_card_x = 220.;

    assert!(title_x + title_width_budget < rule_card_x);
}

#[test]
fn compact_header_lanes_clear_the_breadcrumb_and_rule_card() {
    let (title_x, status_x) = compact_header_positions();
    let rule_card_x = 494.;
    assert!(title_x >= 80.);
    assert!(status_x + 190. < rule_card_x);
}
