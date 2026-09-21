//! Regression coverage for the tests module.

use idle_hands::testing::modules::word_grid::*;

fn enter(game: &mut WordGrid, word: &str) {
    for letter in word.bytes() {
        assert!(game.tap_letter(letter - b'A'));
    }
    assert!(game.submit());
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
