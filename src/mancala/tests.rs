//! Regression coverage for the tests module.

use super::*;

#[test]
fn sowing_a_pit_changes_the_board_and_opponent_turn() {
    let mut game = Mancala::new(1);
    assert!(game.play(0));
    assert!(game.pits[0] < 4);
    assert_eq!(game.moves, 1);
    assert!(game.pits.iter().any(|&stones| stones != 4));
}

#[test]
fn rejects_empty_and_out_of_range_pits() {
    let mut game = Mancala::new(2);
    assert!(!game.play(6));
    game.pits[0] = 0;
    assert!(!game.play(0));
}

#[test]
fn undo_restores_the_full_board() {
    let mut game = Mancala::new(3);
    let before = game.clone_without_undo();
    assert!(game.play(1));
    assert!(game.undo());
    assert_eq!(game.pits, before.pits);
    assert_eq!(game.moves, before.moves);
    assert_eq!(game.seed, before.seed);
}

#[test]
fn an_empty_side_finishes_and_player_can_win() {
    let mut game = Mancala::new(4);
    game.pits = vec![0; 14];
    game.pits[5] = 1;
    game.pits[PLAYER_STORE] = 10;
    game.pits[OPPONENT_START] = 1;
    assert!(game.play(5));
    assert!(game.won());
    assert_eq!(game.pits[PLAYER_STORE], 11);
}

#[test]
fn reset_starts_a_new_seeded_board() {
    let mut game = Mancala::new(5);
    game.play(0);
    game.reset(6);
    assert_eq!(game.seed, 6);
    assert_eq!(game.moves, 0);
    assert_eq!(game.pits[0], 4);
    assert_eq!(game.phase, MancalaPhase::Playing);
}

#[test]
fn hint_prefers_a_pit_that_grants_an_extra_turn_without_mutating_the_board() {
    let mut game = Mancala::new(7);
    game.pits = vec![0; 14];
    game.pits[5] = 1;
    game.pits[0] = 1;
    let before = game.clone_without_undo();

    assert_eq!(game.hint_pit(), Some(5));
    assert_eq!(game.pits, before.pits);
    assert_eq!(game.moves, before.moves);
    assert_eq!(game.phase, before.phase);
}

#[test]
fn hint_is_empty_after_mancala_ends() {
    let mut game = Mancala::new(8);
    game.phase = MancalaPhase::Won;

    assert_eq!(game.hint_pit(), None);
}

#[test]
fn ai_strengths_change_only_the_opponent_policy() {
    let mut game = Mancala::new(8);
    game.set_ai_level(AiLevel::Expert);
    assert_eq!(game.ai_level, AiLevel::Expert);
    assert_eq!(game.pits, Mancala::new(8).pits);
}

#[test]
fn variants_change_the_opening_length_and_survive_reset() {
    for (variant, stones) in [
        (MancalaVariant::Quick, 3),
        (MancalaVariant::Classic, 4),
        (MancalaVariant::Grand, 5),
    ] {
        let mut game = Mancala::new_with_variant(9, variant);
        assert_eq!(game.pits[0], stones);
        assert_eq!(game.pits[12], stones);
        game.reset(10);
        assert_eq!(game.variant, variant);
        assert_eq!(game.pits[0], stones);
    }
}

#[test]
fn move_preview_and_totals_surface_captures() {
    let mut game = Mancala::new(11);
    game.pits = vec![0; 14];
    game.pits[0] = 1;
    game.pits[2] = 1;
    game.pits[7] = 1;
    game.pits[11] = 4;

    assert_eq!(
        game.move_preview(0),
        Some(MovePreview {
            store_gain: 5,
            captured: 5,
            extra_turn: false,
        })
    );
    assert!(game.play(0));
    assert_eq!(game.captured_stones, 5);
}

#[test]
fn landing_in_the_store_tracks_a_bonus_turn() {
    let mut game = Mancala::new(12);
    game.pits = vec![0; 14];
    game.pits[0] = 1;
    game.pits[5] = 1;
    game.pits[7] = 1;

    assert!(game.move_preview(5).unwrap().extra_turn);
    assert!(game.play(5));
    assert_eq!(game.extra_turns, 1);
    assert_eq!(game.pits[PLAYER_STORE], 1);
}

#[test]
fn sharp_opponent_takes_a_bonus_turn_that_gentle_skips() {
    let mut gentle = Mancala::new(13);
    gentle.pits = vec![0; 14];
    gentle.pits[0] = 1;
    gentle.pits[7] = 1;
    gentle.pits[8] = 5;
    gentle.ai_level = AiLevel::Gentle;
    let mut sharp = gentle.clone_without_undo();
    sharp.ai_level = AiLevel::Sharp;

    gentle.cpu_turn();
    sharp.cpu_turn();

    assert_eq!(gentle.pits[OPPONENT_STORE], 0);
    assert!(sharp.pits[OPPONENT_STORE] > 0);
}

#[test]
fn legacy_saves_default_to_classic_without_tactical_totals() {
    let mut value = serde_json::to_value(Mancala::new(14)).unwrap();
    let object = value.as_object_mut().unwrap();
    object.remove("variant");
    object.remove("captured_stones");
    object.remove("extra_turns");
    let loaded: Mancala = serde_json::from_value(value).unwrap();

    assert_eq!(loaded.variant, MancalaVariant::Classic);
    assert_eq!(loaded.captured_stones, 0);
    assert_eq!(loaded.extra_turns, 0);
}
