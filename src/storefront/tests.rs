use super::*;

#[test]
fn purchase_message_names_the_locked_game_and_current_store() {
    let message = purchase_message("Blackjack");
    assert!(message.contains("Blackjack"));
    assert!(message.contains(PRIMARY_STORE));
}
