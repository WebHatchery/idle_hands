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
