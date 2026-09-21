//! Regression coverage for the tests module.

use idle_hands::testing::modules::mancala::*;

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
