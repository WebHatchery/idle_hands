//! The exact endgame solver must account for both terminal rules.

use idle_hands::testing::modules::nim::*;

#[test]
fn exact_solver_finds_a_winning_move_under_both_end_rules() {
    let mut normal = Nim::new(1);
    normal.heaps = [0, 1, 2];
    normal.rule = NimRule::Normal;
    let normal_hint = normal.hint_move().unwrap();
    assert_eq!(
        normal.move_is_winning(normal_hint.0, normal_hint.1),
        Some(true)
    );

    let mut misere = normal.clone();
    misere.rule = NimRule::Misere;
    let misere_hint = misere.hint_move().unwrap();
    assert_eq!(
        misere.move_is_winning(misere_hint.0, misere_hint.1),
        Some(true)
    );
}
