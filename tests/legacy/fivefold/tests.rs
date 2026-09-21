//! Regression coverage for the tests module.

use idle_hands::testing::modules::fivefold::*;

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

#[test]
fn quick_variant_completes_after_nine_calls_and_wild_fivefold_scores_more() {
    let mut quick = Fivefold::new_with_variant(12, FivefoldVariant::Quick);
    for category in Category::ALL.into_iter().take(9) {
        assert!(quick.roll());
        assert!(quick.choose_category(category));
    }
    assert_eq!(quick.status, FivefoldStatus::Complete);

    let mut wild = Fivefold::new_with_variant(13, FivefoldVariant::Wild);
    wild.dice = [5, 5, 5, 5, 5];
    assert_eq!(wild.score_for(Category::FiveOfKind), 75);
}
