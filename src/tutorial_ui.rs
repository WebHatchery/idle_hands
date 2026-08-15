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
        GameId::Hangman => [
            "Tap a visible letter button to guess it.",
            "Six wrong guesses end the quiet round.",
            "Tap NEW WORD to begin another word.",
        ],
        GameId::ConnectFour => [
            "Tap a numbered column to drop your red disc.",
            "The cabinet answers with a yellow disc.",
            "Make four in a row; use UNDO or NEW BOARD visibly.",
        ],
        GameId::Checkers => [
            "Tap a red piece, then tap a diagonal destination.",
            "Captures are mandatory; continue tapping for a chained jump.",
            "Reach the far edge to crown a king; use UNDO or NEW BOARD.",
        ],
        GameId::PegSolitaire => [
            "Tap a peg, then tap a two-step destination over a neighbor.",
            "Each jump removes the middle peg; keep clearing the board.",
            "Leave one peg in the center; use UNDO or NEW BOARD.",
        ],
        GameId::MahjongSolitaire => [
            "Tap a free tile, then tap its matching free partner.",
            "A tile is free when one side is open and no tile covers it.",
            "Clear every pair; use UNDO or NEW BOARD with the visible controls.",
        ],
        GameId::Snake => [
            "Tap a visible direction button to move the coil one step.",
            "Eat red food, avoid the walls, and grow toward twenty points.",
            "Use UNDO or NEW BOARD with the visible controls.",
        ],
        GameId::Breakout => [
            "Tap LEFT, STAY, or RIGHT to move the paddle and step the ball.",
            "Bounce the ball into every brick; keep the paddle below it.",
            "Use UNDO or NEW BOARD with the visible controls.",
        ],
        GameId::HigherLower => [
            "Tap HIGHER or LOWER to guess the hidden next card.",
            "Reach ten correct guesses; ties count as correct.",
            "Use UNDO or NEW ROUND with the visible controls.",
        ],
        GameId::KlondikeGolf => [
            "Tap a top card one rank above or below the waste.",
            "Tap STOCK when no column can play; clear every column to win.",
            "Use UNDO or NEW BOARD with the visible controls.",
        ],
        GameId::Blackjack => [
            "Tap HIT to take another card or STAND to hold.",
            "The dealer draws to seventeen; stay at or below twenty-one.",
            "Use UNDO or NEW ROUND with the visible controls.",
        ],
        GameId::SpiderSolitaire => [
            "Tap a same-suit descending run to select it.",
            "Tap a destination column; deal STOCK when every column is filled.",
            "Clear eight suited runs; use UNDO or NEW DEAL visibly.",
        ],
        GameId::DungeonSweeper => [
            "Tap a room to reveal its trap clue.",
            "Use FLAG MODE for traps and follow the clues to the EXIT.",
            "Reach EXIT; use UNDO or NEW DUNGEON with visible controls.",
        ],
        GameId::Potion2048 => [
            "Tap a visible direction to slide the potions.",
            "Matching potions merge; brew a 4096 tile to finish.",
            "Use UNDO or NEW BREW with the visible controls.",
        ],
        GameId::TinyTowerDefence => [
            "Tap an empty lane cell to build, or a tower to upgrade.",
            "Tap START WAVE, then ADVANCE to stop enemies at the quiet gate.",
            "Reach wave 8; use UNDO or NEW TOWER with visible controls.",
        ],
        GameId::OneRoomRoguelike => [
            "Tap a direction to explore; tap STRIKE when an enemy is beside you.",
            "Collect the cache, defeat every enemy, then reach the EXIT.",
            "Use POTION, UNDO, or NEW ROOM with the visible controls.",
        ],
        GameId::DailyDungeon => [
            "Tap a direction to reveal the next quiet room.",
            "Recover three runes, avoid the one-use traps, then reach EXIT.",
            "Use UNDO or NEW DAY with the visible controls.",
        ],
        GameId::DotsBoxes => [
            "Tap a gap between two dots to draw one edge.",
            "Complete a square to claim it and keep your turn.",
            "Claim more squares than the cabinet; use UNDO or NEW BOARD.",
        ],
        GameId::Sokoban => [
            "Tap a direction to walk across the quiet room.",
            "Push every crate onto a marked square; crates cannot be pulled.",
            "Use UNDO or NEW ROOM with the visible controls.",
        ],
        GameId::Mancala => [
            "Tap one of your six stone pits to sow its stones.",
            "A final stone in your store grants another turn.",
            "Gather more stones than the cabinet; use UNDO or NEW BOARD.",
        ],
        GameId::Hanoi => [
            "Tap a peg to select its top disk, then tap a destination peg.",
            "Never place a larger disk on a smaller one.",
            "Move all five disks to the far peg; use UNDO or NEW BOARD.",
        ],
        GameId::NumberMatch => [
            "Tap a number, then tap an adjacent number to pair it.",
            "Equal numbers or pairs totaling ten disappear together.",
            "Clear the grid; use UNDO or NEW BOARD with the visible controls.",
        ],
        GameId::FloodIt => [
            "Tap one of the visible colors to expand the top-left region.",
            "Fill every square before the move counter runs out.",
            "Use UNDO or NEW FIELD with the visible controls.",
        ],
        GameId::ColorSort => [
            "Tap a tube, then tap a matching color or empty tube.",
            "Sort every color into a full, single-color tube.",
            "Use UNDO or NEW BOARD with the visible controls.",
        ],
        GameId::Battleship => [
            "Tap an unknown square to search for the hidden fleet.",
            "Find all five ship squares; hits show as crosses and misses as dots.",
            "Use UNDO or NEW FLEET with the visible controls.",
        ],
        GameId::WordGrid => [
            "Tap letters to build a five-letter guess, then tap SUBMIT.",
            "Green letters are exact; gold letters belong elsewhere in the word.",
            "Use BACKSPACE, UNDO, or NEW WORD with the visible controls.",
        ],
        GameId::PipeLoop => [
            "Tap a pipe tile to rotate it clockwise.",
            "Join the full quiet path to match the connected solution.",
            "Use UNDO or NEW LOOP with the visible controls.",
        ],
    }
}
