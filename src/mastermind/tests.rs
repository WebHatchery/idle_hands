use super::*;

#[test]
fn seeded_secret_repeats_and_feedback_counts_exact_and_partial_pegs() {
    let first = Mastermind::new(42);
    assert_eq!(first.secret, Mastermind::new(42).secret);
    assert_eq!(score_guess(&[1, 2, 3, 4], &[1, 3, 4, 5]), (1, 2));
}

#[test]
fn picks_fill_a_guess_and_submit_records_feedback() {
    let mut game = Mastermind::new(42);
    game.secret = [1, 2, 3, 4];
    for color in [1, 2, 3, 4] {
        assert!(game.pick(color));
    }
    assert!(game.submit());
    assert_eq!(game.status, MastermindStatus::Won);
    assert_eq!(game.exact[0], 4);
    assert_eq!(game.row, 1);
}

#[test]
fn undo_restores_a_partial_guess_and_clear_empties_it() {
    let mut game = Mastermind::new(7);
    assert!(game.pick(2));
    assert!(game.pick(4));
    assert!(game.clear());
    assert_eq!(game.current, [EMPTY; PEGS]);
    assert!(game.undo());
    assert_eq!(game.current[0], 2);
    assert_eq!(game.current[1], 4);
}

#[test]
fn hint_pick_suggests_a_valid_color_without_mutating_the_game() {
    let game = Mastermind::new(42);
    let before = game.current;
    let (slot, color) = game.hint_pick().unwrap();
    assert_eq!(slot, 0);
    assert!(color < 6);
    assert_eq!(game.current, before);
}
