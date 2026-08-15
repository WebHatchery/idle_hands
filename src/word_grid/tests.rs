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
    enter(&mut game, "EERIE");
    assert_eq!(game.feedback[0][0], LetterState::Present);
    assert_eq!(game.feedback[0][1], LetterState::Absent);
    assert_eq!(game.feedback[0][2], LetterState::Absent);
    assert_eq!(game.feedback[0][3], LetterState::Present);
    assert_eq!(game.feedback[0][4], LetterState::Absent);
}

#[test]
fn correct_guess_wins_and_finished_games_reject_input() {
    let mut game = WordGrid::new(0);
    enter(&mut game, "QUIET");
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
    assert_ne!(game.target, "QUIET");
    assert!(game.guesses.is_empty());
    assert_eq!(game.moves, 0);
}
