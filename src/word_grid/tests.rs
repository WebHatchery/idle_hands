//! Regression coverage for the tests module.

use super::*;

fn enter(game: &mut WordGrid, word: &str) {
    for letter in word.bytes() {
        assert!(game.tap_letter(letter - b'A'));
    }
    assert!(game.submit());
}

#[test]
fn seeded_target_is_repeatable_and_input_is_touch_complete() {
    let mut game = WordGrid::new(0);
    assert_eq!(game.target, WordGrid::new(0).target);
    assert!(game.tap_letter(16));
    assert!(game.backspace());
    assert!(!game.backspace());
    assert!(!game.submit());
}

#[test]
fn duplicate_aware_feedback_marks_exact_present_and_absent_letters() {
    let mut game = WordGrid::new(0);
    enter(&mut game, "LLAMA");
    assert_eq!(game.feedback[0][0], LetterState::Present);
    assert_eq!(game.feedback[0][1], LetterState::Present);
    assert_eq!(game.feedback[0][2], LetterState::Absent);
    assert_eq!(game.feedback[0][3], LetterState::Absent);
    assert_eq!(game.feedback[0][4], LetterState::Absent);
}

#[test]
fn correct_guess_wins_and_finished_games_reject_input() {
    let mut game = WordGrid::new(0);
    enter(&mut game, "STILL");
    assert!(game.won());
    assert!(!game.tap_letter(0));
    assert!(!game.submit());
}

#[test]
fn six_incorrect_guesses_lose_and_undo_restores_the_last_row() {
    let mut game = WordGrid::new(0);
    for _ in 0..6 {
        enter(&mut game, "SHELF");
    }
    assert_eq!(game.phase, WordGridPhase::Lost);
    assert!(game.undo());
    assert_eq!(game.phase, WordGridPhase::Playing);
    assert_eq!(game.guesses.len(), 5);
}

#[test]
fn reset_changes_the_seeded_word_and_clears_rows() {
    let mut game = WordGrid::new(0);
    enter(&mut game, "SHELF");
    game.reset(1);
    assert_ne!(game.target, "STILL");
    assert!(game.guesses.is_empty());
    assert_eq!(game.moves, 0);
}

#[test]
fn hint_returns_a_consistent_probe_without_revealing_the_target() {
    let game = WordGrid::new(0);

    assert!(game.hint_word().is_some());
    assert_ne!(game.hint_word().as_deref(), Some(game.target.as_str()));
    assert!(game.guesses.is_empty());
    assert!(game.current.is_empty());
}

#[test]
fn hint_is_empty_after_word_grid_ends() {
    let mut game = WordGrid::new(1);
    game.phase = WordGridPhase::Won;

    assert_eq!(game.hint_word(), None);
}

#[test]
fn feedback_reduces_the_visible_candidate_pool() {
    let mut game = WordGrid::new(0);
    let before = game.remaining_words().len();
    enter(&mut game, "SHELF");
    assert!(game.remaining_words().len() < before);
    assert!(game.remaining_words().contains(&"STILL"));
}

#[test]
fn hard_mode_requires_green_positions_and_present_letters() {
    let mut game = WordGrid::new_with_mode(0, WordGridMode::Hard);
    enter(&mut game, "SHELF");
    for letter in "GAMES".bytes() {
        assert!(game.tap_letter(letter - b'A'));
    }
    assert!(!game.submit());
    assert!(game.notice.starts_with("HARD:"));
    assert_eq!(game.guesses.len(), 1);
}

#[test]
fn mode_changes_start_a_fresh_word_and_survive_reset() {
    let mut game = WordGrid::new(0);
    enter(&mut game, "SHELF");
    game.set_mode(WordGridMode::Hard, 1);
    assert_eq!(game.mode, WordGridMode::Hard);
    assert!(game.guesses.is_empty());
    game.reset(2);
    assert_eq!(game.mode, WordGridMode::Hard);
}

#[test]
fn repeated_undo_rewinds_multiple_submitted_rows() {
    let mut game = WordGrid::new(0);
    enter(&mut game, "SHELF");
    enter(&mut game, "PAUSE");
    assert!(game.undo());
    assert_eq!(game.guesses.len(), 1);
    assert!(game.undo());
    assert!(game.guesses.is_empty());
    assert!(!game.undo());
}

#[test]
fn legacy_saves_default_to_classic_with_no_notice() {
    let original = WordGrid::new(3);
    let mut value = serde_json::to_value(&original).unwrap();
    let object = value.as_object_mut().unwrap();
    object.remove("mode");
    object.remove("notice");
    let restored: WordGrid = serde_json::from_value(value).unwrap();
    assert_eq!(restored.mode, WordGridMode::Classic);
    assert!(restored.notice.is_empty());
}
