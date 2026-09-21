//! Regression coverage for the tests module.

use idle_hands::testing::modules::storefront_data::*;

#[test]
fn full_build_keeps_every_active_drawer_playable() {
    assert_eq!(
        availability_for_build(GameId::Blackjack, false),
        GameAvailability::Playable
    );
    assert_eq!(
        cabinet_label_for_build(GameId::Blackjack, false, false),
        "Hold the hand"
    );
}

#[test]
fn demo_build_marks_non_demo_drawers_as_purchase_locked() {
    assert_eq!(
        availability_for_build(GameId::Blackjack, true),
        GameAvailability::DemoRestricted
    );
    assert_eq!(
        cabinet_label_for_build(GameId::Blackjack, true, false),
        "FULL VERSION · BUY ON ITCH.IO"
    );
    assert_eq!(
        cabinet_label_for_build(GameId::Blackjack, true, true),
        "FULL VERSION · ITCH.IO"
    );
}

#[test]
fn demo_drawers_remain_playable_and_actions_share_the_same_contract() {
    assert_eq!(
        availability_for_build(GameId::Solitaire, true),
        GameAvailability::Playable
    );
    assert_eq!(GameAvailability::Playable.action_label(), "OPEN");
    assert_eq!(GameAvailability::DemoRestricted.action_label(), "FULL");
    assert_eq!(GameAvailability::ComingSoon.action_label(), "SOON");
}

#[test]
fn demo_badge_reports_the_curated_count_and_stays_hidden_in_full_builds() {
    assert_eq!(build_badge_for(true).as_deref(), Some("DEMO · 30 GAMES"));
    assert_eq!(build_badge_for(false), None);
}
