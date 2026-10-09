//! Generated Sum Circuit rounds must remain solvable.

use idle_hands::testing::modules::misc_games::*;

#[test]
fn sum_circuit_keeps_generating_solvable_rounds() {
    let mut game = MiscGame::new(7, MiscKind::SumCircuit);
    for round in 0..300 {
        for index in game.solution.clone() {
            assert!(game.tap(index));
        }
        assert!(game.submit());
        assert_eq!(game.round, round + 1);
        assert!(!game.won());
        assert!(game.selected.is_empty());
    }
}
