use super::*;

#[test]
fn card_capture_selects_cards_and_seeds_mixed_records() {
    let mut state = AppState::default();
    apply(&mut state, "records_cards");

    assert_eq!(state.records_filter, 3);
    assert_eq!(state.records.solitaire_best_moves, Some(42));
    assert_eq!(state.records.freecell_best_moves, Some(31));
}

#[test]
fn arcade_capture_selects_arcade_and_seeds_the_shelf() {
    let mut state = AppState::default();
    apply(&mut state, "records_arcade");

    assert_eq!(state.records_filter, 7);
    assert_eq!(state.records.snake_best_score, Some(120));
    assert_eq!(state.records.fling_fury_best_score, Some(73));
}

#[test]
fn unrelated_capture_scenes_leave_records_untouched() {
    let mut state = AppState::default();
    apply(&mut state, "cabinet_open");

    assert_eq!(state.records_filter, 0);
    assert_eq!(state.records.best_2048, 0);
    assert_eq!(state.records.solitaire_best_moves, None);
    assert_eq!(state.records.word_ladder_best_moves, None);
    assert_eq!(
        state.achievements,
        vec![false; crate::progression::AchievementId::ALL.len()]
    );
}
