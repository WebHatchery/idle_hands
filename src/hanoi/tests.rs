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
