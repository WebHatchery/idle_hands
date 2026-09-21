//! Regression coverage for the tests module.

use idle_hands::testing::modules::responsive_landscape_cards::*;

#[test]
fn compact_card_instruction_stays_before_the_rule_card() {
    let rule_card_x = 494.;
    assert!(COMPACT_CARD_INSTRUCTION_X + 210. < rule_card_x);
}
