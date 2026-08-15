use super::*;

#[test]
fn seeded_potions_repeat() {
    let first = Potion2048::new(42);
    let second = Potion2048::new(42);
    assert_eq!(first.cells, second.cells);
    assert_eq!(first.cells.iter().filter(|&&value| value != 0).count(), 2);
}

#[test]
fn merge_and_undo_restore_the_brew() {
    let mut game = Potion2048::new(1);
    game.cells = [2, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert!(game.move_in(Direction::Left));
    assert_eq!(&game.cells[0..3], &[4, 4, 0]);
    assert!(game.undo());
    assert_eq!(game.cells[0], 2);
}

#[test]
fn potion_target_is_higher_than_original_2048() {
    let mut game = Potion2048::new(1);
    game.cells[7] = 4096;
    assert!(game.won());
}
