use super::*;

#[test]
fn logic_capture_opens_the_logic_rules_shelf() {
    let mut state = AppState::default();
    apply(&mut state, "rules_logic");

    assert_eq!(state.rules_filter, 4);
    assert_eq!(state.library_scroll, 0);
}

#[test]
fn word_capture_opens_the_word_rules_shelf() {
    let mut state = AppState::default();
    apply(&mut state, "rules_word");

    assert_eq!(state.rules_filter, 6);
}

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
