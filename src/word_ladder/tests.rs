use super::*;

#[test]
fn accepts_one_letter_ladder_steps_and_wins() {
    let mut game = WordLadder::new(0);
    for word in ["PLATE", "PLACE", "PLANE", "PLANK", "BLANK"] {
        for letter in word.bytes() {
            game.tap_letter(letter - b'A');
        }
        assert!(game.submit());
    }
    assert_eq!(game.phase, WordLadderPhase::Won);
    assert_eq!(game.moves, 5);
}

#[test]
fn rejects_non_dictionary_and_invalid_steps() {
    let mut game = WordLadder::new(0);
    for letter in b"ZZZZZ" {
        game.tap_letter(letter - b'A');
    }
    assert!(!game.submit());
    assert!(!game.current.is_empty());
    game.current.clear();
    for letter in b"FLANK" {
        game.tap_letter(letter - b'A');
    }
    assert!(!game.submit());
}

#[test]
fn undo_restores_previous_word() {
    let mut game = WordLadder::new(0);
    for letter in b"PLATE" {
        game.tap_letter(letter - b'A');
    }
    assert!(game.submit());
    assert!(game.undo());
    assert_eq!(game.moves, 0);
    assert!(game.guesses.is_empty());
}
