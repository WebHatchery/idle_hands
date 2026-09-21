//! Regression coverage for the tests module.

use idle_hands::testing::modules::tic_tac_toe::*;

#[test]
fn a_player_move_is_followed_by_a_bounded_ai_move() {
    let mut game = TicTacToe::new(7);
    assert!(game.place(0));
    assert_eq!(game.cells[0], Mark::X);
    assert_eq!(
        game.cells
            .iter()
            .filter(|mark| **mark != Mark::Empty)
            .count(),
        2
    );
}

#[test]
fn ai_blocks_an_immediate_row_win() {
    let mut game = TicTacToe::new(7);
    game.cells = [
        Mark::X,
        Mark::X,
        Mark::Empty,
        Mark::O,
        Mark::Empty,
        Mark::Empty,
        Mark::Empty,
        Mark::Empty,
        Mark::Empty,
    ];
    game.moves = 3;
    assert!(game.place(4));
    assert_eq!(game.cells[2], Mark::O);
}

#[test]
fn a_completed_game_rejects_more_moves_and_undo_restores_the_turn() {
    let mut game = TicTacToe::new(7);
    game.cells = [
        Mark::X,
        Mark::X,
        Mark::Empty,
        Mark::O,
        Mark::O,
        Mark::Empty,
        Mark::Empty,
        Mark::Empty,
        Mark::Empty,
    ];
    game.moves = 4;
    assert!(game.place(2));
    assert_eq!(game.status, TicTacToeStatus::Won(Mark::X));
    assert!(!game.place(5));
    assert!(game.undo());
    assert_eq!(game.status, TicTacToeStatus::Playing);
    assert_eq!(game.cells[2], Mark::Empty);
}

#[test]
fn ai_strengths_are_selectable_without_changing_seeded_openings() {
    let mut game = TicTacToe::new(7);
    for level in [AiLevel::Gentle, AiLevel::Sharp, AiLevel::Expert] {
        game.set_ai_level(level);
        assert_eq!(game.ai_level, level);
        game.reset(7);
        assert_eq!(game.ai_level, level);
    }
}
