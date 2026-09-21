//! Touch-first instruction copy loaded from the authored content catalog.

use crate::{content::GameContent, state::GameId};

pub fn instructions(content: &GameContent, game: GameId) -> &[String; 3] {
    content
        .tutorial(game)
        .expect("validated content has a tutorial for every game")
}
