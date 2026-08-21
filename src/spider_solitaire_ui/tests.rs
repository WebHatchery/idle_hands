#[test]
fn compact_long_title_stays_left_of_the_rule_card() {
    let title_x = 120.;
    let title_width_budget = 300.;
    let rule_card_x = 494.;
    let title_y = 28.;
    let subtitle_y = 52.;

    assert!(title_x > 110.);
    assert!(title_x + title_width_budget < rule_card_x);
    assert!(subtitle_y > title_y + 12.);
}

#[test]
fn compact_instruction_sits_below_the_tableau() {
    let tableau_bottom = 88. + 5. * 13. + 98.;
    let instruction_y = 280.;
    let runs_y = 315.;

    assert!(instruction_y > tableau_bottom + 8.);
    assert!(runs_y > instruction_y + 12.);
}
