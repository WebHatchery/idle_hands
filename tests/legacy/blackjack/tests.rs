//! Regression coverage for the tests module.

use idle_hands::testing::modules::blackjack::*;

#[test]
fn ace_total_uses_soft_value_when_safe() {
    let mut game = Blackjack::new(1);
    game.player = vec![
        Card {
            rank: 1,
            suit: 0,
            face_up: true,
        },
        Card {
            rank: 6,
            suit: 0,
            face_up: true,
        },
    ];
    assert_eq!(game.player_total(), 17);
    game.player.push(Card {
        rank: 9,
        suit: 0,
        face_up: true,
    });
    assert_eq!(game.player_total(), 16);
}

#[test]
fn hit_can_be_undone_and_stand_resolves() {
    let mut game = Blackjack::new(4);
    let before = game.player.clone();
    assert!(game.hit());
    assert_ne!(game.player, before);
    assert!(game.undo());
    assert_eq!(game.player, before);
    assert!(game.stand());
    assert_ne!(game.status, BlackjackStatus::Playing);
}

#[test]
fn hint_uses_player_total_and_visible_dealer_upcard_without_revealing_hidden_card() {
    let mut game = Blackjack::new(1);
    game.player = vec![
        Card {
            rank: 10,
            suit: 0,
            face_up: true,
        },
        Card {
            rank: 6,
            suit: 0,
            face_up: true,
        },
    ];
    game.dealer[1].rank = 5;
    let hidden = game.dealer[0];

    assert_eq!(game.hint_action(), Some(BlackjackHint::Stand));
    assert_eq!(game.dealer[0], hidden);
    assert_eq!(game.deck.len(), 48);
}

#[test]
fn finished_rounds_reject_more_actions() {
    for status in [
        BlackjackStatus::Won,
        BlackjackStatus::Lost,
        BlackjackStatus::Push,
    ] {
        let mut game = Blackjack::new(9);
        game.status = status;
        let before = serde_json::to_value(&game).unwrap();
        assert!(!game.hit(), "{status:?}");
        assert!(!game.stand(), "{status:?}");
        assert_eq!(serde_json::to_value(&game).unwrap(), before, "{status:?}");
    }
}
