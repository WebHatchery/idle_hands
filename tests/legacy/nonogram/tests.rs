//! Regression coverage for the tests module.

use idle_hands::testing::modules::nonogram::*;

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
fn strokes_lock_to_the_dominant_axis() {
    assert_eq!(stroke_indices(5, (0, 2), (4, 3)), vec![10, 11, 12, 13, 14]);
    assert_eq!(stroke_indices(5, (2, 0), (3, 4)), vec![2, 7, 12, 17, 22]);
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
