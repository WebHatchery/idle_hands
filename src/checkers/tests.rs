use super::*;

#[test]
fn seeded_boards_are_repeatable_and_have_twelve_pieces_each() {
    let first = Checkers::new(7);
    let second = Checkers::new(7);
    assert_eq!(first.cells, second.cells);
    assert_eq!(
        first
            .cells
            .iter()
            .filter(|piece| piece.side() == Some(Side::Red))
            .count(),
        12
    );
    assert_eq!(
        first
            .cells
            .iter()
            .filter(|piece| piece.side() == Some(Side::Yellow))
            .count(),
        12
    );
}

#[test]
fn a_red_piece_can_step_and_promote() {
    let mut game = Checkers::new(1);
    game.cells = vec![Piece::Empty; CELLS];
    game.cells[9] = Piece::RedMan;
    assert!(game.tap(9));
    assert!(game.tap(2));
    assert_eq!(game.cells[2], Piece::RedKing);
}

#[test]
fn captures_are_mandatory_and_remove_the_middle_piece() {
    let mut game = Checkers::new(1);
    game.cells = vec![Piece::Empty; CELLS];
    game.cells[5 * SIZE] = Piece::RedMan;
    game.cells[4 * SIZE + 1] = Piece::YellowMan;
    assert!(game.capture_available_for(Side::Red));
    assert!(game.tap(5 * SIZE));
    assert!(game.tap(3 * SIZE + 2));
    assert_eq!(game.cells[4 * SIZE + 1], Piece::Empty);
}

#[test]
fn bounded_yellow_reply_and_undo_restore_the_red_turn() {
    let mut game = Checkers::new(42);
    assert!(game.tap(5 * SIZE));
    assert!(game.tap(4 * SIZE + 1));
    assert_eq!(game.turn, Side::Red);
    assert!(game.undo());
    assert_eq!(game.turn, Side::Red);
    assert_eq!(game.moves, 0);
}

#[test]
fn hint_move_finds_a_mandatory_capture_without_mutating() {
    let mut game = Checkers::new(1);
    game.cells = vec![Piece::Empty; CELLS];
    game.cells[5 * SIZE] = Piece::RedMan;
    game.cells[4 * SIZE + 1] = Piece::YellowMan;
    let before = game.cells.clone();
    assert_eq!(game.hint_move(), Some((5 * SIZE, 3 * SIZE + 2)));
    assert_eq!(game.cells, before);
}

#[test]
fn ai_strengths_keep_seeded_board_state_stable() {
    let mut game = Checkers::new(7);
    let before = game.cells.clone();
    game.set_ai_level(AiLevel::Expert);
    game.reset(7);
    assert_eq!(game.ai_level, AiLevel::Expert);
    assert_eq!(game.cells, before);
}
