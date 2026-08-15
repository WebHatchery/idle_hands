//! Medium landscape layouts for short touch screens.

use crate::{
    cosmetics,
    data::GameData,
    palette_ui,
    state::{AppState, Direction, GameId},
    ui::UiAction,
};
use macroquad::prelude::*;

pub const WIDTH: f32 = 844.;
pub const HEIGHT: f32 = 390.;

fn panel(rect: Rect, fill: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.,
        Color::new(0.45, 0.38, 0.65, 0.65),
    );
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(value, x, y, size, color);
}

fn cabinet_rect(index: usize) -> Rect {
    Rect::new(
        8. + (index % 6) as f32 * 140.,
        38. + (index / 6) as f32 * 44.,
        132.,
        44.,
    )
}

pub fn draw_cabinet(state: &AppState, _data: &GameData, loaded: usize) {
    let accent = cosmetics::cabinet_accent(state.cabinet_decoration);
    text("IDLE HANDS", 12., 30., 25., accent);
    crate::cabinet_art::draw_header_motif(812., 28., 13., accent);
    crate::cabinet_art::draw_shelves(8., 54., 828., 260., accent);
    text(
        "Quiet games for a wider pause",
        190.,
        28.,
        13.,
        Color::new(0.72, 0.68, 0.82, 1.),
    );
    for (index, game) in GameId::ALL.iter().enumerate() {
        let rect = cabinet_rect(index);
        panel(rect, Color::new(0.17, 0.12, 0.27, 1.));
        text(
            game.title(),
            rect.x + 10.,
            rect.y + 12.,
            9.,
            Color::new(0.98, 0.82, 0.42, 1.),
        );
        text(
            cabinet_status(state, *game),
            rect.x + 10.,
            rect.y + 24.,
            7.,
            Color::new(0.98, 0.75, 0.30, 1.),
        );
        text(
            game.subtitle(),
            rect.x + 10.,
            rect.y + 35.,
            6.,
            Color::new(0.69, 0.65, 0.78, 1.),
        );
        draw_circle(
            rect.right() - 20.,
            rect.y + 10.,
            6.,
            cosmetics::cabinet_accent(state.cabinet_decoration),
        );
        text(
            &(index + 1).to_string(),
            rect.right() - 24.,
            rect.y + 13.,
            6.,
            Color::new(0.08, 0.05, 0.12, 1.),
        );
    }
    text(
        &format!("{} stamps  •  {} textures", state.stamps, loaded),
        12.,
        344.,
        11.,
        Color::new(0.52, 0.48, 0.64, 1.),
    );
    for (rect, label) in [
        (Rect::new(450., 330., 112., 42.), "HELP"),
        (Rect::new(570., 330., 112., 42.), "RECORDS"),
        (Rect::new(690., 330., 140., 42.), "SETTINGS"),
    ] {
        panel(rect, Color::new(0.12, 0.08, 0.20, 1.));
        text(label, rect.x + 15., rect.y + 27., 11., WHITE);
    }
}

pub fn cabinet_clicks(p: Vec2) -> Vec<UiAction> {
    for index in 0..GameId::ALL.len() {
        if cabinet_rect(index).contains(p) {
            return vec![UiAction::Open(index)];
        }
    }
    for (rect, action) in [
        (Rect::new(450., 330., 112., 42.), UiAction::Help),
        (Rect::new(570., 330., 112., 42.), UiAction::Records),
        (Rect::new(690., 330., 140., 42.), UiAction::Settings),
    ] {
        if rect.contains(p) {
            return vec![action];
        }
    }
    vec![]
}

