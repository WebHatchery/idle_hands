//! Compact portrait layouts for collection-wide library screens.

use crate::{
    progression::{completed_games, AchievementId},
    state::{AppState, GameId},
    ui::UiAction,
};
use macroquad::prelude::*;

fn panel(rect: Rect, fill: Color) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        crate::theme::drawer_surface(fill),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::BORDER);
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
fn back_button(y: f32) {
    panel(Rect::new(10., y, 150., 44.), crate::theme::MOSS_DARK);
    text("BACK", 62., y + 28., 12., WHITE);
}
fn value(value: Option<u32>) -> String {
    value.map_or_else(|| "-".into(), |number| number.to_string())
}

const RECORDS_VISIBLE_ROWS: usize = 11;
const RULES_VISIBLE_ROWS: usize = 8;

fn scroll_button(rect: Rect, label: &str) {
    panel(rect, crate::theme::SURFACE_DARK);
    text(label, rect.x + 18., rect.y + 28., 11., WHITE);
}

pub fn draw_records(state: &AppState) {
    panel(
        Rect::new(8., 20., 344., 680.),
        crate::theme::BACKGROUND_DEEP,
    );
    text("RECORDS", 20., 62., 29., crate::theme::BRASS);
    text(
        &format!(
            "Milestones  ·  Daily {} clears  ·  log {}/90  ·  best {}",
            state.records.daily_clear_count(),
            state.records.daily_results.len(),
            value(state.records.daily_best_score())
        ),
        22.,
        88.,
        10.,
        crate::theme::SECONDARY,
    );
    let earned = state.achievements.iter().filter(|earned| **earned).count();
    let completed = completed_games(&state.records);
    text(
        &format!(
            "STAMPS {}  -  ACHIEVEMENTS {}/{}",
            state.stamps,
            earned,
            AchievementId::ALL.len()
        ),
        20.,
        110.,
        12.,
        crate::theme::BRASS,
    );
    panel(Rect::new(190., 28., 155., 44.), crate::theme::SURFACE);
    text("ACHIEVEMENTS", 202., 56., 10., WHITE);
    draw_rectangle_lines(190., 28., 155., 44., 3., WHITE);
    text(
        &format!("DRAWERS {}/{}", completed, GameId::ALL.len()),
        210.,
        88.,
        11.,
        crate::theme::BRASS,
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
            "Space Invaders best",
            value(state.records.space_invaders_best_score.map(u32::from)),
        ),
        (
            "Asteroids best",
            value(state.records.asteroids_best_score.map(u32::from)),
        ),
        (
            "Frogger best",
            value(state.records.frogger_best_score.map(u32::from)),
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
        (
            "Sokoban best",
            value(state.records.sokoban_best_moves.map(u32::from)),
        ),
        (
            "Mancala best",
            value(state.records.mancala_best_score.map(u32::from)),
        ),
        (
            "Hanoi best",
            value(state.records.hanoi_best_moves.map(u32::from)),
        ),
        (
            "Number Match best",
            value(state.records.number_match_best_moves.map(u32::from)),
        ),
        (
            "Flood It best",
            value(state.records.flood_it_best_moves.map(u32::from)),
        ),
        (
            "Color Sort best",
            value(state.records.color_sort_best_moves.map(u32::from)),
        ),
        (
            "Battleship best",
            value(state.records.battleship_best_moves.map(u32::from)),
        ),
        (
            "Word Grid best",
            value(state.records.word_grid_best_moves.map(u32::from)),
        ),
        (
            "Pipe Loop best",
            value(state.records.pipe_loop_best_moves.map(u32::from)),
        ),
        (
            "Maze Walk best",
            value(state.records.maze_walk_best_moves.map(u32::from)),
        ),
        (
            "Match Three best",
            value(state.records.match_three_best_score.map(u32::from)),
        ),
        (
            "Pyramid best",
            value(state.records.pyramid_best_moves.map(u32::from)),
        ),
        (
            "TriPeaks best",
            value(state.records.tri_peaks_best_moves.map(u32::from)),
        ),
        (
            "Nim best",
            value(state.records.nim_best_moves.map(u32::from)),
        ),
        (
            "Word Ladder best",
            value(state.records.word_ladder_best_moves.map(u32::from)),
        ),
        (
            "Riddle Room best",
            value(state.records.misc_best_moves[0].map(u32::from)),
        ),
        (
            "Pattern Vault best",
            value(state.records.misc_best_moves[1].map(u32::from)),
        ),
        (
            "Sum Circuit best",
            value(state.records.misc_best_moves[2].map(u32::from)),
        ),
        (
            "Orbit Order best",
            value(state.records.misc_best_moves[3].map(u32::from)),
        ),
        (
            "Word Forge best",
            value(state.records.misc_best_moves[4].map(u32::from)),
        ),
    ];
    let start = state
        .library_scroll
        .min(rows.len().saturating_sub(RECORDS_VISIBLE_ROWS));
    for (index, (label, score)) in rows
        .iter()
        .skip(start)
        .take(RECORDS_VISIBLE_ROWS)
        .enumerate()
    {
        let rect = Rect::new(18., 122. + index as f32 * 42., 324., 36.);
        panel(rect, Color::new(0.13, 0.09, 0.20, 1.));
        text(label, rect.x + 10., rect.y + 24., 13., crate::theme::CREAM);
        let score_width = crate::ui::measure_text(score, None, 14, 1.).width;
        text(
            score,
            rect.right() - score_width - 10.,
            rect.y + 24.,
            14.,
            crate::theme::BRASS,
        );
    }
    scroll_button(Rect::new(10., 602., 100., 44.), "PREV");
    scroll_button(Rect::new(250., 602., 100., 44.), "NEXT");
    text(
        &format!(
            "{}-{} OF {}",
            start + 1,
            (start + RECORDS_VISIBLE_ROWS).min(rows.len()),
            rows.len()
        ),
        128.,
        630.,
        11.,
        crate::theme::CREAM,
    );
    back_button(714.);
}
pub fn records_clicks(p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(190., 28., 155., 44.), p) {
        vec![UiAction::Achievements]
    } else if crate::ui::hit(Rect::new(10., 602., 100., 44.), p) {
        vec![UiAction::LibraryScroll(-1)]
    } else if crate::ui::hit(Rect::new(250., 602., 100., 44.), p) {
        vec![UiAction::LibraryScroll(1)]
    } else if crate::ui::hit(Rect::new(10., 714., 150., 44.), p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}

