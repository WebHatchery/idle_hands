use super::*;

#[test]
fn seeded_deals_are_reproducible_and_complete() {
    let a = FreeCell::new(9);
    let b = FreeCell::new(9);
    assert_eq!(a.cascades, b.cascades);
    assert_eq!(a.cascades.iter().map(Vec::len).sum::<usize>(), 52);
}

#[test]
fn a_free_cell_can_be_moved_to_an_empty_cascade() {
    let mut game = FreeCell::new(4);
    let card = Card {
        rank: 7,
        suit: 0,
        face_up: true,
    };
    game.cells[0] = Some(card);
    game.cascades[0].clear();
    assert!(game.select_cell(0));
    assert!(game.move_selected_to_cascade(0));
    assert_eq!(game.cells[0], None);
    assert_eq!(game.cascades[0].last(), Some(&card));
}

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
    game.cells = [None; 4];
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
