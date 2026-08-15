use super::*;

#[test]
fn seeded_rounds_repeat() {
    let first = HigherLower::new(7);
    let second = HigherLower::new(7);
    assert_eq!(first.current, second.current);
    assert_eq!(first.next, second.next);
}

#[test]
fn a_guess_advances_and_undo_restores() {
    let mut game = HigherLower::new(1);
    let current = game.current;
    assert!(game.guess(Guess::Higher));
    assert_eq!(game.moves, 1);
    assert!(game.undo());
    assert_eq!(game.current, current);
    assert_eq!(game.moves, 0);
}

#[test]
fn an_incorrect_guess_ends_the_round() {
    let mut game = HigherLower::new(2);
    game.current = 13;
    game.next = 1;
    assert!(game.guess(Guess::Higher));
    assert_eq!(game.status, HigherLowerStatus::Lost);
}
