//! Compact portrait layouts for collection-wide library screens.

use crate::{progression::AchievementId, state::AppState, ui::UiAction};
use macroquad::prelude::*;

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
fn back_button(y: f32) {
    panel(
        Rect::new(10., y, 150., 38.),
        Color::new(0.25, 0.16, 0.32, 1.),
    );
    text("BACK", 62., y + 25., 12., WHITE);
}
fn value(value: Option<u32>) -> String {
    value.map_or_else(|| "-".into(), |number| number.to_string())
}

pub fn draw_records(state: &AppState) {
    panel(
        Rect::new(8., 38., 344., 602.),
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    text("RECORDS", 20., 80., 29., Color::new(0.98, 0.83, 0.45, 1.));
    text(
        "Quiet milestones",
        22.,
        103.,
        13.,
        Color::new(0.72, 0.68, 0.82, 1.),
    );
    let earned = state.achievements.iter().filter(|earned| **earned).count();
    text(
        &format!(
            "STAMPS {}  •  ACHIEVEMENTS {}/{}",
            state.stamps,
            earned,
            AchievementId::ALL.len()
        ),
        20.,
        127.,
        11.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    let rows = [
        ("2048 best", state.records.best_2048.to_string()),
        ("Mines beginner", value(state.records.minesweeper[0])),
        ("Mines intermediate", value(state.records.minesweeper[1])),
        ("Mines expert", value(state.records.minesweeper[2])),
        ("Mines custom", value(state.records.minesweeper[3])),
        ("Sudoku easy", value(state.records.sudoku[0])),
        ("Sudoku medium", value(state.records.sudoku[1])),
        ("Sudoku hard", value(state.records.sudoku[2])),
        ("Nonogram 5x5", value(state.records.nonogram[0])),
        ("Nonogram 10x10", value(state.records.nonogram[1])),
        ("Nonogram 15x15", value(state.records.nonogram[2])),
        ("Solitaire best", value(state.records.solitaire_best_moves)),
        ("FreeCell best", value(state.records.freecell_best_moves)),
        (
            "Fivefold total",
            state.records.fivefold_best_total.to_string(),
        ),
        ("Reversi best", state.records.reversi_best_score.to_string()),
        (
            "Lights Out best",
            value(state.records.lights_out_best_moves.map(u32::from)),
        ),
        (
            "Tic-Tac-Toe best",
            value(state.records.tic_tac_toe_best_moves.map(u32::from)),
        ),
        (
            "Memory best",
            value(state.records.memory_pairs_best_moves.map(u32::from)),
        ),
        (
            "Sliding Puzzle best",
            value(state.records.sliding_puzzle_best_moves.map(u32::from)),
        ),
        (
            "Mastermind best",
            value(state.records.mastermind_best_rows.map(u32::from)),
        ),
        ("Spider best", value(state.records.spider_best_moves)),
        (
            "Word Search best",
            value(state.records.word_search_best_moves.map(u32::from)),
        ),
        (
            "Hangman best",
            value(state.records.hangman_best_moves.map(u32::from)),
        ),
        (
            "Connect Four best",
            value(state.records.connect_four_best_moves.map(u32::from)),
        ),
        (
            "Checkers best",
            value(state.records.checkers_best_moves.map(u32::from)),
        ),
        (
            "Peg Solitaire best",
            value(state.records.peg_solitaire_best_moves.map(u32::from)),
        ),
        (
            "Mahjong Solitaire best",
            value(state.records.mahjong_solitaire_best_moves.map(u32::from)),
        ),
        (
            "Snake best",
            value(state.records.snake_best_score.map(u32::from)),
        ),
        (
            "Breakout best",
            value(state.records.breakout_best_score.map(u32::from)),
        ),
        (
            "Higher or Lower best",
            value(state.records.higher_lower_best_score.map(u32::from)),
        ),
        (
            "Klondike Golf best",
            value(state.records.klondike_golf_best_moves.map(u32::from)),
        ),
        (
            "Blackjack wins",
            value(state.records.blackjack_best_wins.map(u32::from)),
        ),
        (
            "Spider Solitaire best",
            value(state.records.spider_solitaire_best_moves),
        ),
        (
            "Dungeon Sweeper best",
            value(state.records.dungeon_sweeper_best_moves.map(u32::from)),
        ),
        (
            "Potion 2048 best",
            value(state.records.potion_2048_best_score),
        ),
        (
            "Tower Defence wave",
            value(state.records.tiny_tower_defence_best_wave.map(u32::from)),
        ),
        (
            "Room Roguelike best",
            value(state.records.one_room_roguelike_best_score),
        ),
        (
            "Daily Dungeon best",
            value(state.records.daily_dungeon_best_score),
        ),
        (
            "Dots & Boxes best",
            value(state.records.dots_boxes_best_score.map(u32::from)),
        ),
    ];
    for (index, (label, score)) in rows.iter().enumerate() {
        let column = index / 19;
        let row = index % 19;
        let x = 20. + column as f32 * 170.;
        let y = 140. + row as f32 * 21.;
        text(label, x, y, 9., Color::new(0.78, 0.73, 0.86, 1.));
        text(score, x + 145., y, 10., Color::new(0.98, 0.83, 0.45, 1.));
    }
    back_button(650.);
}
pub fn records_clicks(p: Vec2) -> Vec<UiAction> {
    if Rect::new(10., 650., 150., 38.).contains(p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}

pub fn draw_rules() {
    panel(
        Rect::new(8., 38., 344., 602.),
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    text("RULES", 20., 80., 29., Color::new(0.98, 0.83, 0.45, 1.));
    text(
        "Every drawer keeps its controls visible.",
        20.,
        103.,
        12.,
        Color::new(0.72, 0.68, 0.82, 1.),
    );
    let lines = [
        "2048  Swipe or tap arrows.",
        "Mines  Reveal, flag, then chord.",
        "Sudoku  Select a cell and number.",
        "Nonogram  Fill or cross from clues.",
        "Solitaire  Tap card, then target.",
        "FreeCell  Move cards to cascades.",
        "Fivefold  Roll, hold, choose a call.",
        "Reversi  Place on glowing squares.",
        "",
        "All games support visible touch controls.",
    ];
    for (index, line) in lines.iter().enumerate() {
        text(
            line,
            20.,
            145. + index as f32 * 36.,
            12.,
            Color::new(0.78, 0.73, 0.86, 1.),
        );
    }
    back_button(650.);
}
pub fn rules_clicks(p: Vec2) -> Vec<UiAction> {
    if Rect::new(10., 650., 150., 38.).contains(p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}

pub fn draw_credits() {
    panel(
        Rect::new(8., 70., 344., 520.),
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    text("CREDITS", 20., 115., 29., Color::new(0.98, 0.83, 0.45, 1.));
    text("IDLE HANDS", 22., 165., 22., WHITE);
    text(
        "A quiet collection for",
        22.,
        210.,
        15.,
        Color::new(0.78, 0.73, 0.86, 1.),
    );
    text(
        "small pauses.",
        22.,
        235.,
        15.,
        Color::new(0.78, 0.73, 0.86, 1.),
    );
    text(
        "Built with Rust, macroquad,",
        22.,
        295.,
        13.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
    text(
        "and the shared toolkit.",
        22.,
        320.,
        13.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
    text(
        "Designed for touch and quiet minutes.",
        22.,
        390.,
        13.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    back_button(650.);
}
pub fn credits_clicks(p: Vec2) -> Vec<UiAction> {
    if Rect::new(10., 650., 150., 38.).contains(p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}

pub fn draw_help() {
    panel(
        Rect::new(8., 38., 344., 602.),
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    text(
        "HOW TO PLAY",
        20.,
        80.,
        26.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    text("Idle Hands is a cabinet", 20., 112., 14., WHITE);
    text("of calm, tactile games.", 20., 135., 14., WHITE);
    text(
        "Tap a cabinet object to open it.",
        20.,
        185.,
        12.,
        Color::new(0.75, 0.70, 0.84, 1.),
    );
    text(
        "Use the visible controls in every drawer.",
        20.,
        215.,
        12.,
        Color::new(0.75, 0.70, 0.84, 1.),
    );
    panel(
        Rect::new(10., 530., 105., 38.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    panel(
        Rect::new(127., 530., 105., 38.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    panel(
        Rect::new(244., 530., 106., 38.),
        Color::new(0.25, 0.16, 0.32, 1.),
    );
    text("RULES", 42., 555., 12., WHITE);
    text("CREDITS", 150., 555., 11., WHITE);
    text("BACK", 277., 555., 12., WHITE);
}
pub fn help_clicks(p: Vec2) -> Vec<UiAction> {
    if Rect::new(10., 530., 105., 38.).contains(p) {
        vec![UiAction::Rules]
    } else if Rect::new(127., 530., 105., 38.).contains(p) {
        vec![UiAction::Credits]
    } else if Rect::new(244., 530., 106., 38.).contains(p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}
