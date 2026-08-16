use super::*;

fn with_tubes(tubes: [Vec<u8>; TUBES]) -> ColorSort {
    ColorSort {
        tubes,
        ..Default::default()
    }
}

#[test]
fn seeded_layout_is_repeatable_and_has_six_tubes() {
    let first = ColorSort::new(7);
    assert_eq!(first.tubes, ColorSort::new(7).tubes);
    assert_eq!(first.tubes.len(), TUBES);
    assert_eq!(
        first.tubes.iter().map(Vec::len).sum::<usize>(),
        COLORS as usize * CAPACITY
    );
}

#[test]
fn tap_selects_then_moves_matching_top_run_into_empty_tube() {
    let mut game = with_tubes([
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
    let mut game = with_tubes([vec![0], vec![1], vec![2; 4], vec![3; 4], vec![], vec![]]);
    assert!(game.tap_tube(0));
    assert!(!game.tap_tube(1));
    assert!(!game.tap_tube(2));
    assert_eq!(game.tubes[0], vec![0]);
}

#[test]
fn undo_restores_the_previous_tubes_and_selection() {
    let mut game = with_tubes([vec![0], vec![], vec![1; 4], vec![2; 4], vec![3; 4], vec![]]);
    assert!(game.tap_tube(0));
    assert!(game.tap_tube(1));
    assert!(game.undo());
    assert_eq!(game.tubes[0], vec![0]);
    assert!(game.tubes[1].is_empty());
    assert_eq!(game.moves, 0);
}

#[test]
fn solved_tubes_finish_the_game() {
    let mut game = with_tubes([
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
