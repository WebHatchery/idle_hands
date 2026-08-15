//! Medium landscape layouts for library and settings screens.

use crate::{cosmetics, progression::AchievementId, state::AppState, ui::UiAction};
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
fn back(rect: Rect) {
    panel(rect, Color::new(0.25, 0.16, 0.32, 1.));
    text("BACK", rect.x + 30., rect.y + 28., 12., WHITE);
}

pub fn draw_records(state: &AppState) {
    panel(
        Rect::new(20., 12., 804., 365.),
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    text("RECORDS", 40., 48., 25., Color::new(0.98, 0.83, 0.45, 1.));
    let earned = state.achievements.iter().filter(|v| **v).count();
    text(
        &format!(
            "STAMPS {}  •  ACHIEVEMENTS {}/{}",
            state.stamps,
            earned,
            AchievementId::ALL.len()
        ),
        250.,
        46.,
        12.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    let rows = [
        ("2048 best", state.records.best_2048.to_string()),
        ("Mines beginner", value(state.records.minesweeper[0])),
        ("Mines intermediate", value(state.records.minesweeper[1])),
        ("Mines expert", value(state.records.minesweeper[2])),
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
    ];
    for (index, (label, score)) in rows.iter().enumerate() {
        let col = index / 7;
        let row = index % 7;
        let y = 86. + row as f32 * 34.;
        text(
            label,
            40. + col as f32 * 390.,
            y,
            12.,
            Color::new(0.78, 0.73, 0.86, 1.),
        );
        text(
            score,
            330. + col as f32 * 390.,
            y,
            12.,
            Color::new(0.98, 0.83, 0.45, 1.),
        );
    }
    back(Rect::new(700., 330., 110., 38.));
}
fn value(value: Option<u32>) -> String {
    value.map_or_else(|| "-".into(), |number| number.to_string())
}
pub fn records_clicks(p: Vec2) -> Vec<UiAction> {
    if Rect::new(700., 330., 110., 38.).contains(p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}

pub fn draw_rules() {
    panel(
        Rect::new(20., 12., 804., 365.),
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    text("RULES", 40., 48., 25., Color::new(0.98, 0.83, 0.45, 1.));
    let lines = [
        "2048  Swipe or tap a direction to merge matching tiles.",
        "Minesweeper  Reveal safely, flag mines, then chord marked numbers.",
        "Sudoku  Select a cell, then use the number pad; notes are optional.",
        "Nonogram  Fill or cross cells from the clues.",
        "Solitaire / FreeCell  Tap a card, then tap a legal destination.",
        "Fivefold  Roll up to three times, hold dice, then choose a score.",
        "Reversi  Place on a glowing legal square; pass only when blocked.",
    ];
    for (index, line) in lines.iter().enumerate() {
        text(
            line,
            40.,
            88. + index as f32 * 35.,
            12.,
            Color::new(0.78, 0.73, 0.86, 1.),
        );
    }
    back(Rect::new(700., 330., 110., 38.));
}
pub fn rules_clicks(p: Vec2) -> Vec<UiAction> {
    if Rect::new(700., 330., 110., 38.).contains(p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}

pub fn draw_credits() {
    panel(
        Rect::new(170., 20., 504., 350.),
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    text("CREDITS", 205., 62., 28., Color::new(0.98, 0.83, 0.45, 1.));
    text("IDLE HANDS", 205., 115., 20., WHITE);
    text(
        "A quiet collection for small pauses.",
        205.,
        155.,
        14.,
        Color::new(0.78, 0.73, 0.86, 1.),
    );
    text(
        "Built with Rust, macroquad, and the shared toolkit.",
        205.,
        205.,
        13.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
    text(
        "Designed for touch and quiet minutes.",
        205.,
        260.,
        13.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    back(Rect::new(365., 315., 110., 38.));
}
pub fn credits_clicks(p: Vec2) -> Vec<UiAction> {
    if Rect::new(365., 315., 110., 38.).contains(p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}

pub fn draw_help() {
    panel(
        Rect::new(20., 12., 804., 365.),
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    text(
        "HOW TO PLAY",
        40.,
        52.,
        26.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    text(
        "Idle Hands is a cabinet of calm, tactile games.",
        40.,
        92.,
        16.,
        WHITE,
    );
    text(
        "Tap a cabinet object to open it and use the visible controls in every drawer.",
        40.,
        125.,
        14.,
        Color::new(0.75, 0.70, 0.84, 1.),
    );
    for (rect, label) in [
        (Rect::new(430., 290., 110., 42.), "RULES"),
        (Rect::new(555., 290., 110., 42.), "CREDITS"),
        (Rect::new(680., 290., 130., 42.), "BACK"),
    ] {
        panel(rect, Color::new(0.20, 0.13, 0.30, 1.));
        text(label, rect.x + 30., rect.y + 27., 11., WHITE);
    }
}
pub fn help_clicks(p: Vec2) -> Vec<UiAction> {
    if Rect::new(430., 290., 110., 42.).contains(p) {
        vec![UiAction::Rules]
    } else if Rect::new(555., 290., 110., 42.).contains(p) {
        vec![UiAction::Credits]
    } else if Rect::new(680., 290., 130., 42.).contains(p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}

pub fn draw_settings(state: &AppState) {
    panel(
        Rect::new(20., 10., 804., 370.),
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    text("SETTINGS", 40., 45., 25., Color::new(0.98, 0.83, 0.45, 1.));
    text(
        &format!(
            "Profile: {}  •  Stamps: {}",
            state.profile_name, state.stamps
        ),
        220.,
        43.,
        13.,
        WHITE,
    );
    let rows = [
        ("CARD BACK", cosmetics::card_back_name(state.card_back)),
        (
            "BOARD THEME",
            cosmetics::board_theme_name(state.board_theme),
        ),
        ("SOUND SET", cosmetics::sound_set_name(state.sound_set)),
        (
            "CABINET DECOR",
            cosmetics::cabinet_decoration_name(state.cabinet_decoration),
        ),
    ];
    for (index, (label, value)) in rows.iter().enumerate() {
        let rect = Rect::new(40., 68. + index as f32 * 42., 370., 34.);
        panel(rect, Color::new(0.16, 0.11, 0.24, 1.));
        text(label, rect.x + 12., rect.y + 22., 11., WHITE);
        text(
            value,
            rect.x + 190.,
            rect.y + 22.,
            11.,
            Color::new(0.98, 0.83, 0.45, 1.),
        );
    }
    panel(
        Rect::new(450., 68., 160., 34.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text(
        if state.sound { "SOUND ON" } else { "SOUND OFF" },
        495.,
        90.,
        11.,
        WHITE,
    );
    panel(
        Rect::new(630., 68., 160., 34.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text(
        if state.reduced_motion {
            "MOTION OFF"
        } else {
            "MOTION ON"
        },
        670.,
        90.,
        11.,
        WHITE,
    );
    panel(
        Rect::new(450., 120., 160., 34.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("SAVE NOW", 500., 142., 11., WHITE);
    panel(
        Rect::new(630., 120., 160., 34.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("LOAD", 690., 142., 11., WHITE);
    panel(
        Rect::new(450., 165., 160., 34.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        if state.high_contrast {
            "CONTRAST ON"
        } else {
            "CONTRAST OFF"
        },
        475.,
        187.,
        10.,
        WHITE,
    );
    panel(
        Rect::new(630., 165., 160., 34.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        if state.large_text {
            "LARGE TEXT ON"
        } else {
            "LARGE TEXT OFF"
        },
        650.,
        187.,
        10.,
        WHITE,
    );
    text(
        "Tap a cosmetic row to cycle unlocked items.",
        450.,
        220.,
        12.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
    panel(
        Rect::new(40., 270., 160., 42.),
        Color::new(0.25, 0.16, 0.32, 1.),
    );
    text("BACK", 98., 297., 12., WHITE);
    panel(
        Rect::new(220., 270., 160., 42.),
        Color::new(0.36, 0.16, 0.22, 1.),
    );
    text("RESET DATA", 267., 297., 11., WHITE);
    if state.confirm_reset {
        panel(
            Rect::new(250., 150., 350., 150.),
            Color::new(0.16, 0.08, 0.16, 0.99),
        );
        text("Reset the cabinet?", 330., 190., 20., WHITE);
        text(
            "This removes saves and records.",
            300.,
            220.,
            13.,
            Color::new(0.78, 0.73, 0.86, 1.),
        );
        panel(
            Rect::new(285., 245., 110., 38.),
            Color::new(0.22, 0.18, 0.35, 1.),
        );
        text("CANCEL", 315., 270., 11., WHITE);
        panel(
            Rect::new(455., 245., 110., 38.),
            Color::new(0.45, 0.20, 0.24, 1.),
        );
        text("RESET", 490., 270., 11., WHITE);
    }
}
pub fn settings_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if state.confirm_reset {
        if Rect::new(285., 245., 110., 38.).contains(p) {
            return vec![UiAction::CancelResetData];
        }
        if Rect::new(455., 245., 110., 38.).contains(p) {
            return vec![UiAction::ConfirmResetData];
        }
        return vec![];
    }
    if Rect::new(40., 68., 370., 34.).contains(p) {
        return vec![UiAction::CycleCardBack];
    }
    if Rect::new(40., 110., 370., 34.).contains(p) {
        return vec![UiAction::CycleBoardTheme];
    }
    if Rect::new(40., 152., 370., 34.).contains(p) {
        return vec![UiAction::CycleSoundSet];
    }
    if Rect::new(40., 194., 370., 34.).contains(p) {
        return vec![UiAction::CycleCabinetDecoration];
    }
    if Rect::new(450., 68., 160., 34.).contains(p) {
        return vec![UiAction::ToggleSound];
    }
    if Rect::new(630., 68., 160., 34.).contains(p) {
        return vec![UiAction::ToggleMotion];
    }
    if Rect::new(450., 165., 160., 34.).contains(p) {
        return vec![UiAction::ToggleHighContrast];
    }
    if Rect::new(630., 165., 160., 34.).contains(p) {
        return vec![UiAction::ToggleLargeText];
    }
    if Rect::new(450., 120., 160., 34.).contains(p) {
        return vec![UiAction::Save];
    }
    if Rect::new(630., 120., 160., 34.).contains(p) {
        return vec![UiAction::Load];
    }
    if Rect::new(40., 270., 160., 42.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    if Rect::new(220., 270., 160., 42.).contains(p) {
        return vec![UiAction::ResetData];
    }
    vec![]
}
