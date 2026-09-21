//! Regression coverage for the tests module.

use idle_hands::testing::modules::minesweeper::*;

#[test]
fn first_reveal_is_safe_and_seeded() {
    let mut left = Minesweeper::beginner(7);
    let mut right = Minesweeper::beginner(7);
    assert!(left.reveal(40));
    assert!(right.reveal(40));
    assert_eq!(left.cells, right.cells);
    assert!(!matches!(left.cells[40], Cell::Revealed(9)));
}

#[test]
fn marking_and_chording_reveal_a_safe_neighbor() {
    let mut game = Minesweeper {
        preset: MinePreset::Beginner,
        width: 3,
        height: 3,
        mines: 1,
        cells: vec![Cell::Hidden; 9],
        seed: 1,
        first_reveal: true,
        status: MineStatus::Playing,
        elapsed_seconds: 0.0,
    };
    game.cells[0] = Cell::Revealed(1);
    game.cells[1] = Cell::Mine;
    assert!(game.toggle_flag(1));
    assert!(game.chord(0));
    assert!(matches!(game.cells[3], Cell::Revealed(_)));
}

#[test]
fn custom_board_clamps_to_touchable_safe_bounds() {
    let game = Minesweeper::custom(2, 99, 999, 3);
    assert_eq!((game.width, game.height), (5, 24));
    assert_eq!(game.mines, 111);
    assert_eq!(game.cells.len(), 120);
}

#[test]
fn revealed_number_can_prove_a_neighbor_is_a_mine() {
    let mut game = Minesweeper {
        preset: MinePreset::Beginner,
        width: 3,
        height: 3,
        mines: 1,
        cells: vec![Cell::Hidden; 9],
        seed: 1,
        first_reveal: true,
        status: MineStatus::Playing,
        elapsed_seconds: 0.0,
    };
    game.cells[0] = Cell::Revealed(3);
    game.cells[1] = Cell::Flagged;
    game.cells[3] = Cell::Flagged;
    let (index, safe) = game.hint_move().unwrap();
    assert_eq!(index, 4);
    assert!(!safe);
}
