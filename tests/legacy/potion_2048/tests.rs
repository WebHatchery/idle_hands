//! Regression coverage for the tests module.

use super::*;

#[test]
fn merge_and_undo_restore_the_brew() {
    let mut game = Potion2048::new(1);
    game.cells = vec![2, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert!(game.move_in(Direction::Left));
    assert_eq!(&game.cells[0..3], &[4, 4, 0]);
    assert!(game.undo());
    assert_eq!(game.cells[0], 2);
}

#[test]
fn consecutive_reactions_build_a_score_multiplier() {
    let mut game = Potion2048::new(1);
    game.cells = vec![2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert!(game.move_in(Direction::Left));
    assert_eq!((game.score, game.combo), (4, 1));

    game.cells = vec![4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert!(game.move_in(Direction::Left));
    assert_eq!((game.score, game.combo), (20, 2));
    assert_eq!(game.best_combo, 2);
}

#[test]
fn catalyst_reacts_with_any_potion_tier() {
    assert_eq!(reaction(1, 8), Some(16));
    assert_eq!(reaction(32, 1), Some(64));
    assert_eq!(reaction(1, 1), Some(2));
    assert_eq!(reaction(4, 8), None);

    let mut game = Potion2048::new(1);
    game.cells = vec![1, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert!(game.move_in(Direction::Left));
    assert_eq!(game.cells[0], 16);
}

#[test]
fn undo_restores_combo_and_catalyst_history() {
    let mut game = Potion2048::new(1);
    game.cells = vec![2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    game.combo = 2;
    game.best_combo = 2;
    assert!(game.move_in(Direction::Left));
    assert_eq!(game.catalysts_brewed, 1);

    assert!(game.undo());
    assert_eq!(game.combo, 2);
    assert_eq!(game.best_combo, 2);
    assert_eq!(game.catalysts_brewed, 0);
    assert_eq!(&game.cells[0..2], &[2, 2]);
}
