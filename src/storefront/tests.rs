//! Regression coverage for the tests module.

use super::*;

#[test]
fn purchase_message_names_the_locked_game_and_current_store() {
    let message = purchase_message("Blackjack");
    assert!(message.contains("Blackjack"));
    assert!(message.contains(PRIMARY_STORE));
}

#[test]
fn availability_message_keeps_locked_and_unreleased_copy_distinct() {
    assert_eq!(
        availability_message("Blackjack", GameAvailability::DemoRestricted),
        "Blackjack is in the full version — buy Idle Hands on itch.io"
    );
    assert_eq!(
        availability_message("Future Game", GameAvailability::ComingSoon),
        "Future Game is coming soon"
    );
}
