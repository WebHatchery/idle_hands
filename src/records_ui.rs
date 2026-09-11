//! Collection-wide records screen.

use crate::ui::UiAction;
use crate::{progression::AchievementId, state::AppState};
use macroquad::prelude::*;

#[cfg(test)]
mod tests;

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
fn value(value: Option<u32>) -> String {
    value.map_or_else(|| "—".into(), |number| number.to_string())
}

pub fn draw_records(state: &AppState) {
    panel(
        Rect::new(120., 55., 1040., 610.),
        crate::theme::BACKGROUND_DEEP,
    );
    crate::ui::draw_text("RECORDS", 170., 125., 46., crate::theme::BRASS);
    crate::ui::draw_text(
        "Milestones from every drawer",
        174.,
        153.,
        19.,
        crate::theme::SECONDARY,
    );
    let summary = crate::collection_summary::from_state(state);
    let fastest = summary.fastest_seconds.map_or_else(
        || "—".into(),
        |seconds| crate::state_records::format_duration(u64::from(seconds)),
    );
    crate::ui::draw_text(
        format!(
            "STAMPS  {}   •   ACHIEVEMENTS  {}/{}   •   DRAWERS  {}/{}",
            summary.stamps,
            summary.earned_achievements,
            summary.total_achievements,
            summary.completed_games,
            summary.total_games
        ),
        174.,
        185.,
        18.,
        crate::theme::BRASS,
    );
    crate::ui::draw_text(
        format!(
            "DAILY ROUTES  {} CLEARED   •   LOG {}/90   •   BEST SCORE  {}   •   NEXT  {}   •   TIME {}   •   {} ACTIVE   •   FASTEST {}",
            state.records.daily_clear_count(),
            state.records.daily_results.len(),
            value(state.records.daily_best_score()),
            next_achievement(&state.records),
            crate::state_records::format_duration(summary.total_playtime_seconds),
            summary.active_games,
            fastest
        ),
        174.,
        210.,
        13.,
        crate::theme::SECONDARY,
    );
    panel(Rect::new(900., 102., 210., 44.), crate::theme::SURFACE);
    draw_rectangle_lines(900., 102., 210., 44., 3., WHITE);
    crate::ui::draw_text("ACHIEVEMENTS", 925., 129., 14., WHITE);
    panel(Rect::new(900., 154., 210., 44.), crate::theme::SURFACE);
    crate::ui::draw_text("DAILY ARCHIVE", 925., 181., 14., WHITE);
    panel(Rect::new(900., 206., 210., 44.), crate::theme::SURFACE);
    crate::ui::draw_text(
        format!(
            "SHELF: {}",
            crate::records_data::filter_label(state.records_filter)
        ),
        925.,
        233.,
        14.,
        WHITE,
    );
    crate::ui::draw_text(
        crate::records_data::summary_label(state, state.records_filter),
        925.,
        247.,
        10.,
        crate::theme::SECONDARY,
    );
    if state.records_filter != 0 {
        draw_filtered_records(state);
        return;
    }
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
            "Munch Maze best",
            value(state.records.munch_maze_best_score),
        ),
        (
            "Block Stack best",
            value(state.records.block_stack_best_score),
        ),
        (
            "Terrain Cannon best",
            value(state.records.terrain_cannon_best_score),
        ),
        (
            "Fling Fury best",
            value(state.records.fling_fury_best_score),
        ),
        (
            "Paddle Duel best",
            value(state.records.paddle_duel_best_score),
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
    let mut rows = Vec::new();
    rows.extend(left);
    rows.extend(middle);
    rows.extend(right);
    rows.extend(far_right);
    for (index, (label, score)) in rows.iter().enumerate() {
        let column = index / 14;
        let row = index % 14;
        let x = 160. + column as f32 * 200.;
        let y = 260. + row as f32 * 26.;
        crate::ui::draw_text(label, x, y, 10., crate::theme::CREAM);
        crate::ui::draw_text(score, x + 150., y, 11., crate::theme::BRASS);
    }
    panel(Rect::new(930., 590., 180., 48.), crate::theme::MOSS_DARK);
    crate::ui::draw_text("BACK", 990., 621., 18., WHITE);
}

fn draw_filtered_records(state: &AppState) {
    let rows = crate::records_data::rows(state, state.records_filter);
    let start = state.library_scroll.min(rows.len().saturating_sub(56));
    for (index, row) in rows.iter().skip(start).take(56).enumerate() {
        let column = index / 14;
        let line = index % 14;
        let x = 160. + column as f32 * 200.;
        let y = 260. + line as f32 * 26.;
        crate::ui::draw_text(row.label, x, y, 10., crate::theme::CREAM);
        crate::ui::draw_text(&row.score, x + 150., y, 11., crate::theme::BRASS);
    }
    crate::ui::draw_text(
        crate::records_data::page_label(start, rows.len(), 56),
        930.,
        575.,
        12.,
        crate::theme::SECONDARY,
    );
    panel(Rect::new(930., 590., 180., 48.), crate::theme::MOSS_DARK);
    crate::ui::draw_text("BACK", 990., 621., 18., WHITE);
}

fn next_achievement(records: &crate::state::CollectionRecords) -> &'static str {
    AchievementId::next_locked(records)
        .map(AchievementId::title)
        .unwrap_or("ALL COMPLETE")
}

pub fn records_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(900., 102., 210., 44.).contains(p) {
        vec![UiAction::Achievements]
    } else if Rect::new(900., 154., 210., 44.).contains(p) {
        vec![UiAction::DailyArchive]
    } else if Rect::new(900., 206., 210., 44.).contains(p) {
        vec![UiAction::RecordsFilter(crate::records_data::next_filter(
            state.records_filter,
        ))]
    } else if Rect::new(930., 590., 180., 48.).contains(p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}
