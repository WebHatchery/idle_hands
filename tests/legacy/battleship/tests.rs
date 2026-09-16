//! Regression coverage for the tests module.

use super::*;

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
