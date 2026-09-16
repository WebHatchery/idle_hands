//! Regression coverage for the tests module.

use super::*;

#[test]
fn rejects_larger_disk_on_smaller_disk() {
    let mut game = Hanoi::new(2);
    game.tap_peg(0);
    game.tap_peg(1);
    game.tap_peg(0);
    assert!(!game.tap_peg(1));
    assert_eq!(game.moves, 1);
}

#[test]
fn rotating_disk_variants_remains_solvable() {
    for disks in [3, 5, 7] {
        let mut game = Hanoi::new_with_disks(1, disks);
        for _ in 0..game.optimal_moves() {
            let Some((source, destination)) = game.hint_move() else {
                break;
            };
            assert!(game.tap_peg(source));
            assert!(game.tap_peg(destination));
            if game.won() {
                break;
            }
        }
        assert!(game.won(), "{disks}-disk room should be solvable");
        assert_eq!(game.moves, game.optimal_moves());
    }
}

#[test]
fn multiple_undos_rewind_multiple_disk_moves() {
    let mut game = Hanoi::new_with_disks(2, 3);
    assert!(game.tap_peg(0));
    assert!(game.tap_peg(2));
    let after_one = game.stacks.clone();
    assert!(game.tap_peg(0));
    assert!(game.tap_peg(1));

    assert!(game.undo());
    assert_eq!(game.stacks, after_one);
    assert!(game.undo());
    assert_eq!(game.stacks, [vec![3, 2, 1], vec![], vec![]]);
    assert!(!game.undo());
}

#[test]
fn optimal_targets_and_clear_ranks_scale_with_disk_count() {
    let mut game = Hanoi::new_with_disks(4, 3);
    assert_eq!(game.optimal_moves(), 7);
    game.phase = HanoiPhase::Won;
    game.moves = 7;
    assert_eq!(game.clear_rank(), "PERFECT");
    game.moves = 9;
    assert_eq!(game.clear_rank(), "CLOSE");
    game.moves = 12;
    assert_eq!(game.clear_rank(), "CLEAR");
    assert_eq!(Hanoi::new_with_disks(4, 7).optimal_moves(), 127);
}
