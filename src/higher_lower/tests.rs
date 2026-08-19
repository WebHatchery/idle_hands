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

#[test]
fn hint_chooses_the_better_odds_without_revealing_the_next_card() {
    let mut game = HigherLower::new(1);
    game.current = 4;
    game.next = 13;
    let before = game.clone();

    assert_eq!(game.hint_guess(), Some(Guess::Higher));
    assert_eq!(game.next, before.next);
    assert_eq!(game.score, before.score);
    assert_eq!(game.moves, before.moves);
}

#[test]
fn hint_is_empty_after_the_round_ends() {
    let mut game = HigherLower::new(1);
    game.status = HigherLowerStatus::Lost;

    assert_eq!(game.hint_guess(), None);
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
fn escalating_pot_can_be_cashed_after_two_correct_guesses() {
    let mut game = HigherLower::new(4);
    game.current = 2;
    game.next = 8;
    game.guess(Guess::Higher);
    game.next = 9;
    game.guess(Guess::Higher);
    assert_eq!((game.score, game.pot), (2, 30));
    assert!(game.cash_out());
    assert_eq!(game.status, HigherLowerStatus::Won);
    assert_eq!((game.banked, game.cashed_out), (30, true));
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

#[test]
fn changing_rule_starts_fresh_and_reset_preserves_it() {
    let mut game = HigherLower::new(7);
    game.score = 4;
    game.set_rule(HigherLowerRule::House, 8);
    assert_eq!((game.rule, game.score), (HigherLowerRule::House, 0));
    game.reset(9);
    assert_eq!(game.rule, HigherLowerRule::House);
}

#[test]
fn legacy_saves_receive_depth_defaults() {
    let original = HigherLower::new(10);
    let mut value = serde_json::to_value(&original).unwrap();
    for field in ["rule", "pot", "banked", "cashed_out"] {
        value.as_object_mut().unwrap().remove(field);
    }
    let restored: HigherLower = serde_json::from_value(value).unwrap();
    assert_eq!(restored.rule, HigherLowerRule::Friendly);
    assert_eq!(
        (restored.pot, restored.banked, restored.cashed_out),
        (0, 0, false)
    );
}
