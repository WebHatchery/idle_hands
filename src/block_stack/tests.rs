use super::*;

#[test]
fn starts_with_a_piece_and_next_piece() {
    let game = BlockStack::new(7);
    assert_eq!(game.board.len(), usize::from(WIDTH) * usize::from(HEIGHT));
    assert!(game.piece < 7);
    assert!(game.next_piece < 7);
}

#[test]
fn touch_drop_changes_the_stack_and_state_round_trips() {
    let mut game = BlockStack::new(8);
    assert!(game.apply_move(BlockMove::Drop));
    assert!(game.moves > 0);
    assert!(game.board.iter().any(|cell| *cell != 0));
    let restored: BlockStack = serde_json::from_value(serde_json::to_value(game).unwrap()).unwrap();
    assert_eq!(restored.board.len(), 200);
    assert_eq!(restored.status, BlockStatus::Playing);
}
