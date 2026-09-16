//! Regression coverage for the tests module.

use super::*;

fn with_tubes(tubes: Vec<Vec<u8>>) -> ColorSort {
    ColorSort {
        tubes,
        ..Default::default()
    }
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
