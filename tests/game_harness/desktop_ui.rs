//! Desktop-only tap-route checks for every cabinet game.
//!
//! The scan stays inside the logical 1280×720 desktop surface and asks the
//! real game route for actions. It catches a screen whose visible controls no
//! longer produce an input action without coupling tests to drawing pixels.

use idle_hands::testing::{
    clicks_for_game, with_desktop_layout, AppState, GameId, Screen, UiAction,
};
use macroquad::prelude::{vec2, Vec2};
use std::{collections::HashSet, mem::discriminant};

pub(super) fn run(game: GameId) {
    with_desktop_layout(|| {
        let state = AppState {
            screen: Screen::Game(game),
            selected: game.index(),
            tutorial: None,
            ..AppState::default()
        };

        let mut target = None;
        let mut action_kinds = HashSet::new();
        let mut has_recovery_action = false;
        for y in (80..=700).step_by(10) {
            for x in (160..=1260).step_by(10) {
                let actions = desktop_clicks(&state, game, vec2(x as f32 + 5., y as f32 + 5.));
                for action in &actions {
                    if !matches!(action, UiAction::Cabinet) {
                        action_kinds.insert(discriminant(action));
                        has_recovery_action |= super::support::is_recovery_action(action);
                    }
                }
                if !actions.is_empty()
                    && actions
                        .iter()
                        .any(|action| !matches!(action, UiAction::Cabinet))
                {
                    target = Some((x, y));
                }
            }
        }

        assert!(
            target.is_some(),
            "{} desktop surface did not expose a tap target",
            game.title()
        );
        assert!(
            action_kinds.len() >= 2,
            "{} desktop surface exposed fewer than two distinct game actions",
            game.title()
        );
        assert!(
            has_recovery_action,
            "{} desktop surface did not expose a visible recovery action",
            game.title()
        );
    });
}

fn desktop_clicks(state: &AppState, game: GameId, point: Vec2) -> Vec<UiAction> {
    clicks_for_game(state, game, point)
}
