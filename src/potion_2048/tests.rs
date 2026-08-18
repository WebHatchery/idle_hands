use super::*;

#[test]
fn seeded_potions_repeat() {
    let first = Potion2048::new(42);
    let second = Potion2048::new(42);
    assert_eq!(first.cells, second.cells);
    assert_eq!(first.cells.iter().filter(|&&value| value != 0).count(), 2);
}

#[test]
fn harder_difficulties_use_larger_grids_and_higher_targets() {
    let standard = Potion2048::new_with_difficulty(42, PotionDifficulty::Standard);
    let hard = Potion2048::new_with_difficulty(42, PotionDifficulty::Hard);
    let expert = Potion2048::new_with_difficulty(42, PotionDifficulty::Expert);
    assert_eq!(standard.cells.len(), 4 * 4);
    assert_eq!(hard.cells.len(), 5 * 5);
    assert_eq!(expert.cells.len(), 6 * 6);
    assert!(standard.target() < hard.target());
    assert!(hard.target() < expert.target());
}

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
fn potion_target_is_higher_than_original_2048() {
    let mut game = Potion2048::new(1);
    game.cells[7] = 4096;
    assert!(game.won());
}

#[test]
fn hint_picks_a_best_direction_without_mutating_the_brew() {
    let mut game = Potion2048::new(1);
    game.cells = vec![2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let before = game.clone();

    assert_eq!(game.hint_direction(), Some(Direction::Left));
    assert_eq!(game.cells, before.cells);
    assert_eq!(game.score, before.score);
    assert_eq!(game.seed, before.seed);
}

#[test]
fn hint_is_empty_after_reaching_the_potion_target() {
    let mut game = Potion2048::new(1);
    game.cells[0] = 4096;

    assert_eq!(game.hint_direction(), None);
}