pub fn draw_rules(state: &AppState) {
    panel(
        Rect::new(8., 20., 344., 680.),
        crate::theme::BACKGROUND_DEEP,
    );
    text("RULES", 20., 62., 29., crate::theme::BRASS);
    text(
        "Every drawer keeps its controls visible.",
        20.,
        88.,
        12.,
        crate::theme::SECONDARY,
    );
    let start = state
        .library_scroll
        .min(GameId::ALL.len().saturating_sub(RULES_VISIBLE_ROWS));
    for (index, game) in GameId::ALL
        .iter()
        .skip(start)
        .take(RULES_VISIBLE_ROWS)
        .enumerate()
    {
        let rect = Rect::new(18., 108. + index as f32 * 60., 324., 54.);
        panel(rect, Color::new(0.13, 0.09, 0.20, 1.));
        text(
            game.title(),
            rect.x + 10.,
            rect.y + 22.,
            14.,
            crate::theme::BRASS,
        );
        text(
            game.subtitle(),
            rect.x + 10.,
            rect.y + 43.,
            11.,
            crate::theme::CREAM,
        );
    }
    scroll_button(Rect::new(10., 602., 100., 44.), "PREV");
    scroll_button(Rect::new(250., 602., 100., 44.), "NEXT");
    text(
        &format!(
            "{}-{} OF {}",
            start + 1,
            (start + RULES_VISIBLE_ROWS).min(GameId::ALL.len()),
            GameId::ALL.len()
        ),
        128.,
        630.,
        11.,
        crate::theme::CREAM,
    );
    back_button(714.);
}
pub fn rules_clicks(p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(10., 602., 100., 44.), p) {
        vec![UiAction::LibraryScroll(-1)]
    } else if crate::ui::hit(Rect::new(250., 602., 100., 44.), p) {
        vec![UiAction::LibraryScroll(1)]
    } else if crate::ui::hit(Rect::new(10., 714., 150., 44.), p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}

pub fn draw_credits() {
    panel(
        Rect::new(8., 70., 344., 520.),
        crate::theme::BACKGROUND_DEEP,
    );
    text("CREDITS", 20., 115., 29., crate::theme::BRASS);
    text("IDLE HANDS", 22., 165., 22., WHITE);
    text("A warm collection for", 22., 210., 15., crate::theme::CREAM);
    text("small pauses.", 22., 235., 15., crate::theme::CREAM);
    text(
        "Built with Rust, macroquad,",
        22.,
        295.,
        13.,
        crate::theme::SECONDARY,
    );
    text(
        "and the shared toolkit.",
        22.,
        320.,
        13.,
        crate::theme::SECONDARY,
    );
    text(
        "Original generated artwork",
        22.,
        370.,
        13.,
        crate::theme::BRASS,
    );
    text(
        "created for Idle Hands.",
        22.,
        395.,
        13.,
        crate::theme::BRASS,
    );
    text(
        "Provenance ships with the game.",
        22.,
        430.,
        11.,
        crate::theme::SECONDARY,
    );
    text("PRIVACY", 22., 475., 13., crate::theme::CREAM);
    text(
        "Anonymous playtime and progress",
        22.,
        500.,
        11.,
        crate::theme::CREAM,
    );
    text(
        "help improve WebHatchery games.",
        22.,
        523.,
        11.,
        crate::theme::CREAM,
    );
    text(
        "No ads; no name or email collected.",
        22.,
        555.,
        11.,
        crate::theme::CREAM,
    );
    back_button(650.);
}
pub fn credits_clicks(p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(10., 650., 150., 44.), p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}

pub fn draw_help() {
    panel(
        Rect::new(8., 38., 344., 602.),
        crate::theme::BACKGROUND_DEEP,
    );
    text("HOW TO PLAY", 20., 80., 26., crate::theme::BRASS);
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
    panel(Rect::new(10., 530., 105., 44.), crate::theme::SURFACE);
    panel(Rect::new(127., 530., 105., 44.), crate::theme::SURFACE);
    panel(Rect::new(244., 530., 106., 44.), crate::theme::MOSS_DARK);
    text("RULES", 42., 558., 12., WHITE);
    text("CREDITS", 150., 558., 11., WHITE);
    text("BACK", 277., 558., 12., WHITE);
}
pub fn help_clicks(p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(10., 530., 105., 44.), p) {
        vec![UiAction::Rules]
    } else if crate::ui::hit(Rect::new(127., 530., 105., 44.), p) {
        vec![UiAction::Credits]
    } else if crate::ui::hit(Rect::new(244., 530., 106., 44.), p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}
