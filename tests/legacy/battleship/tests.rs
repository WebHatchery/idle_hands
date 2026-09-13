//! Regression coverage for the tests module.

use super::*;

#[test]
fn seeded_layouts_are_repeatable_and_hold_three_non_overlapping_ships() {
    let first = Battleship::new(0);
    assert_eq!(first.ships, Battleship::new(0).ships);
    assert_eq!(first.ships.iter().filter(|cell| **cell == 1).count(), 3);
    assert_eq!(first.ships.iter().filter(|cell| **cell == 2).count(), 2);
    assert_eq!(first.ships.iter().filter(|cell| **cell == 3).count(), 2);
    assert_eq!(first.ship_count(), 3);
    assert_ne!(first.ships, Battleship::new(1).ships);
}

#[test]
fn firing_reveals_hit_or_miss_and_rejects_repeat_shots() {
    let mut game = Battleship::new(0);
    let hit = game.ships.iter().position(|ship| *ship != 0).unwrap();
    let miss = game.ships.iter().position(|ship| *ship == 0).unwrap();
    assert!(game.fire(hit));
    assert_eq!(game.shots[hit], Shot::Hit);
    assert!(game.fire(miss));
    assert_eq!(game.shots[miss], Shot::Miss);
    assert!(!game.fire(hit));
    assert_eq!(game.moves, 2);
}

#[test]
fn undo_restores_the_unfired_cell_and_move_count() {
    let mut game = Battleship::new(2);
    let cell = 0;
    assert!(game.fire(cell));
    assert!(game.undo());
    assert_eq!(game.shots[cell], Shot::Unknown);
    assert_eq!(game.moves, 0);
    assert!(!game.undo());
}

#[test]
fn hit_chains_score_more_and_sinking_adds_a_bonus() {
    let mut game = Battleship::new(0);
    assert!(game.fire(1));
    assert_eq!((game.streak, game.score), (1, 10));
    assert!(game.fire(2));
    assert_eq!((game.streak, game.score), (2, 30));
    assert!(game.fire(3));
    assert!(game.ship_sunk(1));
    assert_eq!((game.streak, game.best_streak, game.score), (3, 3, 85));
    assert_eq!(game.sunk_ships(), 1);
    assert!(game.fire(0));
    assert_eq!(game.streak, 0);
}

#[test]
fn sonar_sweeps_a_clamped_three_by_three_area_and_finds_contacts() {
    let mut game = Battleship::new(0);
    assert!(game.toggle_sonar());
    assert!(game.sonar_armed);
    assert!(game.fire(2));
    assert!(!game.sonar_armed);
    assert_eq!(game.sonar_charges, 1);
    assert!(game.is_scanned(1));
    assert!(game.is_scanned(2));
    assert!(game.is_scanned(8));
    assert!(!game.is_scanned(20));
    assert_eq!(game.contact_count(), 3);
    assert_eq!(game.shots[2], Shot::Unknown);
    assert_eq!(game.moves, 0);
    assert_eq!(game.hint_cell(), Some(1));
}

#[test]
fn sonar_and_shots_share_a_full_undo_history() {
    let mut game = Battleship::new(1);
    assert!(game.toggle_sonar());
    assert!(game.fire(0));
    assert!(game.fire(1));
    assert_eq!(game.moves, 1);
    assert!(game.undo());
    assert_eq!(game.moves, 0);
    assert!(game.is_scanned(0));
    assert!(game.undo());
    assert_eq!(game.sonar_charges, 2);
    assert!(!game.is_scanned(0));
    assert!(!game.undo());
}

#[test]
fn firing_all_ship_cells_wins_and_finished_games_stop() {
    let mut game = Battleship::new(3);
    for cell in 0..CELLS {
        if game.ships[cell] != 0 {
            assert!(game.fire(cell));
        }
    }
    assert!(game.won());
    assert!(!game.fire(0));
}

#[test]
fn reset_starts_a_new_hidden_fleet() {
    let mut game = Battleship::new(0);
    let old = game.ships.clone();
    game.reset(1);
    assert_ne!(old, game.ships);
    assert_eq!(game.moves, 0);
    assert_eq!(game.hits(), 0);
    assert_eq!(game.sonar_charges, 2);
}

#[test]
fn legacy_saves_receive_two_sonar_sweeps_and_safe_empty_intel() {
    let original = Battleship::new(2);
    let mut value = serde_json::to_value(&original).unwrap();
    let object = value.as_object_mut().unwrap();
    for field in [
        "streak",
        "best_streak",
        "score",
        "sonar_charges",
        "sonar_armed",
        "scanned",
    ] {
        object.remove(field);
    }
    let restored: Battleship = serde_json::from_value(value).unwrap();
    assert_eq!(restored.sonar_charges, 2);
    assert_eq!(restored.score, 0);
    assert!(!restored.is_scanned(0));
}

#[test]
fn hint_uses_a_neighbor_after_a_hit_without_mutating_shots() {
    let mut game = Battleship::new(15);
    game.shots[0] = Shot::Hit;
    let before = game.shots.clone();

    assert_eq!(game.hint_cell(), Some(1));
    assert_eq!(game.shots, before);
    assert_eq!(game.moves, 0);
}

#[test]
fn hint_falls_back_to_a_checkerboard_cell_and_ends_cleanly() {
    let game = Battleship::new(16);

    assert_eq!(game.hint_cell(), Some(0));
    let mut finished = game.clone();
    finished.phase = BattleshipPhase::Won;
    assert_eq!(finished.hint_cell(), None);
}

#[test]
fn armada_adds_a_fourth_ship_to_the_existing_fleet() {
    let game = Battleship::new_with_fleet(59, BattleshipFleet::Armada);
    assert_eq!(game.ship_count(), 4);
    assert!(game.ships.contains(&4));
}
