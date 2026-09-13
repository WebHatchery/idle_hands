//! Regression coverage for the tests module.

use super::*;

fn with_tubes(tubes: Vec<Vec<u8>>) -> ColorSort {
    ColorSort {
        tubes,
        ..Default::default()
    }
}

#[test]
fn seeded_layout_is_repeatable_and_has_six_tubes() {
    let data = crate::data::GameData::load().unwrap();
    let first =
        ColorSort::new_with_config(7, ColorSortDifficulty::Standard, &data.puzzles.color_sort);
    assert_eq!(
        first.tubes,
        ColorSort::new_with_config(7, ColorSortDifficulty::Standard, &data.puzzles.color_sort)
            .tubes
    );
    let standard = &data.puzzles.color_sort.difficulties[0];
    assert_eq!(first.tubes.len(), standard.tubes);
    assert_eq!(
        first.tubes.iter().map(Vec::len).sum::<usize>(),
        standard.colors as usize * data.puzzles.color_sort.capacity
    );
}

#[test]
fn difficulty_boards_add_tubes_and_colors_without_starting_solved() {
    let data = crate::data::GameData::load().unwrap();
    for (difficulty, expected) in ColorSortDifficulty::ALL
        .into_iter()
        .zip(data.puzzles.color_sort.difficulties.iter())
    {
        let game = ColorSort::new_with_config(19, difficulty, &data.puzzles.color_sort);
        assert_eq!(game.tubes.len(), expected.tubes);
        assert_eq!(
            game.tubes.iter().map(Vec::len).sum::<usize>(),
            expected.colors as usize * data.puzzles.color_sort.capacity
        );
        assert!(game
            .tubes
            .iter()
            .any(|tube| tube.windows(2).any(|pair| pair[0] != pair[1])));
        assert!(game.hint_move().is_some());
    }
}

#[test]
fn tap_selects_then_moves_matching_top_run_into_empty_tube() {
    let mut game = with_tubes(vec![
        vec![0, 1, 1],
        vec![],
        vec![2; 4],
        vec![3; 4],
        vec![0; 1],
        vec![0],
    ]);
    assert!(game.tap_tube(0));
    assert_eq!(game.selected, Some(0));
    assert!(game.tap_tube(1));
    assert_eq!(game.tubes[1], vec![1, 1]);
    assert_eq!(game.tubes[0], vec![0]);
    assert_eq!(game.moves, 1);
}

#[test]
fn rejects_full_and_mismatched_destinations() {
    let mut game = with_tubes(vec![
        vec![0],
        vec![1],
        vec![2; 4],
        vec![3; 4],
        vec![],
        vec![],
    ]);
    assert!(game.tap_tube(0));
    assert!(!game.tap_tube(1));
    assert!(!game.tap_tube(2));
    assert_eq!(game.tubes[0], vec![0]);
}

#[test]
fn undo_restores_the_previous_tubes_and_selection() {
    let mut game = with_tubes(vec![
        vec![0],
        vec![],
        vec![1; 4],
        vec![2; 4],
        vec![3; 4],
        vec![],
    ]);
    assert!(game.tap_tube(0));
    assert!(game.tap_tube(1));
    assert!(game.undo());
    assert_eq!(game.tubes[0], vec![0]);
    assert!(game.tubes[1].is_empty());
    assert_eq!(game.moves, 0);
}

#[test]
fn solved_tubes_finish_the_game() {
    let mut game = with_tubes(vec![
        vec![0; 4],
        vec![1; 4],
        vec![2; 4],
        vec![3; 3],
        vec![3],
        vec![],
    ]);
    assert!(game.tap_tube(4));
    assert!(game.tap_tube(3));
    assert!(game.won());
}

#[test]
fn reset_changes_seeded_board_and_finished_games_stop() {
    let mut game = ColorSort::new(11);
    let old = game.tubes.clone();
    game.reset(12);
    assert_ne!(old, game.tubes);
    game.phase = ColorSortPhase::Won;
    assert!(!game.tap_tube(0));
}

#[test]
fn hint_returns_a_legal_progress_move_without_mutating_the_tubes() {
    let game = ColorSort::new(13);
    let before = game.tubes.clone();

    let Some((source, destination)) = game.hint_move() else {
        panic!("seeded Color Sort should have a legal move");
    };
    assert_ne!(source, destination);
    assert_eq!(game.tubes, before);
    assert_eq!(game.selected, None);
    assert_eq!(game.moves, 0);
}

#[test]
fn hint_is_empty_after_color_sort_ends() {
    let mut game = ColorSort::new(14);
    game.phase = ColorSortPhase::Won;

    assert_eq!(game.hint_move(), None);
}

#[test]
fn pour_preview_reports_run_capacity_and_sealing() {
    let game = with_tubes(vec![
        vec![0, 1, 1],
        vec![1, 1],
        vec![2; 4],
        vec![3; 4],
        vec![0],
        vec![],
    ]);
    assert_eq!(
        game.pour_preview(0, 1),
        Some(PourPreview {
            count: 2,
            stacks_match: true,
            completes_tube: true,
        })
    );
    assert_eq!(game.pour_preview(0, 5).unwrap().count, 2);
}

#[test]
fn completed_tubes_are_sealed_as_sources() {
    let mut game = with_tubes(vec![
        vec![0; 4],
        vec![1, 2],
        vec![1, 1],
        vec![2, 2],
        vec![],
        vec![],
    ]);
    assert!(game.is_complete_tube(0));
    assert_eq!(game.completed_tubes(), 1);
    assert!(!game.tap_tube(0));
    assert_eq!(game.selected, None);
}

#[test]
fn matching_pours_build_points_and_full_undo_history() {
    let mut game = with_tubes(vec![
        vec![0, 1, 1],
        vec![],
        vec![1, 1],
        vec![2; 4],
        vec![3; 4],
        vec![0],
    ]);
    game.tap_tube(0);
    game.tap_tube(1);
    assert_eq!(game.combo, 1);
    assert_eq!(game.points, 2);
    game.tap_tube(2);
    game.tap_tube(1);
    assert_eq!(game.combo, 2);
    assert_eq!(game.points, 16);
    assert!(game.is_complete_tube(1));

    assert!(game.undo());
    assert_eq!(game.points, 2);
    assert!(game.undo());
    assert_eq!(game.points, 0);
    assert!(!game.undo());
}

#[test]
fn legacy_saves_default_to_an_unchained_sort() {
    let mut value = serde_json::to_value(ColorSort::new(15)).unwrap();
    let object = value.as_object_mut().unwrap();
    for field in ["last_poured", "combo", "best_combo", "points"] {
        object.remove(field);
    }
    let loaded: ColorSort = serde_json::from_value(value).unwrap();
    assert_eq!(loaded.last_poured, 0);
    assert_eq!(loaded.combo, 0);
    assert_eq!(loaded.points, 0);
}
