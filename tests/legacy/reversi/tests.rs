//! The bounded Reversi AI returns a deterministic legal move.

use idle_hands::testing::modules::reversi::*;

#[test]
fn ai_levels_choose_a_legal_move_for_a_fixed_position() {
    for level in [AiLevel::Gentle, AiLevel::Sharp] {
        let mut game = Reversi::new(5, level);
        game.place(2 * 8 + 3);
        assert!(game.ai_move());
        assert!(game
            .legal_moves(1)
            .iter()
            .all(|index| game.board[*index] == 0));
    }
}
