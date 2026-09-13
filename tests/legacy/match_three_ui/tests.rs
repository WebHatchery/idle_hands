//! Regression coverage for the tests module.

use super::*;

#[test]
fn portrait_match_three_summary_stays_before_the_rule_card() {
    let summary = portrait_summary(0, 18, 100);

    assert_eq!(summary, "0 pts  •  18 moves  •  T100");
    assert!(summary.chars().count() < 30);
}
