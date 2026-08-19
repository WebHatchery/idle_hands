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
    for letter in [b'B', b'C', b'F', b'G', b'H', b'J'] {
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
