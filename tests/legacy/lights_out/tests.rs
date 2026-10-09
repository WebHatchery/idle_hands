//! The exact Lights Out solver must return a route that clears the board.

use idle_hands::testing::modules::lights_out::*;

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
