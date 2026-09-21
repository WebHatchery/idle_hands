//! Regression coverage for the tests module.

use idle_hands::testing::modules::spider::*;

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
