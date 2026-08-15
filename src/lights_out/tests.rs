use super::*;

#[test]
fn seeded_boards_repeat_and_start_lit() {
    let first = LightsOut::new(7);
    assert_eq!(first.cells, LightsOut::new(7).cells);
    assert!(first.cells.iter().any(|cell| *cell));
    assert_eq!(first.status, LightsOutStatus::Playing);
}

#[test]
fn pressing_a_cell_toggles_its_cross_and_undo_restores_it() {
    let mut game = LightsOut {
        cells: [false; CELLS],
        moves: 0,
        seed: 7,
        status: LightsOutStatus::Playing,
        undo: None,
    };
    assert!(game.press(0));
    assert_eq!(game.cells[0..3], [true, true, false]);
    assert!(game.cells[5]);
    assert_eq!(game.moves, 1);
    assert!(game.undo());
    assert_eq!(game.cells, [false; CELLS]);
    assert_eq!(game.moves, 0);
}

#[test]
fn solving_the_generated_board_is_possible_by_repeating_the_seed_moves() {
    let mut game = LightsOut::new(7);
    let original = game.cells;
    let mut source = 7;
    for _ in 0..12 {
        source = next_seed(source);
        game.press((source as usize) % CELLS);
    }
    assert_eq!(game.cells, [false; CELLS]);
    assert_eq!(game.status, LightsOutStatus::Won);
    assert_ne!(original, game.cells);
}
