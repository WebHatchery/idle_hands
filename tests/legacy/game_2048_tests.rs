//! Regression tests for the 2048 rule engine.

use super::*;

#[test]
fn hint_returns_the_first_legal_direction() {
    let mut game = Game2048::new(1);
    game.cells = vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(game.hint_direction(), Some(Direction::Down));
}

#[test]
fn hint_returns_none_for_a_blocked_board() {
    let mut game = Game2048::new(1);
    game.cells = vec![2, 4, 2, 4, 4, 2, 4, 2, 2, 4, 2, 4, 4, 2, 4, 2];
    assert_eq!(game.hint_direction(), None);
}

#[test]
fn five_by_five_board_spawns_two_tiles_and_merges_dynamically() {
    let mut game = Game2048::new_with_size(42, Game2048Size::Five);

    assert_eq!(game.board_size, Game2048Size::Five);
    assert_eq!(game.cells.len(), 25);
    assert_eq!(game.cells.iter().filter(|&&value| value != 0).count(), 2);

    game.cells = vec![
        2, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    game.move_in(Direction::Left);

    assert_eq!(&game.cells[..5], &[4, 4, 0, 0, 0]);
}

#[test]
fn legacy_four_by_four_save_defaults_board_size() {
    let game = Game2048::new(7);
    let mut value = serde_json::to_value(&game).unwrap();
    value.as_object_mut().unwrap().remove("board_size");

    let restored: Game2048 = serde_json::from_value(value).unwrap();

    assert_eq!(restored.board_size, Game2048Size::Four);
    assert_eq!(restored.cells.len(), 16);
}
