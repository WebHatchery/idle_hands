use super::*;

#[test]
fn seeded_standard_deals_repeat_with_ten_columns() {
    let first = SpiderSolitaire::new(42);
    let second = SpiderSolitaire::new(42);
    assert_eq!(first.tableau, second.tableau);
    assert_eq!(first.stock, second.stock);
    assert_eq!(first.tableau.len(), 10);
    assert_eq!(first.stock.len(), 50);
    assert_eq!(first.tableau.iter().map(Vec::len).sum::<usize>(), 54);
}

#[test]
fn same_suit_descending_run_moves_and_undoes() {
    let mut game = SpiderSolitaire::new(42);
    game.tableau[0] = vec![
        Card {
            rank: 2,
            suit: 0,
            face_up: false,
        },
        Card {
            rank: 12,
            suit: 1,
            face_up: true,
        },
        Card {
            rank: 11,
            suit: 1,
            face_up: true,
        },
    ];
    game.tableau[1] = vec![Card {
        rank: 13,
        suit: 1,
        face_up: true,
    }];
    assert!(game.select_column(0, 1));
    assert!(game.move_selected(1));
    assert!(game.tableau[0][0].face_up);
    assert!(game.undo());
    assert_eq!(game.tableau[1].len(), 1);
    assert_eq!(game.moves, 0);
}

#[test]
fn different_suit_run_cannot_be_selected_or_completed() {
    let mut game = SpiderSolitaire::new(42);
    game.tableau[0] = vec![
        Card {
            rank: 13,
            suit: 0,
            face_up: true,
        },
        Card {
            rank: 12,
            suit: 1,
            face_up: true,
        },
    ];
    assert!(!game.select_column(0, 0));
    assert!(game.select_column(0, 1));
    assert!(!is_complete_run(&game.tableau[0]));
}

#[test]
fn stock_deal_and_completed_run_are_tracked() {
    let mut game = SpiderSolitaire::new(42);
    let before = game.tableau.iter().map(Vec::len).collect::<Vec<_>>();
    assert!(game.deal_stock());
    assert_eq!(game.moves, 1);
    assert!(game
        .tableau
        .iter()
        .enumerate()
        .all(|(i, stack)| stack.len() == before[i] + 1));
    game.tableau[0] = (1..=13)
        .rev()
        .map(|rank| Card {
            rank,
            suit: 2,
            face_up: true,
        })
        .collect();
    game.tableau[1] = Vec::new();
    game.selected = Some((0, 0));
    assert!(game.move_selected(1));
    assert_eq!(game.completed, 1);
    assert!(game.tableau[1].is_empty());
}
