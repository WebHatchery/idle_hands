//! Regression coverage for the tests module.

use super::*;

#[test]
fn six_wrong_letters_lose_the_round() {
    let mut game = Hangman::new(0);
    for letter in b"BCFGHJ" {
        assert!(game.guess(letter - b'A'));
    }
    assert_eq!(game.wrong_count, 6);
    assert_eq!(game.status, HangmanStatus::Lost);
}

#[test]
fn reveal_spends_one_charge_without_scoring_and_undo_refunds_it() {
    let mut game = Hangman::new(0);
    assert!(game.reveal());
    assert!(game.is_revealed(b'S' - b'A'));
    assert_eq!((game.reveals, game.score, game.combo), (0, 0, 0));
    assert!(!game.reveal());
    assert!(game.undo());
    assert_eq!((game.reveals, game.moves), (1, 0));
    assert!(!game.is_revealed(b'S' - b'A'));
}

#[test]
fn undo_restores_failure_pressure_and_scoring_state() {
    let mut game = Hangman::new(0);
    game.guess(b'S' - b'A');
    game.guess(b'B' - b'A');
    assert_eq!((game.wrong_count, game.combo), (1, 0));
    assert!(game.undo());
    assert_eq!((game.wrong_count, game.combo, game.score), (0, 1, 10));
    assert!(!game.wrong[(b'B' - b'A') as usize]);
}

#[test]
fn candidate_count_respects_length_positions_and_excluded_letters() {
    let mut game = Hangman::new_with_options(0, HangmanCategory::Nature, HangmanRule::Classic);
    let initial = game.candidate_count();
    game.guess(b'G' - b'A');
    assert!(game.candidate_count() < initial);
    assert!(game.candidate_count() >= 1);
    game.guess(b'B' - b'A');
    assert!(game.candidate_count() >= 1);
}
