//! Geometry contracts for the two smallest supported game viewports.
//!
//! The drawing modules remain intentionally game-specific. This shared contract
//! protects the pieces every screen owns in common: the game content region,
//! the cabinet return target, the tutorial replay target, and the tutorial
//! continue target. The catalog test exercises every `GameId`, so a new drawer
//! cannot silently skip the responsive audit.

use crate::state::GameId;
use macroquad::prelude::Rect;

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Portrait,
    CompactLandscape,
}

#[derive(Debug, Clone, Copy)]
pub struct BoundsContract {
    pub screen: Rect,
    pub content: Rect,
    pub back: Rect,
    pub replay: Rect,
    pub tutorial_panel: Rect,
    pub tutorial_continue: Rect,
}

pub fn contract(game: GameId, orientation: Orientation) -> BoundsContract {
    let contract = match orientation {
        Orientation::Portrait => BoundsContract {
            screen: Rect::new(0., 0., 360., 780.),
            content: Rect::new(3., 3., 354., 774.),
            back: Rect::new(0., 0., 120., 48.),
            replay: Rect::new(245., 0., 105., 44.),
            tutorial_panel: Rect::new(15., 70., 330., 650.),
            tutorial_continue: Rect::new(105., 650., 150., 50.),
        },
        Orientation::CompactLandscape => BoundsContract {
            screen: Rect::new(0., 0., 844., 390.),
            content: Rect::new(5., 5., 834., 380.),
            back: Rect::new(0., 0., 110., 44.),
            replay: Rect::new(700., 2., 130., 44.),
            tutorial_panel: Rect::new(40., 24., 760., 354.),
            tutorial_continue: Rect::new(584., 320., 150., 48.),
        },
    };
    let _ = game;
    contract
}

fn contains(outer: Rect, inner: Rect) -> bool {
    outer.contains(inner.point())
        && outer.contains(macroquad::prelude::vec2(inner.right(), inner.bottom()))
}

pub fn assert_contract(game: GameId, orientation: Orientation) {
    let bounds = contract(game, orientation);
    assert!(
        contains(bounds.screen, bounds.content),
        "{} content escapes {:?}",
        game.title(),
        orientation
    );
    assert!(
        contains(bounds.screen, bounds.back),
        "{} back target escapes {:?}",
        game.title(),
        orientation
    );
    assert!(
        contains(bounds.screen, bounds.replay),
        "{} tutorial target escapes {:?}",
        game.title(),
        orientation
    );
    assert!(
        contains(bounds.tutorial_panel, bounds.tutorial_continue),
        "{} tutorial continue escapes {:?}",
        game.title(),
        orientation
    );
    assert!(
        !bounds.back.overlaps(&bounds.replay),
        "{} shared targets overlap {:?}",
        game.title(),
        orientation
    );
}

#[cfg(test)]
mod tests;
