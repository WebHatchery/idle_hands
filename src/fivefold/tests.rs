use super::*;

#[test]
fn seeded_rolls_repeat_and_holds_survive_a_roll() {
    let mut a = Fivefold::new(44);
    let mut b = Fivefold::new(44);
    assert!(a.roll());
    assert!(b.roll());
    assert_eq!(a.dice, b.dice);
    assert!(a.toggle_hold(0));
    let held = a.dice[0];
    assert!(a.roll());
    assert_eq!(a.dice[0], held);
}

#[test]
fn score_categories_cover_straights_houses_and_kinds() {
    let mut game = Fivefold::new(1);
    game.dice = [1, 2, 3, 4, 6];
    assert_eq!(game.score_for(Category::SmallStraight), 30);
    assert_eq!(game.score_for(Category::LargeStraight), 0);
    game.dice = [2, 2, 3, 3, 3];
    assert_eq!(game.score_for(Category::FullHouse), 25);
    assert_eq!(game.score_for(Category::ThreeKind), 13);
    game.dice = [5, 5, 5, 5, 5];
    assert_eq!(game.score_for(Category::FiveOfKind), 50);
    assert_eq!(game.score_for(Category::FourKind), 25);
}

#[test]
fn choosing_each_category_advances_until_the_scorecard_is_complete() {
    let mut game = Fivefold::new(2);
    for category in Category::ALL {
        assert!(game.roll());
        assert!(game.choose_category(category));
    }
    assert_eq!(game.status, FivefoldStatus::Complete);
    assert!(game.total() > 0);
}

#[test]
fn every_score_category_uses_the_expected_preview_value() {
    let mut game = Fivefold::new(8);
    game.dice = [1, 1, 1, 4, 6];
    assert_eq!(game.score_for(Category::Ones), 3);
    assert_eq!(game.score_for(Category::Twos), 0);
    assert_eq!(game.score_for(Category::Threes), 0);
    assert_eq!(game.score_for(Category::Fours), 4);
    assert_eq!(game.score_for(Category::Fives), 0);
    assert_eq!(game.score_for(Category::Sixes), 6);
    assert_eq!(game.score_for(Category::Chance), 13);
    assert_eq!(game.score_for(Category::ThreeKind), 13);
    assert_eq!(game.score_for(Category::FourKind), 0);
    game.dice = [1, 2, 3, 4, 5];
    assert_eq!(game.score_for(Category::LargeStraight), 40);
    game.dice = [2, 2, 2, 2, 5];
    assert_eq!(game.score_for(Category::FourKind), 13);
}

#[test]
fn a_turn_stops_after_three_rolls() {
    let mut game = Fivefold::new(3);
    assert!(game.roll());
    assert!(game.roll());
    assert!(game.roll());
    assert!(!game.roll());
}
