use super::*;

#[test]
fn seeded_deals_repeat_and_have_a_stock() {
    let first = Spider::new(42);
    let second = Spider::new(42);
    assert_eq!(first.tableau, second.tableau);
    assert_eq!(first.stock, second.stock);
    assert_eq!(first.stock.len(), 50);
    assert!(first
        .tableau
        .iter()
        .all(|stack| stack.last().unwrap().face_up));
}

#[test]
fn ordered_runs_move_and_undo_reveals_the_source() {
    let mut game = Spider::new(42);
    game.tableau[0] = vec![
        Card {
            rank: 2,
            suit: 0,
            face_up: false,
        },
        Card {
            rank: 12,
            suit: 0,
            face_up: true,
        },
        Card {
            rank: 11,
            suit: 0,
            face_up: true,
        },
    ];
    game.tableau[1] = vec![Card {
        rank: 13,
        suit: 0,
        face_up: true,
    }];
    assert!(game.select_column(0, 1));
    assert!(game.move_selected(1));
    assert_eq!(game.tableau[1].len(), 3);
    assert!(game.tableau[0][0].face_up);
    assert!(game.undo());
    assert_eq!(game.tableau[1].len(), 1);
    assert_eq!(game.moves, 0);
}

#[test]
fn tapping_a_selected_run_releases_it_or_replaces_it() {
    let mut game = Spider::new(45);
    game.tableau[0] = vec![Card {
        rank: 5,
        suit: 0,
        face_up: true,
    }];
    game.tableau[1] = vec![Card {
        rank: 9,
        suit: 0,
        face_up: true,
    }];

    assert!(game.select_column(0, 0));
    assert!(game.tap_column(0, 0));
    assert_eq!(game.selected, None);

    assert!(game.select_column(0, 0));
    assert!(game.tap_column(1, 0));
    assert_eq!(game.selected, Some((1, 0)));
}

#[test]
fn dealing_stock_clears_the_selected_run() {
    let mut game = Spider::new(46);
    assert!(game.select_column(0, game.tableau[0].len() - 1));
    assert!(game.deal_stock());
    assert_eq!(game.selected, None);
}

#[test]
fn complete_run_is_removed_and_counts_toward_win() {
    let mut game = Spider::new(42);
    game.tableau[0] = (1..=13)
        .rev()
        .map(|rank| Card {
            rank,
            suit: 0,
            face_up: true,
        })
        .collect();
    game.tableau[1] = Vec::new();
    assert!(is_complete_run(&game.tableau[0]));
    game.selected = Some((0, 0));
    assert!(game.move_selected(1));
    assert_eq!(game.completed, 1);
    assert!(game.tableau[1].is_empty());
}

#[test]
fn hint_returns_the_first_legal_run_move_without_mutating_the_tableau() {
    let mut game = Spider::new(43);
    game.tableau[0] = vec![
        Card {
            rank: 4,
            suit: 0,
            face_up: false,
        },
        Card {
            rank: 12,
            suit: 0,
            face_up: true,
        },
        Card {
            rank: 11,
            suit: 0,
            face_up: true,
        },
    ];
    game.tableau[1] = vec![Card {
        rank: 13,
        suit: 0,
        face_up: true,
    }];
    let before = game.tableau.clone();

    assert_eq!(game.hint_move(), Some((0, 1, 1)));
    assert_eq!(game.hint_move(), Some((0, 1, 1)));
    assert_eq!(game.tableau, before);
    assert_eq!(game.selected, None);
    assert_eq!(game.moves, 0);
}

#[test]
fn won_spider_has_no_hint_move() {
    let mut game = Spider::new(44);
    game.status = SpiderStatus::Won;
    assert_eq!(game.hint_move(), None);
}

#[test]
fn two_suit_mode_separates_runs_while_preserving_the_eight_run_goal() {
    let game = Spider::new_with_mode(58, SpiderMode::TwoSuit);
    let suits = game
        .tableau
        .iter()
        .flatten()
        .chain(game.stock.iter())
        .map(|card| card.suit)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(suits, [0, 1].into_iter().collect());
    assert_eq!(game.completed, 0);
}
