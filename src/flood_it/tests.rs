use super::*;

#[test]
fn choosing_a_color_expands_the_origin_region() {
    let mut game = FloodIt::new(1);
    let old = game.active_color;
    let color = (old + 1) % game.color_count();
    assert!(game.choose(color));
    assert_eq!(game.active_color, color);
    assert_eq!(game.moves, 1);
    assert_eq!(game.cells[0], color);
}

#[test]
fn harder_difficulties_use_larger_fields_and_more_colors() {
    let data = crate::data::GameData::load().unwrap();
    let games: Vec<_> = FloodDifficulty::ALL
        .into_iter()
        .map(|difficulty| FloodIt::new_with_config(1, difficulty, &data.puzzles.flood_it))
        .collect();
    for (game, expected) in games.iter().zip(data.puzzles.flood_it.difficulties.iter()) {
        assert_eq!(game.cells.len(), expected.side * expected.side);
        assert_eq!(game.color_count(), expected.colors);
        assert_eq!(game.move_limit(), expected.move_limit);
    }
    assert!(games[0].color_count() < games[1].color_count());
    assert!(games[1].color_count() < games[2].color_count());
}

#[test]
fn rejects_the_active_color_and_invalid_colors() {
    let mut game = FloodIt::new(2);
    assert!(!game.choose(game.active_color));
    assert!(!game.choose(game.color_count()));
    assert_eq!(game.moves, 0);
}

#[test]
fn undo_restores_the_flooded_region() {
    let mut game = FloodIt::new(3);
    let before = game.cells.clone();
    let color = (game.active_color + 1) % game.color_count();
    game.choose(color);
    assert!(game.undo());
    assert_eq!(game.cells, before);
    assert_eq!(game.moves, 0);
}

#[test]
fn a_uniform_board_is_a_win() {
    let mut game = FloodIt::new(4);
    game.cells = vec![0; game.side() * game.side()];
    game.active_color = 0;
    assert!(game.choose(1));
    assert!(game.won());
}

#[test]
fn move_limit_produces_a_loss() {
    let mut game = FloodIt::new(5);
    for _ in 0..game.move_limit() {
        if game.phase != FloodPhase::Playing {
            break;
        }
        let color = (game.active_color + 1) % game.color_count();
        game.choose(color);
    }
    assert_ne!(game.phase, FloodPhase::Playing);
}

#[test]
fn reset_starts_a_new_board() {
    let mut game = FloodIt::new(6);
    game.choose((game.active_color + 1) % game.color_count());
    game.reset(7);
    assert_eq!(game.seed, FloodIt::new(7).seed);
    assert_eq!(game.moves, 0);
    assert_eq!(game.phase, FloodPhase::Playing);
}

#[test]
fn hint_prefers_the_color_with_the_largest_frontier_gain_without_mutating() {
    let mut game = FloodIt::new(7);
    game.cells = vec![0; game.side() * game.side()];
    game.cells[1] = 1;
    let below_origin = game.side();
    game.cells[below_origin] = 2;
    game.active_color = 0;
    let before = game.cells.clone();

    assert_eq!(game.hint_color(), Some(1));
    assert_eq!(game.cells, before);
    assert_eq!(game.moves, 0);
}

#[test]
fn hint_is_empty_after_flood_it_ends() {
    let mut game = FloodIt::new(8);
    game.phase = FloodPhase::Won;

    assert_eq!(game.hint_color(), None);
}
