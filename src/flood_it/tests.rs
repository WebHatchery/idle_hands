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

#[test]
fn forecasts_and_region_masks_do_not_mutate_the_field() {
    let game = FloodIt::new(9);
    let before = game.cells.clone();
    let color = (game.active_color + 1) % game.color_count();
    let gain = game.preview_gain(color);
    let mask = game.region_mask();

    assert_eq!(
        mask.into_iter().filter(|included| *included).count(),
        game.region_size()
    );
    assert_eq!(game.cells, before);
    assert!(gain < game.cells.len());
}

#[test]
fn three_strong_growth_moves_earn_a_free_surge() {
    let mut game = FloodIt::new(10);
    let side = game.side();
    let colors = game.color_count() as usize;
    for row in 0..side {
        for col in 0..side {
            game.cells[row * side + col] = (col % colors) as u8;
        }
    }
    game.active_color = 0;

    for color in [1, 2, 3] {
        assert!(game.choose(color));
    }
    assert_eq!(game.surges, 1);
    assert_eq!(game.momentum, 0);
    assert_eq!(game.combo, 3);
    assert_eq!(game.points, 48);

    assert!(game.use_surge());
    assert_eq!(game.moves, 3);
    assert_eq!(game.surges, 0);
    assert_eq!(game.combo, 4);
    assert_eq!(game.points, 80);
    assert!(game.undo());
    assert_eq!(game.surges, 1);
    assert_eq!(game.moves, 3);
}

#[test]
fn empty_growth_breaks_chain_and_momentum() {
    let mut game = FloodIt::new(11);
    let side = game.side();
    game.cells = vec![5; side * side];
    for row in 0..side {
        game.cells[row * side] = 0;
        game.cells[row * side + 1] = 1;
    }
    game.active_color = 0;
    assert!(game.choose(1));
    assert!(game.combo > 0);
    assert!(game.choose(4));
    assert_eq!(game.last_gain, 0);
    assert_eq!(game.combo, 0);
    assert_eq!(game.momentum, 0);
}

#[test]
fn full_history_rewinds_multiple_flood_choices() {
    let mut game = FloodIt::new(12);
    let start = game.cells.clone();
    let first = game.hint_color().unwrap();
    game.choose(first);
    let after_one = game.cells.clone();
    let second = game.hint_color().unwrap();
    game.choose(second);

    assert!(game.undo());
    assert_eq!(game.cells, after_one);
    assert!(game.undo());
    assert_eq!(game.cells, start);
    assert!(!game.undo());
}

#[test]
fn legacy_saves_default_to_an_unchained_field() {
    let mut value = serde_json::to_value(FloodIt::new(13)).unwrap();
    let object = value.as_object_mut().unwrap();
    for field in [
        "last_gain",
        "combo",
        "best_combo",
        "points",
        "momentum",
        "surges",
    ] {
        object.remove(field);
    }
    let loaded: FloodIt = serde_json::from_value(value).unwrap();
    assert_eq!(loaded.points, 0);
    assert_eq!(loaded.combo, 0);
    assert_eq!(loaded.surges, 0);
}
