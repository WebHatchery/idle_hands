use super::*;

#[test]
fn seeded_rounds_repeat_with_two_hands() {
    let first = Blackjack::new(12);
    let second = Blackjack::new(12);
    assert_eq!(first.player, second.player);
    assert_eq!(first.dealer, second.dealer);
    assert_eq!(first.player.len(), 2);
    assert_eq!(first.dealer.len(), 2);
    assert_eq!(first.deck.len(), 48);
}

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
fn finished_round_rejects_more_actions() {
    let mut game = Blackjack::new(9);
    game.status = BlackjackStatus::Lost;
    assert!(!game.hit());
    assert!(!game.stand());
}

#[test]
fn new_round_keeps_session_win_count_and_advances_round() {
    let mut game = Blackjack::new(9);
    game.wins = 2;
    game.rounds = 3;
    game.reset(10);
    assert_eq!(game.rounds, 4);
    assert!(game.wins >= 2);
}
