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
    assert_eq!(standard.difficulty.catalyst_chain(), 3);
    assert_eq!(hard.difficulty.catalyst_chain(), 4);
    assert_eq!(expert.difficulty.catalyst_chain(), 5);
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
fn sliding_without_a_reaction_breaks_the_combo() {
    let mut game = Potion2048::new(1);
    game.cells = vec![0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    game.combo = 2;

    assert!(game.move_in(Direction::Left));
    assert_eq!(game.combo, 0);
    assert_eq!(game.last_merges, 0);
}

#[test]
fn third_standard_reaction_brews_a_wild_catalyst() {
    let mut game = Potion2048::new(1);
    for expected_combo in 1..=3 {
        game.cells = vec![2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert!(game.move_in(Direction::Left));
        assert_eq!(game.combo, expected_combo);
    }

    assert_eq!(game.catalysts_brewed, 1);
    assert_eq!(game.cells.iter().filter(|value| **value == 1).count(), 1);
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

#[test]
fn legacy_saves_default_to_an_unchained_brew() {
    let mut value = serde_json::to_value(Potion2048::new(1)).unwrap();
    for field in ["combo", "best_combo", "catalysts_brewed", "last_merges"] {
        value.as_object_mut().unwrap().remove(field);
    }
    let loaded: Potion2048 = serde_json::from_value(value).unwrap();

    assert_eq!(loaded.combo, 0);
    assert_eq!(loaded.best_combo, 0);
    assert_eq!(loaded.catalysts_brewed, 0);
}
