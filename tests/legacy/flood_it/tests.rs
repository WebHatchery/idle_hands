//! Regression coverage for the tests module.

use idle_hands::testing::modules::flood_it::*;

#[test]
fn move_limit_produces_a_loss() {
    let mut game = FloodIt::new(5);
    // Alternating these colors cannot absorb the surrounding field of ones.
    game.cells.fill(1);
    game.cells[0] = 0;
    game.active_color = 0;
    for turn in 0..game.move_limit() {
        assert_eq!(game.phase, FloodPhase::Playing);
        assert!(game.choose(if turn % 2 == 0 { 2 } else { 0 }));
    }
    assert_eq!(game.phase, FloodPhase::Lost);
    assert_eq!(game.moves, game.move_limit());
    assert!(!game.choose(1));
}

#[test]
fn three_strong_growth_moves_earn_a_free_surge() {
    let mut game = FloodIt::new(10);
    let side = game.side();
    let colors = game.color_count() as usize;
    for row in 0..side {
        for col in 0..side {
            game.cells[row * side + col] = (col % colors) as u8;
        }
    }
    game.active_color = 0;

    for color in [1, 2, 3] {
        assert!(game.choose(color));
    }
    assert_eq!(game.surges, 1);
    assert_eq!(game.momentum, 0);
    assert_eq!(game.combo, 3);
    assert_eq!(game.points, 48);

    assert!(game.use_surge());
    assert_eq!(game.moves, 3);
    assert_eq!(game.surges, 0);
    assert_eq!(game.combo, 4);
    assert_eq!(game.points, 80);
    assert!(game.undo());
    assert_eq!(game.surges, 1);
    assert_eq!(game.moves, 3);
}

#[test]
fn full_history_rewinds_multiple_flood_choices() {
    let mut game = FloodIt::new(12);
    let start = game.cells.clone();
    let first = game.hint_color().unwrap();
    game.choose(first);
    let after_one = game.cells.clone();
    let second = game.hint_color().unwrap();
    game.choose(second);

    assert!(game.undo());
    assert_eq!(game.cells, after_one);
    assert!(game.undo());
    assert_eq!(game.cells, start);
    assert!(!game.undo());
}

#[test]
fn legacy_saves_default_to_an_unchained_field() {
    let mut value = serde_json::to_value(FloodIt::new(13)).unwrap();
    let object = value.as_object_mut().unwrap();
    for field in [
        "last_gain",
        "combo",
        "best_combo",
        "points",
        "momentum",
        "surges",
    ] {
        object.remove(field);
    }
    let loaded: FloodIt = serde_json::from_value(value).unwrap();
    assert_eq!(loaded.points, 0);
    assert_eq!(loaded.combo, 0);
    assert_eq!(loaded.surges, 0);
}
