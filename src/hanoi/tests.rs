//! Regression coverage for the tests module.

use super::*;

#[test]
fn selects_and_moves_a_top_disk() {
    let mut game = Hanoi::new(1);
    assert!(game.tap_peg(0));
    assert!(game.tap_peg(1));
    assert_eq!(game.stacks[0], vec![5, 4, 3, 2]);
    assert_eq!(game.stacks[1], vec![1]);
    assert_eq!(game.moves, 1);
}

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
fn undo_restores_the_selected_stack_state() {
    let mut game = Hanoi::new(3);
    game.tap_peg(0);
    game.tap_peg(1);
    assert!(game.undo());
    assert_eq!(game.stacks, [vec![5, 4, 3, 2, 1], vec![], vec![]]);
    assert_eq!(game.moves, 0);
}

#[test]
fn the_classic_solution_wins() {
    let mut game = Hanoi::new(4);
    for (source, destination) in [
        (0, 2),
        (0, 1),
        (2, 1),
        (0, 2),
        (1, 0),
        (1, 2),
        (0, 2),
        (0, 1),
        (2, 1),
        (2, 0),
        (1, 0),
        (2, 1),
        (0, 2),
        (0, 1),
        (2, 1),
        (0, 2),
        (1, 0),
        (1, 2),
        (0, 2),
        (1, 0),
        (2, 1),
        (2, 0),
        (1, 0),
        (1, 2),
        (0, 2),
        (0, 1),
        (2, 1),
        (0, 2),
        (1, 0),
        (1, 2),
        (0, 2),
    ] {
        assert!(game.tap_peg(source));
        assert!(game.tap_peg(destination));
    }
    assert!(game.won());
    assert_eq!(game.moves, 31);
}

#[test]
fn reset_starts_the_five_disk_room() {
    let mut game = Hanoi::new(5);
    game.tap_peg(0);
    game.tap_peg(1);
    game.reset(6);
    assert_eq!(game.seed, 6);
    assert_eq!(game.moves, 0);
    assert_eq!(game.stacks[0].len(), 5);
    assert_eq!(game.phase, HanoiPhase::Playing);
}

#[test]
fn hint_returns_the_first_shortest_move_without_mutating_the_stacks() {
    let game = Hanoi::new(7);
    let before = game.stacks.clone();

    assert_eq!(game.hint_move(), Some((0, 2)));
    assert_eq!(game.stacks, before);
    assert_eq!(game.selected, None);
    assert_eq!(game.moves, 0);
}

#[test]
fn following_hints_solves_the_hanoi_room() {
    let mut game = Hanoi::new(8);
    for _ in 0..31 {
        let Some((source, destination)) = game.hint_move() else {
            break;
        };
        assert!(game.tap_peg(source));
        assert!(game.tap_peg(destination));
        if game.won() {
            break;
        }
    }
    assert!(game.won());
    assert_eq!(game.hint_move(), None);
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
fn legal_destination_preview_respects_disk_sizes() {
    let mut game = Hanoi::new_with_disks(3, 3);
    game.stacks = [vec![3], vec![2], vec![1]];

    assert!(!game.can_move(0, 1));
    assert!(!game.can_move(0, 2));
    assert!(game.can_move(2, 0));
    assert!(game.can_move(2, 1));
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

#[test]
fn disk_selector_builds_a_fresh_master_tower() {
    let mut game = Hanoi::new(5);
    game.tap_peg(0);
    game.tap_peg(2);
    game.set_disks(7, 6);

    assert_eq!(game.seed, 6);
    assert_eq!(game.disks, 7);
    assert_eq!(game.stacks[0], vec![7, 6, 5, 4, 3, 2, 1]);
    assert_eq!(game.moves, 0);
    assert!(!game.undo());
}
