use super::*;

#[test]
fn seeded_layouts_are_repeatable_and_hold_two_non_overlapping_ships() {
    let first = Battleship::new(0);
    assert_eq!(first.ships, Battleship::new(0).ships);
    assert_eq!(first.ships.iter().filter(|cell| **cell == 1).count(), 3);
    assert_eq!(first.ships.iter().filter(|cell| **cell == 2).count(), 2);
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
