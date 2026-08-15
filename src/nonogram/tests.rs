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
