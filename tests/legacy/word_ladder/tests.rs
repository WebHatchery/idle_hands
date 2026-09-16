//! Regression coverage for the tests module.

use super::*;

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

fn enter(game: &mut WordLadder, word: &str) -> bool {
    game.current.clear();
    for letter in word.bytes() {
        assert!(game.tap_letter(letter - b'A'));
    }
    game.submit()
}

#[test]
fn scenic_ladder_requires_the_waypoint_before_the_target() {
    let mut game = WordLadder::new_with_mode(2, LadderMode::Scenic);
    assert!(!enter(&mut game, "NIGHT"));
    assert!(game.message.contains("RIGHT"));
    for word in ["MIGHT", "RIGHT", "NIGHT"] {
        assert!(enter(&mut game, word));
    }
    assert!(game.waypoint_reached);
    assert_eq!(game.phase, WordLadderPhase::Won);
    assert_eq!(game.moves, 3);
}

#[test]
fn repeated_undo_rewinds_multiple_ladder_steps() {
    let mut game = WordLadder::new(0);
    assert!(enter(&mut game, "PLATE"));
    assert!(enter(&mut game, "PLACE"));
    assert!(game.undo());
    assert_eq!(game.guesses, ["PLATE"]);
    assert!(game.undo());
    assert!(game.guesses.is_empty());
    assert!(!game.undo());
}

#[test]
fn legacy_saves_default_to_direct_without_a_waypoint_requirement() {
    let original = WordLadder::new(0);
    let mut value = serde_json::to_value(&original).unwrap();
    let object = value.as_object_mut().unwrap();
    for field in ["waypoint", "waypoint_reached", "mode", "par"] {
        object.remove(field);
    }
    let restored: WordLadder = serde_json::from_value(value).unwrap();
    assert_eq!(restored.mode, LadderMode::Direct);
    assert!(restored.waypoint.is_empty());
}
