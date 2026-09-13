use super::{back_button, panel, scroll_button, text, value, RECORDS_VISIBLE_ROWS};
use crate::{progression::AchievementId, state::AppState, ui::UiAction};
use macroquad::prelude::*;

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
    let summary = crate::collection_summary::from_state(state);
    text(
        &format!(
            "STAMPS {}  -  ACHIEVEMENTS {}/{} ({}%)  -  TIME {}  -  {} ACTIVE",
            summary.stamps,
            summary.earned_achievements,
            summary.total_achievements,
            summary.achievement_percent(),
            crate::state_records::format_duration(summary.total_playtime_seconds),
            summary.active_games
        ),
        20.,
        110.,
        12.,
        crate::theme::BRASS,
    );
    text(
        &format!(
            "DRAWERS {}/{} ({}%)  ·  NEXT {}  ·  {}",
            summary.completed_games,
            summary.total_games,
            summary.completion_percent(),
            next_achievement(state),
            summary.fastest_label()
        ),
        20.,
        148.,
        9.,
        crate::theme::SECONDARY,
    );
    panel(Rect::new(190., 28., 155., 44.), crate::theme::SURFACE);
    text("ACHIEVEMENTS", 202., 56., 10., WHITE);
    draw_rectangle_lines(190., 28., 155., 44., 3., WHITE);
    panel(Rect::new(190., 76., 155., 44.), crate::theme::SURFACE);
    text("DAILY LOG", 225., 104., 10., WHITE);
    draw_rectangle_lines(190., 76., 155., 44., 3., WHITE);
    panel(Rect::new(190., 124., 155., 44.), crate::theme::SURFACE);
    text(
        &format!(
            "SHELF {}",
            crate::records_data::filter_label(state.records_filter)
        ),
        214.,
        152.,
        10.,
        WHITE,
    );
    text(
        &crate::rules_data::summary_label(state.rules_filter),
        214.,
        68.,
        7.,
        crate::theme::SECONDARY,
    );
    text(
        &crate::records_data::summary_label(state, state.records_filter),
        211.,
        164.,
        7.,
        crate::theme::SECONDARY,
    );
    draw_rectangle_lines(190., 124., 155., 44., 3., WHITE);
    panel(Rect::new(18., 124., 155., 44.), crate::theme::SURFACE);
    text("STATS", 70., 152., 10., WHITE);
    draw_rectangle_lines(18., 124., 155., 44., 3., WHITE);
    if state.records_filter != 0 {
        draw_filtered_records(state);
        return;
    }
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
        let rect = Rect::new(18., 174. + index as f32 * 42., 324., 36.);
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
            "{}  ·  {}-{} OF {}",
            crate::records_data::page_label(start, rows.len(), RECORDS_VISIBLE_ROWS),
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

fn draw_filtered_records(state: &AppState) {
    let rows = crate::records_data::rows(state, state.records_filter);
    let start = state
        .library_scroll
        .min(rows.len().saturating_sub(RECORDS_VISIBLE_ROWS));
    for (index, row) in rows
        .iter()
        .skip(start)
        .take(RECORDS_VISIBLE_ROWS)
        .enumerate()
    {
        let rect = Rect::new(18., 174. + index as f32 * 42., 324., 36.);
        panel(rect, Color::new(0.13, 0.09, 0.20, 1.));
        text(
            row.label,
            rect.x + 10.,
            rect.y + 24.,
            13.,
            crate::theme::CREAM,
        );
        let score_width = crate::ui::measure_text(&row.score, None, 14, 1.).width;
        text(
            &row.score,
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
            "{}  ·  {}-{} OF {}",
            crate::records_data::page_label(start, rows.len(), RECORDS_VISIBLE_ROWS),
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

fn next_achievement(state: &AppState) -> String {
    AchievementId::next_locked(&state.records)
        .map(|achievement| achievement.title_from(&state.content))
        .unwrap_or_else(|| "ALL COMPLETE".into())
}

pub fn records_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(190., 28., 155., 44.), p) {
        vec![UiAction::Achievements]
    } else if crate::ui::hit(Rect::new(190., 76., 155., 44.), p) {
        vec![UiAction::DailyArchive]
    } else if crate::ui::hit(Rect::new(190., 124., 155., 44.), p) {
        vec![UiAction::RecordsFilter(crate::records_data::next_filter(
            state.records_filter,
        ))]
    } else if crate::ui::hit(Rect::new(18., 124., 155., 44.), p) {
        vec![UiAction::Statistics]
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
