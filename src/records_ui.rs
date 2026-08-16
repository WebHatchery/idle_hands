//! Collection-wide records screen.

use crate::progression::AchievementId;
use crate::state::AppState;
use crate::ui::UiAction;
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
fn value(value: Option<u32>) -> String {
    value.map_or_else(|| "—".into(), |number| number.to_string())
}

pub fn draw_records(state: &AppState) {
    panel(
        Rect::new(120., 55., 1040., 610.),
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    draw_text("RECORDS", 170., 125., 46., Color::new(0.98, 0.83, 0.45, 1.));
    draw_text(
        "Quiet milestones from every drawer",
        174.,
        153.,
        19.,
        Color::new(0.72, 0.68, 0.82, 1.),
    );
    let earned = state.achievements.iter().filter(|earned| **earned).count();
    draw_text(
        format!(
            "STAMPS  {}   •   ACHIEVEMENTS  {}/{}",
            state.stamps,
            earned,
            AchievementId::ALL.len()
        ),
        174.,
        185.,
        18.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    let left = [
        ("2048 best score", state.records.best_2048.to_string()),
        ("Minesweeper beginner", value(state.records.minesweeper[0])),
        (
            "Minesweeper intermediate",
            value(state.records.minesweeper[1]),
        ),
        ("Minesweeper expert", value(state.records.minesweeper[2])),
        ("Minesweeper custom", value(state.records.minesweeper[3])),
        ("Sudoku easy moves", value(state.records.sudoku[0])),
        ("Sudoku medium moves", value(state.records.sudoku[1])),
    ];
    let middle = [
        ("Sudoku hard moves", value(state.records.sudoku[2])),
        ("Nonogram 5 × 5 moves", value(state.records.nonogram[0])),
        ("Nonogram 10 × 10 moves", value(state.records.nonogram[1])),
        ("Nonogram 15 × 15 moves", value(state.records.nonogram[2])),
        (
            "Solitaire best moves",
            value(state.records.solitaire_best_moves),
        ),
        (
            "FreeCell best moves",
            value(state.records.freecell_best_moves),
        ),
        (
            "Fivefold best total",
            state.records.fivefold_best_total.to_string(),
        ),
        (
            "Reversi best score",
            state.records.reversi_best_score.to_string(),
        ),
    ];
    let right = [
        (
            "Lights Out best moves",
            value(state.records.lights_out_best_moves.map(u32::from)),
        ),
        (
            "Tic-Tac-Toe best moves",
            value(state.records.tic_tac_toe_best_moves.map(u32::from)),
        ),
        (
            "Memory best moves",
            value(state.records.memory_pairs_best_moves.map(u32::from)),
        ),
        (
            "Sliding Puzzle best moves",
            value(state.records.sliding_puzzle_best_moves.map(u32::from)),
        ),
        (
            "Mastermind best guesses",
            value(state.records.mastermind_best_rows.map(u32::from)),
        ),
        ("Spider best moves", value(state.records.spider_best_moves)),
        (
            "Word Search best moves",
            value(state.records.word_search_best_moves.map(u32::from)),
        ),
        (
            "Hangman best moves",
            value(state.records.hangman_best_moves.map(u32::from)),
        ),
    ];
    let far_right = [
        (
            "Connect Four best moves",
            value(state.records.connect_four_best_moves.map(u32::from)),
        ),
        (
            "Checkers best moves",
            value(state.records.checkers_best_moves.map(u32::from)),
        ),
        (
            "Peg Solitaire best moves",
            value(state.records.peg_solitaire_best_moves.map(u32::from)),
        ),
        (
            "Mahjong Solitaire best moves",
            value(state.records.mahjong_solitaire_best_moves.map(u32::from)),
        ),
        (
            "Snake best score",
            value(state.records.snake_best_score.map(u32::from)),
        ),
        (
            "Breakout best score",
            value(state.records.breakout_best_score.map(u32::from)),
        ),
        (
            "Higher or Lower best score",
            value(state.records.higher_lower_best_score.map(u32::from)),
        ),
        (
            "Klondike Golf best moves",
            value(state.records.klondike_golf_best_moves.map(u32::from)),
        ),
        (
            "Blackjack best wins",
            value(state.records.blackjack_best_wins.map(u32::from)),
        ),
        (
            "Spider Solitaire best moves",
            value(state.records.spider_solitaire_best_moves),
        ),
        (
            "Dungeon best",
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
            "Sokoban best moves",
            value(state.records.sokoban_best_moves.map(u32::from)),
        ),
        (
            "Mancala best stones",
            value(state.records.mancala_best_score.map(u32::from)),
        ),
        (
            "Hanoi best moves",
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
            "Pyramid best moves",
            value(state.records.pyramid_best_moves.map(u32::from)),
        ),
        (
            "TriPeaks best moves",
            value(state.records.tri_peaks_best_moves.map(u32::from)),
        ),
        (
            "Nim best moves",
            value(state.records.nim_best_moves.map(u32::from)),
        ),
        (
            "Word Ladder best",
            value(state.records.word_ladder_best_moves.map(u32::from)),
        ),
    ];
    let mut rows = Vec::new();
    rows.extend(left);
    rows.extend(middle);
    rows.extend(right);
    rows.extend(far_right);
    for (index, (label, score)) in rows.iter().enumerate() {
        let column = index / 11;
        let row = index % 11;
        let x = 160. + column as f32 * 200.;
        let y = 240. + row as f32 * 32.;
        draw_text(label, x, y, 10., Color::new(0.78, 0.73, 0.86, 1.));
        draw_text(score, x + 150., y, 11., Color::new(0.98, 0.83, 0.45, 1.));
    }
    panel(
        Rect::new(930., 590., 180., 48.),
        Color::new(0.25, 0.16, 0.32, 1.),
    );
    draw_text("BACK", 990., 621., 18., WHITE);
}
pub fn records_clicks(p: Vec2) -> Vec<UiAction> {
    if Rect::new(930., 590., 180., 48.).contains(p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}
