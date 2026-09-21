//! Regression coverage for the tests module.

use idle_hands::testing::modules::game::game_capture_records::*;

#[test]
fn card_capture_selects_cards_and_seeds_mixed_records() {
    let mut state = AppState::default();
    apply(&mut state, "records_cards");

    assert_eq!(state.records_filter, 3);
    assert_eq!(state.records.solitaire_best_moves, Some(42));
    assert_eq!(state.records.freecell_best_moves, Some(31));
}
