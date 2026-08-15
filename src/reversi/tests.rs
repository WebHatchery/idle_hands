use super::*;

#[test]
fn opening_position_has_four_moves_and_flips_a_line() {
    let mut game = Reversi::new(1, AiLevel::Gentle);
    assert_eq!(game.legal_moves(1).len(), 4);
    assert!(game.place(2 * 8 + 3));
    assert_eq!(game.board[3 * 8 + 3], 1);
    assert_eq!(game.score(1), 4);
    assert_eq!(game.turn, 2);
}

#[test]
fn ai_levels_are_deterministic_and_choose_a_legal_move() {
    let mut gentle = Reversi::new(5, AiLevel::Gentle);
    let mut sharp = Reversi::new(5, AiLevel::Sharp);
    gentle.place(2 * 8 + 3);
    sharp.place(2 * 8 + 3);
    assert!(gentle.ai_move());
    assert!(sharp.ai_move());
    assert_eq!(
        gentle.score(1) + gentle.score(2),
        sharp.score(1) + sharp.score(2)
    );
    assert!(gentle
        .legal_moves(1)
        .iter()
        .all(|index| gentle.board[*index] == 0));
}

#[test]
fn a_board_with_no_legal_moves_can_pass_and_end() {
    let mut game = Reversi::new(7, AiLevel::Gentle);
    game.board = vec![1; 64];
    game.board[0] = 0;
    game.turn = 1;
    assert!(game.pass());
    assert_eq!(game.status, ReversiStatus::Won);
    assert_eq!(game.winner, Some(1));
}
