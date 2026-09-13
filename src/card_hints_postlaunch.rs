//! Hint copy and recommendation logic for the post-launch game drawers.

use crate::state::AppState;

#[path = "card_hints_postlaunch/logic.rs"]
mod logic;

pub use logic::{
    battleship, blackjack, breakout, color_sort, daily_dungeon, dots_boxes, dungeon_sweeper,
    flood_it, hanoi, higher_lower, mancala, match_three, maze_walk, number_match,
    one_room_roguelike, pipe_loop, potion_2048, snake, sokoban, tiny_tower_defence, word_grid,
};

fn direction_label(direction: crate::domain::Direction) -> &'static str {
    match direction {
        crate::domain::Direction::Up => "UP",
        crate::domain::Direction::Right => "RIGHT",
        crate::domain::Direction::Down => "DOWN",
        crate::domain::Direction::Left => "LEFT",
    }
}

fn battleship_cell_label(index: usize) -> String {
    format!(
        "{}{}",
        (b'A' + (index % crate::battleship::SIDE) as u8) as char,
        index / crate::battleship::SIDE + 1
    )
}

fn flood_color_label(color: u8) -> &'static str {
    [
        "RED", "AMBER", "GREEN", "BLUE", "VIOLET", "PINK", "MINT", "ORANGE",
    ][color as usize % 8]
}

fn cell_label(index: usize) -> String {
    format!(
        "{}{}",
        (b'A' + (index % crate::number_match::SIDE) as u8) as char,
        index / crate::number_match::SIDE + 1
    )
}

fn daily_direction_label(direction: crate::domain::Direction) -> &'static str {
    match direction {
        crate::domain::Direction::Up => "UP",
        crate::domain::Direction::Left => "LEFT",
        crate::domain::Direction::Down => "DOWN",
        crate::domain::Direction::Right => "RIGHT",
    }
}

fn rogue_direction_label(direction: crate::domain::Direction) -> &'static str {
    daily_direction_label(direction)
}

fn potion_direction_label(direction: crate::domain::Direction) -> &'static str {
    daily_direction_label(direction)
}

fn blackjack_hint_label(action: crate::blackjack::BlackjackHint) -> &'static str {
    match action {
        crate::blackjack::BlackjackHint::Hit => "HIT",
        crate::blackjack::BlackjackHint::Stand => "STAND",
    }
}

fn guess_label(guess: crate::higher_lower::Guess) -> &'static str {
    match guess {
        crate::higher_lower::Guess::Higher => "HIGHER",
        crate::higher_lower::Guess::Lower => "LOWER",
    }
}

fn movement_label(movement: crate::breakout::PaddleMove) -> &'static str {
    match movement {
        crate::breakout::PaddleMove::Left => "LEFT",
        crate::breakout::PaddleMove::Stay => "STAY",
        crate::breakout::PaddleMove::Right => "RIGHT",
    }
}

fn snake_direction_label(direction: crate::snake::SnakeDirection) -> &'static str {
    match direction {
        crate::snake::SnakeDirection::Up => "UP",
        crate::snake::SnakeDirection::Right => "RIGHT",
        crate::snake::SnakeDirection::Down => "DOWN",
        crate::snake::SnakeDirection::Left => "LEFT",
    }
}
