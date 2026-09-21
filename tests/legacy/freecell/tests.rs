//! Regression coverage for the tests module.

use idle_hands::testing::cards::Card;
use idle_hands::testing::modules::freecell::*;

#[test]
fn foundations_require_the_next_card_of_the_matching_suit() {
    let mut game = FreeCell::new(2);
    game.cells[0] = Some(Card {
        rank: 1,
        suit: 2,
        face_up: true,
    });
    assert!(game.select_cell(0));
    assert!(game.move_selected_to_foundation(2));
    assert_eq!(game.foundations[2], 1);
    assert!(game.undo());
    assert_eq!(game.cells[0].map(|card| card.rank), Some(1));
}

#[test]
fn an_invalid_stack_cannot_be_moved_as_a_supermove() {
    let mut game = FreeCell::new(3);
    game.cascades[0] = vec![
        Card {
            rank: 9,
            suit: 0,
            face_up: true,
        },
        Card {
            rank: 7,
            suit: 1,
            face_up: true,
        },
    ];
    game.cascades[1].clear();
    assert!(game.select_cascade(0, 0));
    assert!(!game.move_selected_to_cascade(1));
}

#[test]
fn empty_column_capacity_rejects_two_cards_without_free_space() {
    let mut game = FreeCell::new(8);
    game.cells = [
        Some(Card {
            rank: 1,
            suit: 0,
            face_up: true,
        }),
        Some(Card {
            rank: 2,
            suit: 0,
            face_up: true,
        }),
        Some(Card {
            rank: 3,
            suit: 0,
            face_up: true,
        }),
        Some(Card {
            rank: 4,
            suit: 0,
            face_up: true,
        }),
    ];
    game.cascades = vec![
        vec![
            Card {
                rank: 9,
                suit: 0,
                face_up: true,
            },
            Card {
                rank: 8,
                suit: 2,
                face_up: true,
            },
        ],
        vec![Card {
            rank: 13,
            suit: 1,
            face_up: true,
        }],
        vec![Card {
            rank: 12,
            suit: 0,
            face_up: true,
        }],
        vec![Card {
            rank: 11,
            suit: 1,
            face_up: true,
        }],
        vec![Card {
            rank: 10,
            suit: 0,
            face_up: true,
        }],
        vec![Card {
            rank: 9,
            suit: 1,
            face_up: true,
        }],
        vec![Card {
            rank: 8,
            suit: 0,
            face_up: true,
        }],
        vec![],
    ];
    assert!(game.select_cascade(0, 0));
    assert!(!game.move_selected_to_cascade(7));
}

#[test]
fn select_then_select_moves_a_legal_cascade_stack() {
    let mut game = FreeCell::new(12);
    game.cascades[0] = vec![
        Card {
            rank: 8,
            suit: 0,
            face_up: true,
        },
        Card {
            rank: 7,
            suit: 2,
            face_up: true,
        },
    ];
    game.cascades[1] = vec![Card {
        rank: 9,
        suit: 2,
        face_up: true,
    }];
    assert!(game.select_cascade(0, 0));
    assert!(game.move_selected_to_cascade(1));
    assert!(game.cascades[0].is_empty());
    assert_eq!(game.cascades[1].len(), 3);
}
