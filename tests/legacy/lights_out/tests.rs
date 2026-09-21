//! Regression coverage for the tests module.

use idle_hands::testing::modules::lights_out::*;

#[test]
fn pressing_a_cell_toggles_its_cross_and_undo_restores_it() {
    let mut game = LightsOut::new(7);
    game.cells = [false; CELLS];
    game.moves = 0;
    assert!(game.press(0));
    assert_eq!(game.cells[0..3], [true, true, false]);
    assert!(game.cells[5]);
    assert_eq!(game.moves, 1);
    assert!(game.undo());
    assert_eq!(game.cells, [false; CELLS]);
    assert_eq!(game.moves, 0);
}

#[test]
fn exact_solver_returns_a_route_that_clears_the_board() {
    let game = LightsOut::new(8);
    let solution = game.minimum_solution();
    assert!(!solution.is_empty());
    assert_eq!(solution.len(), game.displayed_par());
    let mut replay = game.clone();
    for press in solution {
        replay.press(press);
    }
    assert_eq!(replay.status, LightsOutStatus::Won);
}

#[test]
fn multi_step_undo_walks_back_multiple_presses() {
    let mut game = LightsOut::new(12);
    let original = game.cells;
    game.press(0);
    game.press(1);
    assert!(game.undo());
    assert_eq!(game.moves, 1);
    assert!(game.undo());
    assert_eq!(game.cells, original);
}

#[test]
fn legacy_saves_receive_depth_defaults() {
    let original = LightsOut::new(13);
    let mut value = serde_json::to_value(&original).unwrap();
    for field in ["difficulty", "par", "guide"] {
        value.as_object_mut().unwrap().remove(field);
    }
    let restored: LightsOut = serde_json::from_value(value).unwrap();
    assert_eq!(restored.difficulty, LightsDifficulty::Classic);
    assert!(!restored.guide);
    assert!(restored.displayed_par() > 0);
}
