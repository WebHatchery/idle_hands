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
