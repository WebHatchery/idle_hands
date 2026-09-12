//! Regression coverage for the tests module.

use super::*;

#[test]
fn seeded_words_repeat_and_guessing_tracks_progress() {
    let mut first = Hangman::new(42);
    let second = Hangman::new(42);
    assert_eq!(first.word, second.word);
    assert!(first.guess(first.word.as_bytes()[0] - b'A'));
    assert_eq!(first.moves, 1);
    assert!(!first.guess(first.word.as_bytes()[0] - b'A'));
}

#[test]
fn all_word_letters_win_the_round() {
    let mut game = Hangman::new(0);
    for letter in b"STARE" {
        assert!(game.guess(letter - b'A'));
    }
    assert_eq!(game.status, HangmanStatus::Won);
    assert!(!game.guess(0));
}

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
fn hint_letter_is_unplayed_and_does_not_mutate_the_round() {
    let game = Hangman::new(42);
    let before = game.guessed;
    let letter = game.hint_letter().unwrap();
    assert!(!game.guessed[letter as usize]);
    assert_eq!(game.guessed, before);
}

#[test]
fn categories_select_distinct_word_pools_and_survive_reset() {
    let mut game = Hangman::new_with_options(3, HangmanCategory::Nature, HangmanRule::Classic);
    assert!(game.word_list.contains(&game.word));
    game.reset(4);
    assert_eq!(game.category, HangmanCategory::Nature);
    assert!(game.word_list.contains(&game.word));
    game.set_category(HangmanCategory::Voyage, 5);
    assert!(game.word_list.contains(&game.word));
}

#[test]
fn rapid_rule_shortens_the_clock_and_doubles_letter_points() {
    let mut game = Hangman::new_with_options(0, HangmanCategory::Cabinet, HangmanRule::Rapid);
    assert!(game.guess(b'S' - b'A'));
    assert_eq!((game.score, game.combo), (20, 1));
    for letter in b"BCFG" {
        assert!(game.guess(letter - b'A'));
    }
    assert_eq!(game.status, HangmanStatus::Lost);
    assert_eq!(game.wrong_count, 4);
}

#[test]
fn correct_guesses_build_a_chain_and_wrong_guesses_break_it() {
    let mut game = Hangman::new(0);
    assert!(game.guess(b'S' - b'A'));
    assert!(game.guess(b'T' - b'A'));
    assert_eq!((game.score, game.combo, game.best_combo), (30, 2, 2));
    assert!(game.guess(b'B' - b'A'));
    assert_eq!((game.score, game.combo, game.best_combo), (30, 0, 2));
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
fn legacy_saves_receive_depth_defaults() {
    let original = Hangman::new(2);
    let mut value = serde_json::to_value(&original).unwrap();
    for field in [
        "category",
        "rule",
        "score",
        "combo",
        "best_combo",
        "reveals",
    ] {
        value.as_object_mut().unwrap().remove(field);
    }
    let restored: Hangman = serde_json::from_value(value).unwrap();
    assert_eq!(restored.category, HangmanCategory::Cabinet);
    assert_eq!(restored.rule, HangmanRule::Classic);
    assert_eq!(
        (restored.score, restored.combo, restored.best_combo),
        (0, 0, 0)
    );
    assert_eq!(restored.reveals, 1);
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
