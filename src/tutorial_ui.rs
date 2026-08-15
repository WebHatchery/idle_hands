//! First-run and replayable touch tutorials shared by all game drawers.

use crate::state::GameId;
use crate::ui::UiAction;
use macroquad::prelude::*;

pub const REPLAY_RECT: Rect = Rect::new(220., 20., 150., 50.);
const CONTINUE_RECT: Rect = Rect::new(900., 565., 180., 52.);

pub fn clicks(p: Vec2) -> Vec<UiAction> {
    if CONTINUE_RECT.contains(p) {
        vec![UiAction::TutorialContinue]
    } else {
        vec![]
    }
}
pub fn draw_replay_button() {
    draw_rectangle(
        REPLAY_RECT.x,
        REPLAY_RECT.y,
        REPLAY_RECT.w,
        REPLAY_RECT.h,
        Color::new(0.16, 0.11, 0.25, 0.96),
    );
    draw_rectangle_lines(
        REPLAY_RECT.x,
        REPLAY_RECT.y,
        REPLAY_RECT.w,
        REPLAY_RECT.h,
        2.,
        Color::new(0.45, 0.38, 0.65, 0.8),
    );
    draw_text(
        "TUTORIAL",
        REPLAY_RECT.x + 22.,
        REPLAY_RECT.y + 31.,
        16.,
        WHITE,
    );
}
pub fn draw_overlay(game: GameId) {
    draw_rectangle(250., 150., 780., 485., Color::new(0.07, 0.045, 0.13, 0.98));
    draw_rectangle_lines(
        250.,
        150.,
        780.,
        485.,
        3.,
        Color::new(0.78, 0.58, 0.30, 0.95),
    );
    draw_text(
        "HOW TO PLAY",
        315.,
        225.,
        38.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    draw_text(game.title(), 315., 270., 25., WHITE);
    for (index, line) in instructions(game).iter().enumerate() {
        draw_text(
            line,
            315.,
            330. + index as f32 * 38.,
            19.,
            Color::new(0.78, 0.73, 0.86, 1.),
        );
    }
    draw_rectangle(
        CONTINUE_RECT.x,
        CONTINUE_RECT.y,
        CONTINUE_RECT.w,
        CONTINUE_RECT.h,
        Color::new(0.25, 0.45, 0.34, 1.),
    );
    draw_text(
        "CONTINUE",
        CONTINUE_RECT.x + 42.,
        CONTINUE_RECT.y + 33.,
        17.,
        WHITE,
    );
}
fn instructions(game: GameId) -> [&'static str; 3] {
    match game {
        GameId::Game2048 => [
            "Swipe the board to move tiles.",
            "You can also tap a visible direction arrow.",
            "Tap TUTORIAL above to see this again.",
        ],
        GameId::Minesweeper => [
            "Tap a hidden square to reveal it.",
            "Tap REVEAL MODE / FLAG MODE to mark safely.",
            "Tap a revealed number to use the visible chord action.",
        ],
        GameId::Sudoku => [
            "Tap a cell to select it.",
            "Tap a number on the visible number pad to place it.",
            "Tap ERASE or PENCIL when you need another mark.",
        ],
        GameId::Nonogram => [
            "Tap a cell to fill or cross it.",
            "Tap FILL / CROSS to change the visible mode.",
            "Drag across a row or column for a straight stroke.",
        ],
        GameId::Solitaire => [
            "Tap a face-up card to select it.",
            "Tap a legal tableau or foundation destination.",
            "Tap STOCK to deal; use the visible UNDO button.",
        ],
        GameId::FreeCell => [
            "Tap a card to select it.",
            "Tap a cascade, free cell, or foundation destination.",
            "Every card stays visible while you build sequences.",
        ],
        GameId::Yahtzee => [
            "Tap ROLL DICE for the first roll.",
            "Tap dice to hold them, then tap ROLL AGAIN.",
            "Tap a score row to record the visible preview.",
        ],
        GameId::Reversi => [
            "Tap a glowing square to place a disc.",
            "The captured line flips visibly after your move.",
            "Tap PASS only when no legal square remains.",
        ],
        GameId::LightsOut => [
            "Tap a light to toggle its cross.",
            "Turn every light off to complete the board.",
            "Use UNDO or NEW BOARD when you need it.",
        ],
        GameId::TicTacToe => [
            "Tap an empty square to place your X.",
            "The cabinet answers with O after your move.",
            "Use UNDO or NEW BOARD when you need it.",
        ],
        GameId::MemoryPairs => [
            "Tap a card to turn it face up.",
            "Find its matching card before the board goes quiet.",
            "Use UNDO or NEW BOARD whenever you need it.",
        ],
        GameId::SlidingPuzzle => [
            "Tap a tile beside the empty space.",
            "Put every numbered tile back in order.",
            "Use UNDO or NEW BOARD whenever you need it.",
        ],
        GameId::Mastermind => [
            "Tap four colors to build a guess.",
            "GUESS shows exact and partial matches.",
            "Use CLEAR, UNDO, or NEW BOARD visibly below.",
        ],
        GameId::Spider => [
            "Tap a face-up descending run to select it.",
            "Tap a destination column to move the run.",
            "Tap STOCK to deal one card to every column.",
        ],
        GameId::WordSearch => [
            "Tap the first letter of a hidden word.",
            "Tap its final letter in a straight line.",
            "Use CLEAR or NEW BOARD with the visible controls.",
        ],
    }
}
