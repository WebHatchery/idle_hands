//! Regression coverage for the tests module.

use idle_hands::testing::modules::word_search_ui::*;

#[test]
fn compact_word_search_instruction_stays_before_the_rule_card() {
    let instruction = compact_instruction_position();

    assert!(instruction.x >= 350.);
    assert_eq!(instruction.y, 30.);
    assert!(instruction.x + 120. < 494.);
}
