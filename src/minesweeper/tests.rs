use super::*;

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
fn flood_reveal_opens_empty_region() {
    let mut game = Minesweeper {
        preset: MinePreset::Beginner,
        width: 3,
        height: 3,
        mines: 0,
        cells: vec![Cell::Hidden; 9],
        seed: 1,
        first_reveal: true,
        status: MineStatus::Playing,
        elapsed_seconds: 0.0,
    };
    assert!(game.reveal(0));
    assert_eq!(
        game.cells
            .iter()
            .filter(|cell| matches!(cell, Cell::Revealed(_)))
            .count(),
        9
    );
}

#[test]
fn presets_have_expected_dimensions_and_timer_only_runs_in_play() {
    let beginner = Minesweeper::new(MinePreset::Beginner, 1);
    let intermediate = Minesweeper::new(MinePreset::Intermediate, 1);
    let expert = Minesweeper::new(MinePreset::Expert, 1);
    assert_eq!(
        (beginner.width, beginner.height, beginner.mines),
        (9, 9, 10)
    );
    assert_eq!(
        (intermediate.width, intermediate.height, intermediate.mines),
        (16, 16, 40)
    );
    assert_eq!((expert.width, expert.height, expert.mines), (30, 16, 99));
    let mut game = Minesweeper::beginner(1);
    game.tick(4.0);
    assert_eq!(game.elapsed_whole_seconds(), 0);
    game.reveal(40);
    game.tick(2.2);
    assert_eq!(game.elapsed_whole_seconds(), 2);
}
