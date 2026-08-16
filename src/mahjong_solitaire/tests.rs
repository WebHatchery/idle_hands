use super::*;

#[test]
fn seeded_layouts_repeat_with_eighteen_pairs() {
    let first = MahjongSolitaire::new(7);
    let second = MahjongSolitaire::new(7);
    assert_eq!(first.tiles, second.tiles);
    assert_eq!(first.tiles.len(), TILE_COUNT);
    assert_eq!(first.tiles.iter().filter(|tile| !tile.removed).count(), 36);
}

#[test]
fn matching_available_tiles_remove_as_a_pair_and_undo() {
    let mut game = MahjongSolitaire::new(1);
    let (first, second) = (0..game.tiles.len())
        .filter(|&index| game.available(index))
        .find_map(|first| {
            (first + 1..game.tiles.len())
                .find(|&index| {
                    game.available(index) && game.tiles[index].kind == game.tiles[first].kind
                })
                .map(|second| (first, second))
        })
        .unwrap();
    assert!(game.tap(first));
    assert!(game.tap(second));
    assert!(game.tiles[first].removed);
    assert!(game.tiles[second].removed);
    assert!(game.undo());
    assert!(!game.tiles[first].removed);
}

#[test]
fn a_board_without_remaining_pairs_becomes_stuck() {
    let mut game = MahjongSolitaire::new(1);
    for tile in &mut game.tiles {
        tile.removed = true;
    }
    game.tiles[0].removed = false;
    game.tiles[1].removed = false;
    game.tiles[0].kind = 1;
    game.tiles[1].kind = 2;
    game.resolve();
    assert_eq!(game.status, MahjongStatus::Stuck);
}

#[test]
fn hint_pair_finds_free_matching_tiles_without_mutating() {
    let game = MahjongSolitaire::new(1);
    let before = game.tiles.clone();
    let (first, second) = game.hint_pair().unwrap();
    assert!(game.available(first));
    assert!(game.available(second));
    assert_eq!(game.tiles[first].kind, game.tiles[second].kind);
    assert_eq!(game.tiles, before);
}
