use super::*;

#[test]
fn new_board_has_two_tiles_and_is_seeded() {
    let first = Game2048::new(42);
    let second = Game2048::new(42);
    assert_eq!(first.cells, second.cells);
    assert_eq!(first.cells.iter().filter(|&&value| value != 0).count(), 2);
}

#[test]
fn horizontal_move_merges_once_and_can_undo() {
    let mut game = Game2048::new(1);
    game.cells = [2, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert!(game.move_in(Direction::Left));
    assert_eq!(&game.cells[0..3], &[4, 4, 0]);
    assert!(game.undo());
    assert_eq!(game.cells[0], 2);
}

#[test]
fn blocked_board_has_no_available_move() {
    let mut game = Game2048::new(1);
    game.cells = [2, 4, 2, 4, 4, 2, 4, 2, 2, 4, 2, 4, 4, 2, 4, 2];
    assert!(!game.can_move());
}

#[test]
fn collection_save_round_trips_game_and_profile_state() {
    let mut state = AppState::default();
    state.profile_name = "Quiet Player".into();
    state.game.score = 128;
    state.mine_records[0] = Some(42);
    state.records.best_2048 = 128;
    state.records.fivefold_best_total = 275;
    state.tutorial_seen[6] = true;
    let save = CollectionSave::from_state(&state, "1.0.0");
    let mut restored = AppState::default();
    save.apply_to(&mut restored);
    assert_eq!(restored.profile_name, "Quiet Player");
    assert_eq!(restored.game.score, 128);
    assert_eq!(restored.mine_records[0], Some(42));
    assert_eq!(restored.records.best_2048, 128);
    assert_eq!(restored.records.fivefold_best_total, 275);
    assert!(restored.tutorial_seen[6]);
}
