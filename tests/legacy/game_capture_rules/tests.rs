//! Regression coverage for the tests module.

use super::*;

#[test]
fn unrelated_capture_scenes_do_not_change_the_rules_shelf() {
    let mut state = AppState {
        rules_filter: 5,
        library_scroll: 3,
        ..AppState::default()
    };
    apply(&mut state, "records_progress");

    assert_eq!(state.rules_filter, 5);
    assert_eq!(state.library_scroll, 3);
}

#[test]
fn word_capture_keeps_the_first_filtered_rule_launchable_by_canonical_index() {
    let mut state = AppState::default();
    apply(&mut state, "rules_word");
    let first = crate::rules_data::page_rows(state.rules_filter, state.library_scroll, 8)
        .into_iter()
        .next()
        .expect("word shelf has a first rule");

    assert_eq!(first.game, crate::state::GameId::Mastermind);
    assert_eq!(first.game.index(), 12);
}
