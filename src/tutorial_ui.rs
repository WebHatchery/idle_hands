//! First-run and replayable touch tutorials shared by all game drawers.

use crate::state::GameId;
use crate::ui::UiAction;
use macroquad::prelude::*;

pub const REPLAY_RECT: Rect = Rect::new(1080., 8., 150., 42.);
const OVERLAY_RECT: Rect = Rect::new(250., 120., 780., 560.);
const CONTINUE_RECT: Rect = Rect::new(800., 600., 180., 52.);

pub fn clicks(p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(CONTINUE_RECT, p) {
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
        Color::new(
            crate::theme::GAME_PANEL.r,
            crate::theme::GAME_PANEL.g,
            crate::theme::GAME_PANEL.b,
            0.96,
        ),
    );
    draw_rectangle_lines(
        REPLAY_RECT.x,
        REPLAY_RECT.y,
        REPLAY_RECT.w,
        REPLAY_RECT.h,
        2.,
        Color::new(
            crate::theme::BRASS.r,
            crate::theme::BRASS.g,
            crate::theme::BRASS.b,
            0.8,
        ),
    );
    crate::ui::draw_text(
        "TUTORIAL",
        REPLAY_RECT.x + 22.,
        REPLAY_RECT.y + 27.,
        16.,
        WHITE,
    );
}
pub fn draw_overlay(game: GameId) {
    draw_rectangle(
        OVERLAY_RECT.x,
        OVERLAY_RECT.y,
        OVERLAY_RECT.w,
        OVERLAY_RECT.h,
        Color::new(
            crate::theme::BACKGROUND_DEEP.r,
            crate::theme::BACKGROUND_DEEP.g,
            crate::theme::BACKGROUND_DEEP.b,
            0.98,
        ),
    );
    draw_rectangle_lines(
        OVERLAY_RECT.x,
        OVERLAY_RECT.y,
        OVERLAY_RECT.w,
        OVERLAY_RECT.h,
        3.,
        Color::new(0.78, 0.58, 0.30, 0.95),
    );
    crate::ui::draw_text("HOW TO PLAY", 315., 225., 38., crate::theme::BRASS);
    crate::ui::draw_text(game.title(), 315., 270., 25., WHITE);
    for (index, line) in instructions(game).iter().enumerate() {
        crate::ui::draw_text(
            line,
            315.,
            330. + index as f32 * 38.,
            19.,
            crate::theme::CREAM,
        );
    }
    draw_rectangle(
        CONTINUE_RECT.x,
        CONTINUE_RECT.y,
        CONTINUE_RECT.w,
        CONTINUE_RECT.h,
        crate::theme::MOSS_DARK,
    );
    crate::ui::draw_text(
        "CONTINUE",
        CONTINUE_RECT.x + 42.,
        CONTINUE_RECT.y + 33.,
        17.,
        WHITE,
    );
}
pub(crate) fn instructions(game: GameId) -> [&'static str; 3] {
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
            "Find its matching card before the board is complete.",
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
            "Six wrong guesses end the round.",
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
            "Tap CLASSIC, WRAP, or GARDEN, then use the visible direction buttons to steer.",
            "WRAP crosses edges; GARDEN adds rocks. Gold food is worth three and the pace rises.",
            "Reach 20; tap PAUSE, UNDO, or NEW BOARD with the visible controls.",
        ],
        GameId::Breakout => [
            "Tap LEFT, STAY, or RIGHT to steer while the ball moves automatically.",
            "Clear three patterned walls. Armored bricks need two or three hits.",
            "You have three balls. Tap LAUNCH after a miss, or PAUSE, UNDO, and NEW BOARD as needed.",
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
            "Tap EXPLORER, DELVER, or PERIL to choose hearts, traps, and required relics.",
            "Reveal rooms for clues; K marks a relic. FLAG MODE and clue taps safely chord rooms.",
            "Collect every K to unlock EXIT; HINT, UNDO, and NEW DUNGEON stay visible.",
        ],
        GameId::Potion2048 => [
            "Tap a visible direction to slide the potions.",
            "Consecutive merging moves multiply score and fill the visible Chain counter.",
            "Complete the Chain to brew a C catalyst that merges with any tier; UNDO and NEW BREW stay visible.",
        ],
        GameId::TinyTowerDefence => [
            "Tap BOLT, FROST, or BURST, then tap an empty lane cell to build.",
            "Bolt hits hard, Frost slows, and Burst splashes nearby lanes; tap a tower to upgrade it.",
            "Tap START WAVE and survive wave 8. PAUSE, UNDO, and NEW TOWER remain visible.",
        ],
        GameId::OneRoomRoguelike => [
            "Tap BLADE, WARDEN, or ALCHEMIST to begin with a different hero talent.",
            "Tap directions to explore; defeat G guards, moving S stalkers, and tough B brutes.",
            "Clear five rooms and tap onto STAIRS; POTION, UNDO, and NEW RUN stay visible.",
        ],
        GameId::DailyDungeon => [
            "Each day is WAYFINDER, FORAGER, or DAREDEVIL with a different trap and SCOUT supply.",
            "Tap SCOUT to reveal adjacent rooms; entering one unseen earns extra bravery score.",
            "Find three runes, use + springs, and reach EXIT; HINT, UNDO, and NEW DAY stay visible.",
        ],
        GameId::DotsBoxes => [
            "Tap a gap between two dots to draw one edge.",
            "Complete a square to keep your turn; a circled ! warns that one edge remains.",
            "HARD avoids gifts and EXPERT minimizes forced chains; use HINT, UNDO, or NEW BOARD.",
        ],
        GameId::Sokoban => [
            "Tap UP, LEFT, DOWN, or RIGHT to walk and push; crates cannot be pulled.",
            "A red X marks a cornered crate; tap UNDO or RESTART to recover.",
            "Beat PAR across six rooms, then tap NEXT ROOM after each clear.",
        ],
        GameId::Mancala => [
            "Tap QUICK, CLASSIC, or GRAND for three, four, or five opening stones per pit.",
            "Tap one of your six pits: E earns another turn and C previews a capture.",
            "GENTLE, SHARP, and EXPERT change cabinet tactics; gather the majority.",
        ],
        GameId::Hanoi => [
            "Tap 3 DISKS, 5 DISKS, or 7 DISKS to choose the tower and its par.",
            "Tap a source peg; green rings are legal destinations and red rings are blocked.",
            "Move the numbered tower to the far peg; use HINT, multi-step UNDO, or RESTART.",
        ],
        GameId::NumberMatch => [
            "Tap NEAR, LINES, or DIAGONAL to choose which clear paths may connect a pair.",
            "Tap equal numbers or numbers totaling ten; green outlines show valid links.",
            "Chain pairs for points; if NO LINKS appears, tap multi-step UNDO or REMIX.",
        ],
        GameId::FloodIt => [
            "Tap a color; its +number forecasts how many cells join the outlined region.",
            "Chain growing moves for points; three strong gains earn a free SURGE.",
            "Fill the field before the limit; tap SURGE, HINT, multi-step UNDO, or NEW FIELD.",
        ],
        GameId::ColorSort => [
            "Tap an unsealed tube; green tubes preview where its full top run can pour.",
            "Letters identify colors; matching pours build a chain and full tubes seal.",
            "Seal every color tube; use HINT, multi-step UNDO, or NEW BOARD.",
        ],
        GameId::Battleship => [
            "Tap unknown water to fire, or tap SONAR then a cell to sweep its 3 × 3 area.",
            "SONAR marks contacts ! and clear water ~; sunk vessels change to gold S marks.",
            "Sink all three ships; chain hits for points and use multi-step UNDO or NEW FLEET.",
        ],
        GameId::WordGrid => [
            "Tap letters to build a five-letter guess, then tap SUBMIT.",
            "Tiles use = for exact, ? for present, and X for absent; the header counts candidates.",
            "HARD binds every clue; use HINT, BACKSPACE, multi-step UNDO, or NEW WORD.",
        ],
        GameId::PipeLoop => [
            "Tap a pipe tile to rotate it clockwise.",
            "Join the full path to match the connected solution.",
            "Use UNDO or NEW LOOP with the visible controls.",
        ],
        GameId::MazeWalk => [
            "Tap a visible direction to move the traveler.",
            "Follow the open path to the glowing exit square.",
            "Use UNDO or NEW MAZE with the visible controls.",
        ],
        GameId::MatchThree => [
            "Tap two adjacent tiles to swap their positions.",
            "Match four for a line arrow; match five or a cross for a burst tile.",
            "Reach the target before moves run out. Tap HINT, UNDO, or NEW BOARD as needed.",
        ],
        GameId::Pyramid => [
            "Tap a king, or tap an exposed card to select it.",
            "Pair two exposed cards whose ranks total thirteen.",
            "Tap STOCK for another card; use UNDO or NEW PYRAMID visibly.",
        ],
        GameId::TriPeaks => [
            "Tap an exposed card one rank above or below the waste.",
            "Clear all three peaks before the stock runs out.",
            "Tap STOCK when no card can play; use UNDO or NEW TRIPEAKS.",
        ],
        GameId::Nim => [
            "Tap a non-empty heap to select it.",
            "Tap TAKE 1, TAKE 2, or TAKE 3 to remove stones.",
            "Take the final stone; use UNDO or NEW BOARD when needed.",
        ],
        GameId::WordLadder => [
            "Tap letters to build a five-letter step, then tap SUBMIT.",
            "Change exactly one letter from the last word.",
            "Use HINT, UNDO, or NEW LADDER with the visible controls.",
        ],
    }
}

#[cfg(test)]
mod tests;