pub fn draw_2048(state: &AppState) {
    let game = &state.game;
    text("‹ CABINET", 12., 26., 13., Color::new(0.78, 0.70, 0.92, 1.));
    text("2048", 12., 58., 27., Color::new(0.98, 0.83, 0.45, 1.));
    text(
        &format!("Score {}  •  Best {}", game.score, game.best),
        120.,
        51.,
        12.,
        WHITE,
    );
    let board = Rect::new(12., 65., 320., 320.);
    panel(board, Color::new(0.10, 0.07, 0.16, 1.));
    for index in 0..16 {
        let rect = Rect::new(
            board.x + 8. + (index % 4) as f32 * 78.,
            board.y + 8. + (index / 4) as f32 * 78.,
            72.,
            72.,
        );
        let value = game.cells[index];
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            palette_ui::tile_color(value, state.board_theme),
        );
        if value > 0 {
            let label = value.to_string();
            let size = if value < 100 { 25. } else { 19. };
            let width = measure_text(&label, None, size as u16, 1.).width;
            text(
                &label,
                rect.x + (rect.w - width) / 2.,
                rect.y + 45.,
                size,
                WHITE,
            );
        }
    }
    for (index, direction) in [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ]
    .iter()
    .enumerate()
    {
        let rect = Rect::new(
            380. + (index % 2) as f32 * 82.,
            145. + (index / 2) as f32 * 58.,
            74.,
            46.,
        );
        panel(rect, Color::new(0.18, 0.12, 0.28, 1.));
        text(
            ["UP", "LEFT", "DOWN", "RIGHT"][index],
            rect.x + 12.,
            rect.y + 29.,
            11.,
            Color::new(0.98, 0.83, 0.45, 1.),
        );
        let _ = direction;
    }
    panel(
        Rect::new(590., 145., 110., 46.),
        Color::new(0.18, 0.12, 0.28, 1.),
    );
    text("UNDO", 625., 174., 12., WHITE);
    panel(
        Rect::new(715., 145., 115., 46.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("NEW GAME", 738., 174., 11., WHITE);
    text(
        "Swipe the board or tap a direction.",
        380.,
        285.,
        13.,
        Color::new(0.70, 0.64, 0.78, 1.),
    );
    if state.confirm_restart {
        panel(
            Rect::new(375., 215., 300., 120.),
            Color::new(0.16, 0.09, 0.20, 1.),
        );
        text("Start a new board?", 435., 250., 18., WHITE);
        panel(
            Rect::new(395., 270., 115., 42.),
            Color::new(0.25, 0.16, 0.32, 1.),
        );
        text("CANCEL", 425., 297., 12., WHITE);
        panel(
            Rect::new(535., 270., 115., 42.),
            Color::new(0.45, 0.22, 0.25, 1.),
        );
        text("START", 572., 297., 12., WHITE);
    }
}

pub fn game2048_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(8., 5., 100., 30.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    if state.confirm_restart {
        if Rect::new(395., 270., 115., 42.).contains(p) {
            return vec![UiAction::Cancel];
        }
        if Rect::new(535., 270., 115., 42.).contains(p) {
            return vec![UiAction::ConfirmRestart];
        }
        return vec![];
    }
    if Rect::new(590., 145., 110., 46.).contains(p) && state.game.can_undo() {
        return vec![UiAction::Undo];
    }
    if Rect::new(715., 145., 115., 46.).contains(p) {
        return vec![UiAction::Restart];
    }
    for (index, direction) in [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ]
    .iter()
    .enumerate()
    {
        if Rect::new(
            380. + (index % 2) as f32 * 82.,
            145. + (index / 2) as f32 * 58.,
            74.,
            46.,
        )
        .contains(p)
        {
            return vec![UiAction::Move(*direction)];
        }
    }
    vec![]
}

const TUTORIAL_RECT: Rect = Rect::new(88., 48., 668., 292.);
const CONTINUE_RECT: Rect = Rect::new(584., 272., 150., 48.);
const REPLAY_RECT: Rect = Rect::new(220., 20., 150., 50.);

pub fn tutorial_clicks(p: Vec2) -> Vec<UiAction> {
    if CONTINUE_RECT.contains(p) {
        vec![UiAction::TutorialContinue]
    } else {
        vec![]
    }
}

pub fn replay_clicks(p: Vec2) -> bool {
    REPLAY_RECT.contains(p)
}

pub fn draw_replay_button() {
    panel(REPLAY_RECT, Color::new(0.16, 0.11, 0.25, 0.96));
    text(
        "TUTORIAL",
        REPLAY_RECT.x + 22.,
        REPLAY_RECT.y + 31.,
        16.,
        WHITE,
    );
}

pub fn draw_tutorial(game: GameId) {
    panel(TUTORIAL_RECT, Color::new(0.07, 0.045, 0.13, 0.98));
    text(
        "HOW TO PLAY",
        122.,
        93.,
        30.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    text(game.title(), 122., 130., 21., WHITE);
    for (index, line) in tutorial_lines(game).iter().enumerate() {
        text(
            line,
            122.,
            174. + index as f32 * 31.,
            16.,
            Color::new(0.78, 0.73, 0.86, 1.),
        );
    }
    panel(CONTINUE_RECT, Color::new(0.25, 0.45, 0.34, 1.));
    text("CONTINUE", 618., 302., 15., WHITE);
}

fn tutorial_lines(game: GameId) -> [&'static str; 3] {
    match game {
        GameId::Game2048 => [
            "Swipe the board or tap a visible direction.",
            "Use UNDO when you want to step back.",
            "Tap TUTORIAL above to see this again.",
        ],
        GameId::Minesweeper => [
            "Tap hidden squares to reveal them.",
            "Use REVEAL MODE / FLAG MODE to mark squares.",
            "Tap a number to use the visible chord action.",
        ],
        GameId::Sudoku => [
            "Tap a cell, then tap a number on the pad.",
            "Use ERASE or NOTES when you need another mark.",
            "Use UNDO to step back through your entries.",
        ],
        GameId::Nonogram => [
            "Tap a cell to fill or cross it.",
            "Use FILL / CROSS to change the visible mode.",
            "Drag across a row or column for a straight stroke.",
        ],
        GameId::Solitaire => [
            "Tap a face-up card to select it.",
            "Tap a legal tableau or foundation destination.",
            "Tap STOCK to deal; use the visible UNDO button.",
        ],
        GameId::FreeCell => [
            "Tap a card, then a legal destination.",
            "Every card stays visible while you build sequences.",
            "Use UNDO when you want to step back.",
        ],
        GameId::Yahtzee => [
            "Tap ROLL DICE for the first roll.",
            "Tap dice to hold them, then ROLL AGAIN.",
            "Tap a score row to record the preview.",
        ],
        GameId::Reversi => [
            "Tap a glowing square to place a disc.",
            "The captured line flips after your move.",
            "Tap PASS only when no legal square remains.",
        ],
        GameId::LightsOut => [
            "Tap a light to toggle its cross.",
            "Turn every light off to complete the board.",
            "Use UNDO or NEW BOARD whenever you need it.",
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
            "Tap HIT for another card or STAND to hold.",
            "Beat the dealer without going over twenty-one.",
            "Use UNDO or NEW ROUND with the visible controls.",
        ],
        GameId::SpiderSolitaire => [
            "Tap a suited descending run, then tap its destination.",
            "Tap STOCK to deal one card to every column.",
            "Clear eight runs; use UNDO or NEW DEAL visibly.",
        ],
        GameId::DungeonSweeper => [
            "Tap a room to reveal its clue or use FLAG MODE.",
            "Tap a revealed clue after flags match to chord nearby rooms.",
            "Use UNDO or NEW DUNGEON with visible controls.",
        ],
        GameId::Potion2048 => [
            "Tap a visible direction to slide the potions.",
            "Merge matching potions until 4096 appears.",
            "Use UNDO or NEW BREW with visible controls.",
        ],
        GameId::TinyTowerDefence => [
            "Tap empty lane cells to build and towers to upgrade.",
            "START WAVE spawns enemies; ADVANCE lets your towers fire.",
            "Reach wave 8; use UNDO or NEW TOWER with visible controls.",
        ],
        GameId::OneRoomRoguelike => [
            "Tap directions to explore; STRIKE attacks an adjacent enemy.",
            "Collect the cache, clear the room, then reach the EXIT.",
            "Use POTION, UNDO, or NEW ROOM with visible controls.",
        ],
        GameId::DailyDungeon => [
            "Tap directions to reveal rooms and recover the three runes.",
            "Traps are one-use; reach EXIT after the runes are gathered.",
            "Use UNDO or NEW DAY with visible controls.",
        ],
        GameId::DotsBoxes => [
            "Tap a gap between dots to draw an edge.",
            "Complete a square to keep your turn and claim it.",
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
            "Clear the grid; use UNDO or NEW BOARD with visible controls.",
        ],
        GameId::FloodIt => [
            "Tap one of the visible colors to expand the top-left region.",
            "Fill every square before the move counter runs out.",
            "Use UNDO or NEW FIELD with visible controls.",
        ],
        GameId::ColorSort => [
            "Tap a tube, then tap a matching color or empty tube.",
            "Sort every color into a full, single-color tube.",
            "Use UNDO or NEW BOARD with visible controls.",
        ],
        GameId::Battleship => [
            "Tap an unknown square to search for the hidden fleet.",
            "Find all five ship squares; hits show as crosses and misses as dots.",
            "Use UNDO or NEW FLEET with visible controls.",
        ],
        GameId::WordGrid => [
            "Tap letters to build a five-letter guess, then tap SUBMIT.",
            "Green letters are exact; gold letters belong elsewhere in the word.",
            "Use BACKSPACE, UNDO, or NEW WORD with visible controls.",
        ],
        GameId::PipeLoop => [
            "Tap a pipe tile to rotate it clockwise.",
            "Join the full quiet path to match the connected solution.",
            "Use UNDO or NEW LOOP with visible controls.",
        ],
        GameId::MazeWalk => [
            "Tap a visible direction to move the quiet traveler.",
            "Follow the open path to the glowing exit square.",
            "Use UNDO or NEW MAZE with visible controls.",
        ],
        GameId::MatchThree => [
            "Tap two adjacent tiles to swap their positions.",
            "Clear groups of three or more matching colors to reach the target.",
            "Use UNDO or NEW BOARD with visible controls.",
        ],
    }
}

fn cabinet_status(state: &AppState, game: GameId) -> &'static str {
    match game {
        GameId::Game2048 if state.records.best_2048 >= 2048 => "COMPLETE",
        GameId::Minesweeper if state.records.minesweeper.iter().any(Option::is_some) => "COMPLETE",
        GameId::Sudoku if state.records.sudoku.iter().any(Option::is_some) => "COMPLETE",
        GameId::Nonogram if state.records.nonogram.iter().any(Option::is_some) => "COMPLETE",
        GameId::Solitaire if state.records.solitaire_best_moves.is_some() => "COMPLETE",
        GameId::FreeCell if state.records.freecell_best_moves.is_some() => "COMPLETE",
        GameId::Yahtzee if state.records.fivefold_best_total > 0 => "COMPLETE",
        GameId::Reversi if state.records.reversi_best_score > 0 => "COMPLETE",
        GameId::LightsOut if state.records.lights_out_best_moves.is_some() => "COMPLETE",
        GameId::TicTacToe if state.records.tic_tac_toe_best_moves.is_some() => "COMPLETE",
        GameId::MemoryPairs if state.records.memory_pairs_best_moves.is_some() => "COMPLETE",
        GameId::SlidingPuzzle if state.records.sliding_puzzle_best_moves.is_some() => "COMPLETE",
        GameId::Mastermind if state.records.mastermind_best_rows.is_some() => "COMPLETE",
        GameId::Spider if state.records.spider_best_moves.is_some() => "COMPLETE",
        GameId::WordSearch if state.records.word_search_best_moves.is_some() => "COMPLETE",
        GameId::Hangman if state.records.hangman_best_moves.is_some() => "COMPLETE",
        GameId::ConnectFour if state.records.connect_four_best_moves.is_some() => "COMPLETE",
        GameId::Checkers if state.records.checkers_best_moves.is_some() => "COMPLETE",
        GameId::PegSolitaire if state.records.peg_solitaire_best_moves.is_some() => "COMPLETE",
        GameId::MahjongSolitaire if state.records.mahjong_solitaire_best_moves.is_some() => {
            "COMPLETE"
        }
        GameId::Snake if state.records.snake_best_score.is_some() => "COMPLETE",
        GameId::Breakout if state.records.breakout_best_score.is_some() => "COMPLETE",
        GameId::HigherLower if state.records.higher_lower_best_score.is_some() => "COMPLETE",
        GameId::KlondikeGolf if state.records.klondike_golf_best_moves.is_some() => "COMPLETE",
        GameId::Blackjack if state.records.blackjack_best_wins.is_some() => "COMPLETE",
        GameId::SpiderSolitaire if state.records.spider_solitaire_best_moves.is_some() => {
            "COMPLETE"
        }
        GameId::DungeonSweeper if state.records.dungeon_sweeper_best_moves.is_some() => "COMPLETE",
        GameId::Potion2048 if state.records.potion_2048_best_score.is_some() => "COMPLETE",
        GameId::TinyTowerDefence if state.records.tiny_tower_defence_best_wave.is_some() => {
            "COMPLETE"
        }
        GameId::OneRoomRoguelike if state.records.one_room_roguelike_best_score.is_some() => {
            "COMPLETE"
        }
        GameId::DailyDungeon if state.records.daily_dungeon_best_score.is_some() => "COMPLETE",
        GameId::DotsBoxes if state.records.dots_boxes_best_score.is_some() => "COMPLETE",
        GameId::Sokoban if state.records.sokoban_best_moves.is_some() => "COMPLETE",
        GameId::Mancala if state.records.mancala_best_score.is_some() => "COMPLETE",
        GameId::Hanoi if state.records.hanoi_best_moves.is_some() => "COMPLETE",
        GameId::NumberMatch if state.records.number_match_best_moves.is_some() => "COMPLETE",
        GameId::FloodIt if state.records.flood_it_best_moves.is_some() => "COMPLETE",
        GameId::ColorSort if state.records.color_sort_best_moves.is_some() => "COMPLETE",
        GameId::Battleship if state.records.battleship_best_moves.is_some() => "COMPLETE",
        GameId::WordGrid if state.records.word_grid_best_moves.is_some() => "COMPLETE",
        GameId::PipeLoop if state.records.pipe_loop_best_moves.is_some() => "COMPLETE",
        GameId::MazeWalk if state.records.maze_walk_best_moves.is_some() => "COMPLETE",
        GameId::MatchThree if state.records.match_three_best_score.is_some() => "COMPLETE",
        _ => "PLAY NOW",
    }
}
