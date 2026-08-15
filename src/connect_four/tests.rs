use super::*;

#[test]
fn seeded_empty_boards_repeat() {
    let first = ConnectFour::new(42);
    let second = ConnectFour::new(42);
    assert_eq!(first.cells, second.cells);
    assert_eq!(first.status, ConnectFourStatus::Playing);
}

#[test]
fn gravity_and_four_in_a_row_are_detected() {
    let mut cells = vec![Disc::Empty; CELLS];
    for column in 0..4 {
        cells[(ROWS - 1) * COLUMNS + column] = Disc::Red;
    }
    assert!(has_four(&cells, Disc::Red));
    let mut game = ConnectFour::new(1);
    game.cells = cells;
    assert_eq!(game.cells[(ROWS - 1) * COLUMNS], Disc::Red);
    assert_eq!(game.cells[(ROWS - 2) * COLUMNS], Disc::Empty);
}

#[test]
fn a_player_drop_can_be_undone_after_the_bounded_ai_reply() {
    let mut game = ConnectFour::new(42);
    assert!(game.drop(3));
    assert!(game.cells.contains(&Disc::Red));
    assert!(game.cells.contains(&Disc::Yellow));
    assert!(game.undo());
    assert!(game.cells.iter().all(|cell| *cell == Disc::Empty));
    assert_eq!(game.moves, 0);
}
