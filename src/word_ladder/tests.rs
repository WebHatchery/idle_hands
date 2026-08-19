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

fn enter(game: &mut WordLadder, word: &str) -> bool {
    game.current.clear();
    for letter in word.bytes() {
        assert!(game.tap_letter(letter - b'A'));
    }
    game.submit()
}

#[test]
fn hint_follows_the_shortest_route_to_the_target() {
    let mut game = WordLadder::new(0);
    assert_eq!(game.hint_word(), Some("PLATE"));
    while let Some(next) = game.hint_word() {
        assert!(enter(&mut game, next));
    }
    assert_eq!(game.phase, WordLadderPhase::Won);
    assert_eq!(game.moves, game.par);
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
fn route_metrics_publish_remaining_and_branch_count() {
    let game = WordLadder::new_with_mode(1, LadderMode::Scenic);
    assert!(game.par >= 4);
    assert!(game.remaining_steps() > 0);
    assert!(game.legal_step_count() > 0);
    assert_eq!(game.current_difference_count(), 0);
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
fn mode_switch_and_reset_keep_the_scenic_rule() {
    let mut game = WordLadder::new(0);
    game.set_mode(LadderMode::Scenic, 1);
    assert_eq!(game.mode, LadderMode::Scenic);
    game.reset(2);
    assert_eq!(game.mode, LadderMode::Scenic);
    assert!(!game.waypoint.is_empty());
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
