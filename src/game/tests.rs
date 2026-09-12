//! Regression coverage for the tests module.

use crate::game_input::card_drag_actions;
use crate::state::{GameId, Screen};
use crate::ui;

use super::*;

#[test]
fn portrait_card_drag_dispatches_source_and_destination_actions() {
    let solitaire = crate::state::AppState {
        screen: Screen::Game(GameId::Solitaire),
        ..Default::default()
    };
    let actions = card_drag_actions(&solitaire, vec2(6., 205.), vec2(55., 205.), true, false);
    assert!(matches!(
        actions.as_slice(),
        [
            ui::UiAction::SolitaireTableau(0, 0),
            ui::UiAction::SolitaireTableau(1, 0),
        ]
    ));

    let freecell = crate::state::AppState {
        screen: Screen::Game(GameId::FreeCell),
        ..Default::default()
    };
    let actions = card_drag_actions(&freecell, vec2(6., 205.), vec2(55., 205.), true, false);
    assert!(matches!(
        actions.as_slice(),
        [
            ui::UiAction::FreeCellCascade(0, 0),
            ui::UiAction::FreeCellCascade(1, 0),
        ]
    ));
}

#[test]
fn card_drag_dispatch_ignores_non_card_screens() {
    let state = crate::state::AppState::default();
    assert!(card_drag_actions(&state, vec2(6., 205.), vec2(55., 205.), true, false).is_empty());
}

#[test]
fn new_game_actions_require_confirmation_but_existing_restart_does_not() {
    assert!(!ui::UiAction::Open(GameId::Solitaire.index()).starts_new_round());
    assert!(!ui::UiAction::ContinueGame.starts_new_round());
    assert!(game_restart::requires_new_confirmation(
        ui::UiAction::MatchThreeNew
    ));
    assert!(game_restart::requires_new_confirmation(
        ui::UiAction::NimNew
    ));
    assert!(game_restart::requires_new_confirmation(
        ui::UiAction::WordLadderNew
    ));
    assert!(game_restart::requires_new_confirmation(
        ui::UiAction::MineRestart
    ));
    assert!(game_restart::requires_new_confirmation(
        ui::UiAction::MinePreset(crate::minesweeper::MinePreset::Beginner)
    ));
    assert!(game_restart::requires_new_confirmation(
        ui::UiAction::SudokuDifficulty(crate::sudoku::SudokuDifficulty::Easy)
    ));
    assert!(game_restart::requires_new_confirmation(
        ui::UiAction::NonogramPreset(crate::nonogram::NonogramPreset::Small)
    ));
    assert!(!game_restart::requires_new_confirmation(
        ui::UiAction::MatchThreeHint
    ));
    assert!(!game_restart::requires_new_confirmation(
        ui::UiAction::Restart
    ));
}

#[test]
fn depth_mode_controls_confirm_before_discarding_an_active_round() {
    let actions = [
        ui::UiAction::PyramidDrawRule(crate::pyramid::PyramidDraw::Three),
        ui::UiAction::TriPeaksRule(crate::tri_peaks::TriPeaksRule::Wrap),
        ui::UiAction::NimRule(crate::nim::NimRule::Misere),
        ui::UiAction::WordGridMode(crate::word_grid::WordGridMode::Hard),
        ui::UiAction::WordLadderMode(crate::word_ladder::LadderMode::Scenic),
        ui::UiAction::PipePattern(crate::pipe_loop::PipePattern::Trunk),
        ui::UiAction::MazeMode(crate::maze_walk::MazeMode::Fog),
    ];
    for action in actions {
        assert!(
            action.starts_new_round(),
            "{action:?} must register a new round"
        );
        assert!(
            game_restart::requires_new_confirmation(action),
            "{action:?} must confirm before replacing progress"
        );
    }
}
