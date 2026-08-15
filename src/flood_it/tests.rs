use super::*;

#[test]
fn choosing_a_color_expands_the_origin_region() {
    let mut game = FloodIt::new(1);
    let old = game.active_color;
    let color = (old + 1) % COLORS;
    assert!(game.choose(color));
    assert_eq!(game.active_color, color);
    assert_eq!(game.moves, 1);
    assert_eq!(game.cells[0], color);
}

#[test]
fn rejects_the_active_color_and_invalid_colors() {
    let mut game = FloodIt::new(2);
    assert!(!game.choose(game.active_color));
    assert!(!game.choose(COLORS));
    assert_eq!(game.moves, 0);
}

#[test]
fn undo_restores_the_flooded_region() {
    let mut game = FloodIt::new(3);
    let before = game.cells.clone();
    let color = (game.active_color + 1) % COLORS;
    game.choose(color);
    assert!(game.undo());
    assert_eq!(game.cells, before);
    assert_eq!(game.moves, 0);
}

#[test]
fn a_uniform_board_is_a_win() {
    let mut game = FloodIt::new(4);
    game.cells = vec![0; CELLS];
    game.active_color = 0;
    assert!(game.choose(1));
    assert!(game.won());
}

#[test]
fn move_limit_produces_a_loss() {
    let mut game = FloodIt::new(5);
    for _ in 0..MOVE_LIMIT {
        if game.phase != FloodPhase::Playing {
            break;
        }
        let color = (game.active_color + 1) % COLORS;
        game.choose(color);
    }
    assert_ne!(game.phase, FloodPhase::Playing);
}

#[test]
fn reset_starts_a_new_board() {
    let mut game = FloodIt::new(6);
    game.choose((game.active_color + 1) % COLORS);
    game.reset(7);
    assert_eq!(game.seed, FloodIt::new(7).seed);
    assert_eq!(game.moves, 0);
    assert_eq!(game.phase, FloodPhase::Playing);
}
