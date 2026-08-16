use super::*;

#[test]
fn hint_returns_the_first_legal_direction() {
    let mut game = Game2048::new(1);
    game.cells = [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(game.hint_direction(), Some(Direction::Down));
}

#[test]
fn hint_returns_none_for_a_blocked_board() {
    let mut game = Game2048::new(1);
    game.cells = [2, 4, 2, 4, 4, 2, 4, 2, 2, 4, 2, 4, 4, 2, 4, 2];
    assert_eq!(game.hint_direction(), None);
}
