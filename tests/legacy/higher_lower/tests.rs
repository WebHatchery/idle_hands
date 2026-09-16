//! Regression coverage for the tests module.

use super::*;

#[test]
fn an_incorrect_guess_ends_the_round() {
    let mut game = HigherLower::new(2);
    game.current = 13;
    game.next = 1;
    assert!(game.guess(Guess::Higher));
    assert_eq!(game.status, HigherLowerStatus::Lost);
}

#[test]
fn friendly_ties_win_while_house_ties_lose() {
    let mut friendly = HigherLower::new(3);
    friendly.current = 7;
    friendly.next = 7;
    assert!(friendly.guess(Guess::Higher));
    assert_eq!(friendly.score, 1);

    let mut house = HigherLower::new(3);
    house.rule = HigherLowerRule::House;
    house.current = 7;
    house.next = 7;
    assert!(house.guess(Guess::Higher));
    assert_eq!(house.status, HigherLowerStatus::Lost);
}

#[test]
fn cash_out_requires_a_real_run_and_undo_reopens_it() {
    let mut game = HigherLower::new(5);
    assert!(!game.cash_out());
    game.score = 2;
    game.pot = 30;
    assert!(game.cash_out());
    assert!(game.undo());
    assert_eq!(game.status, HigherLowerStatus::Playing);
    assert_eq!((game.pot, game.banked, game.cashed_out), (30, 0, false));
}

#[test]
fn house_rule_doubles_pot_growth_and_changes_displayed_odds() {
    let mut game = HigherLower::new(6);
    game.rule = HigherLowerRule::House;
    game.current = 4;
    game.next = 10;
    assert_eq!(game.chance(Guess::Higher), 69);
    assert_eq!(game.chance(Guess::Lower), 23);
    game.guess(Guess::Higher);
    assert_eq!(game.pot, 20);
    assert!(game.undo());
    assert_eq!((game.score, game.pot), (0, 0));
}
