use super::*;

#[test]
fn catalog_presets_have_expected_sizes_and_clues() {
    let small = Nonogram::new(NonogramPreset::Small);
    let medium = Nonogram::new(NonogramPreset::Medium);
    let large = Nonogram::new(NonogramPreset::Large);
    assert_eq!(small.size, 5);
    assert_eq!(medium.size, 10);
    assert_eq!(large.size, 15);
    assert_eq!(small.row_clues.len(), 5);
    assert!(small.row_clues.iter().any(|clue| clue != &[0]));
}

#[test]
fn nonogram_focus_window_keeps_large_boards_touchable() {
    assert_eq!(visible_size(15, false), 15);
    assert_eq!(visible_size(15, true), 9);
    assert_eq!(focus_origin(15, true, (99, 99)), (6, 6));
    assert_eq!(focus_origin(10, true, (99, 99)), (1, 1));
    assert_eq!(focus_origin(5, true, (99, 99)), (0, 0));
}

#[test]
fn fill_cross_and_undo_are_touch_safe() {
    let mut game = Nonogram::default();
    assert!(game.toggle(0));
    assert_eq!(game.marks[0], NonogramMark::Filled);
    game.toggle_mode();
    assert!(game.toggle(1));
    assert_eq!(game.marks[1], NonogramMark::Crossed);
    assert!(game.undo());
    assert_eq!(game.marks[1], NonogramMark::Empty);
}

#[test]
fn exact_solution_marks_win() {
    let mut game = Nonogram::default();
    for index in 0..game.solution.len() {
        if game.solution[index] {
            game.marks[index] = NonogramMark::Filled;
        }
    }
    game.check_win();
    assert_eq!(game.status, NonogramStatus::Won);
}

#[test]
fn strokes_lock_to_the_dominant_axis() {
    assert_eq!(stroke_indices(5, (0, 2), (4, 3)), vec![10, 11, 12, 13, 14]);
    assert_eq!(stroke_indices(5, (2, 0), (3, 4)), vec![2, 7, 12, 17, 22]);
}

#[test]
fn hint_cell_identifies_the_first_empty_solution_mark_without_mutating() {
    let game = Nonogram::default();
    let before = game.marks.clone();
    let (index, filled) = game.hint_cell().unwrap();
    assert_eq!(index, 0);
    assert!(filled);
    assert_eq!(game.marks, before);
}

#[test]
fn authored_variants_keep_their_clues_and_solutions_distinct() {
    let variants: Vec<_> = (0..VARIANT_COUNT)
        .map(|variant| Nonogram::new_with_variant(NonogramPreset::Medium, variant))
        .collect();
    assert!(variants
        .windows(2)
        .all(|pair| pair[0].solution != pair[1].solution));
    assert!(variants
        .iter()
        .all(|game| game.row_clues.iter().any(|clue| clue != &[0])));
}
