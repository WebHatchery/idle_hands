//! Medium landscape layouts for library and settings screens.

use crate::{cosmetics, progression::AchievementId, state::AppState, ui::UiAction};
use macroquad::prelude::*;

#[cfg(test)]
mod tests;

pub(crate) fn panel(rect: Rect, fill: Color) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        crate::theme::drawer_surface(fill),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::BORDER);
}
pub(crate) fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
pub(crate) fn back(rect: Rect) {
    panel(rect, crate::theme::MOSS_DARK);
    text("BACK", rect.x + 30., rect.y + 28., 12., WHITE);
}

const RECORDS_VISIBLE_ROWS: usize = 10;

pub(crate) fn scroll(rect: Rect, label: &str) {
    panel(rect, crate::theme::SURFACE_DARK);
    text(label, rect.x + 16., rect.y + 28., 10., WHITE);
}

pub fn draw_records(state: &AppState) {
    panel(
        Rect::new(20., 12., 804., 365.),
        crate::theme::BACKGROUND_DEEP,
    );
    draw_rectangle_lines(
        20.,
        12.,
        804.,
        365.,
        2.,
        if state.high_contrast {
            WHITE
        } else {
            crate::theme::BORDER
        },
    );
    text("RECORDS", 40., 48., 25., crate::theme::BRASS);
    let summary = crate::collection_summary::from_state(state);
    text(
        &format!(
            "STAMPS {}  -  ACHIEVEMENTS {}/{} ({}%)  -  DRAWERS {}/{} ({}%)",
            summary.stamps,
            summary.earned_achievements,
            summary.total_achievements,
            summary.achievement_percent(),
            summary.completed_games,
            summary.total_games,
            summary.completion_percent()
        ),
        250.,
        46.,
        12.,
        crate::theme::BRASS,
    );
    text(
        &format!(
            "DAILY {} CLEARS  -  LOG {}/90  -  BEST {}  -  NEXT {}  -  TIME {}  -  {} ACTIVE  -  {}",
            state.records.daily_clear_count(),
            state.records.daily_results.len(),
            value(state.records.daily_best_score()),
            next_achievement(&state.records),
            crate::state_records::format_duration(summary.total_playtime_seconds),
            summary.active_games,
            summary.fastest_label()
        ),
        40.,
        62.,
        10.,
        crate::theme::SECONDARY,
    );
    panel(Rect::new(650., 2., 150., 44.), crate::theme::SURFACE);
    text("ACHIEVEMENTS", 663., 30., 9., WHITE);
    draw_rectangle_lines(650., 2., 150., 44., 3., WHITE);
    panel(Rect::new(480., 2., 150., 44.), crate::theme::SURFACE);
    text("DAILY LOG", 510., 30., 9., WHITE);
    draw_rectangle_lines(480., 2., 150., 44., 3., WHITE);
    panel(Rect::new(310., 2., 150., 44.), crate::theme::SURFACE);
    text(
        &format!(
            "SHELF {}",
            crate::records_data::filter_label(state.records_filter)
        ),
        335.,
        30.,
        9.,
        WHITE,
    );
    text(
        &crate::records_data::summary_label(state, state.records_filter),
        335.,
        42.,
        7.,
        crate::theme::SECONDARY,
    );
    panel(Rect::new(140., 2., 150., 44.), crate::theme::SURFACE);
    text("STATS", 195., 30., 9., WHITE);
    draw_rectangle_lines(140., 2., 150., 44., 3., WHITE);
    if state.records_filter != 0 {
        draw_filtered_records(state);
        return;
    }
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
        let rect = Rect::new(
            30. + (index % 2) as f32 * 380.,
            85. + (index / 2) as f32 * 50.,
            360.,
            44.,
        );
        panel(rect, Color::new(0.13, 0.09, 0.20, 1.));
        text(label, rect.x + 12., rect.y + 29., 13., crate::theme::CREAM);
        let width = crate::ui::measure_text(score, None, 14, 1.).width;
        text(
            score,
            rect.right() - width - 12.,
            rect.y + 29.,
            14.,
            crate::theme::BRASS,
        );
    }
    scroll(Rect::new(430., 330., 100., 44.), "PREV");
    scroll(Rect::new(545., 330., 100., 44.), "NEXT");
    back(Rect::new(700., 330., 110., 44.));
    draw_rectangle_lines(700., 330., 110., 44., 3., WHITE);
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
        let rect = Rect::new(
            30. + (index % 2) as f32 * 380.,
            85. + (index / 2) as f32 * 50.,
            360.,
            44.,
        );
        panel(rect, Color::new(0.13, 0.09, 0.20, 1.));
        text(
            row.label,
            rect.x + 12.,
            rect.y + 29.,
            13.,
            crate::theme::CREAM,
        );
        let width = crate::ui::measure_text(&row.score, None, 14, 1.).width;
        text(
            &row.score,
            rect.right() - width - 12.,
            rect.y + 29.,
            14.,
            crate::theme::BRASS,
        );
    }
    text(
        &crate::records_data::page_label(start, rows.len(), RECORDS_VISIBLE_ROWS),
        650.,
        78.,
        8.,
        crate::theme::SECONDARY,
    );
    scroll(Rect::new(430., 330., 100., 44.), "PREV");
    scroll(Rect::new(545., 330., 100., 44.), "NEXT");
    back(Rect::new(700., 330., 110., 44.));
    draw_rectangle_lines(700., 330., 110., 44., 3., WHITE);
}

fn next_achievement(records: &crate::state::CollectionRecords) -> &'static str {
    AchievementId::next_locked(records)
        .map(AchievementId::title)
        .unwrap_or("ALL COMPLETE")
}

fn value(value: Option<u32>) -> String {
    value.map_or_else(|| "-".into(), |number| number.to_string())
}
pub fn records_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(310., 2., 150., 44.), p) {
        vec![UiAction::RecordsFilter(crate::records_data::next_filter(
            state.records_filter,
        ))]
    } else if crate::ui::hit(Rect::new(140., 2., 150., 44.), p) {
        vec![UiAction::Statistics]
    } else if crate::ui::hit(Rect::new(480., 2., 150., 44.), p) {
        vec![UiAction::DailyArchive]
    } else if crate::ui::hit(Rect::new(650., 2., 150., 44.), p) {
        vec![UiAction::Achievements]
    } else if crate::ui::hit(Rect::new(430., 330., 100., 44.), p) {
        vec![UiAction::LibraryScroll(-1)]
    } else if crate::ui::hit(Rect::new(545., 330., 100., 44.), p) {
        vec![UiAction::LibraryScroll(1)]
    } else if crate::ui::hit(Rect::new(700., 330., 110., 44.), p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}

pub fn draw_credits(state: &AppState) {
    let credits_panel = Rect::new(170., 20., 504., 350.);
    panel(credits_panel, crate::theme::BACKGROUND_DEEP);
    if state.high_contrast {
        draw_rectangle_lines(
            credits_panel.x,
            credits_panel.y,
            credits_panel.w,
            credits_panel.h,
            3.,
            WHITE,
        );
    }
    text(
        "CREDITS",
        205.,
        62.,
        crate::accessibility::text_size(28., state.large_text),
        crate::theme::BRASS,
    );
    text(
        crate::credits_data::TITLE,
        205.,
        115.,
        crate::accessibility::text_size(20., state.large_text),
        WHITE,
    );
    let mut y = 155.;
    for (index, paragraph) in crate::credits_data::PARAGRAPHS.iter().enumerate() {
        let base_size = if index == crate::credits_data::PARAGRAPHS.len() - 1 {
            13.
        } else if index == 0 {
            14.
        } else {
            10.
        };
        let size = crate::accessibility::text_size(base_size, state.large_text);
        let color = if state.high_contrast {
            WHITE
        } else if index == 0 || index == crate::credits_data::PARAGRAPHS.len() - 1 {
            crate::theme::CREAM
        } else {
            crate::theme::SECONDARY
        };
        for line in macroquad_toolkit::ui::wrap_text(paragraph, 440., size) {
            text(&line, 205., y, size, color);
            y += size + 4.;
        }
        y += 4.;
    }
    back(Rect::new(365., 315., 110., 44.));
    if state.high_contrast {
        draw_rectangle_lines(365., 315., 110., 44., 3., WHITE);
    }
}
pub fn credits_clicks(p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(365., 315., 110., 44.), p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}

pub fn draw_help(state: &AppState) {
    panel(
        Rect::new(20., 12., 804., 365.),
        crate::theme::BACKGROUND_DEEP,
    );
    text(
        "HOW TO PLAY",
        40.,
        52.,
        crate::accessibility::text_size(26., state.large_text),
        crate::theme::BRASS,
    );
    let mut y = 92.;
    for (index, paragraph) in crate::help_data::PARAGRAPHS.iter().enumerate() {
        let size = crate::accessibility::text_size(14., state.large_text);
        for line in macroquad_toolkit::ui::wrap_text(paragraph, 760., size) {
            text(
                &line,
                40.,
                y,
                if index == 0 {
                    crate::accessibility::text_size(16., state.large_text)
                } else {
                    size
                },
                if index == 0 || state.high_contrast {
                    WHITE
                } else {
                    Color::new(0.75, 0.70, 0.84, 1.)
                },
            );
            y += size + 8.;
        }
        y += 5.;
    }
    for (rect, label) in [
        Rect::new(300., 288., 110., 44.),
        Rect::new(430., 288., 110., 44.),
        Rect::new(555., 288., 110., 44.),
        Rect::new(680., 288., 130., 44.),
    ]
    .into_iter()
    .zip(crate::help_data::NAV_LABELS)
    {
        panel(rect, crate::theme::SURFACE);
        text(
            label,
            rect.x + 30.,
            rect.y + 29.,
            crate::accessibility::text_size(11., state.large_text),
            WHITE,
        );
    }
}
pub fn help_clicks(p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(300., 288., 110., 44.), p) {
        vec![UiAction::Tutorials]
    } else if crate::ui::hit(Rect::new(430., 288., 110., 44.), p) {
        vec![UiAction::Rules]
    } else if crate::ui::hit(Rect::new(555., 288., 110., 44.), p) {
        vec![UiAction::Credits]
    } else if crate::ui::hit(Rect::new(680., 288., 130., 44.), p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}

pub fn draw_settings(state: &AppState) {
    let labels = crate::settings_data::accessibility_labels(state);
    let summary = crate::collection_summary::from_state(state);
    panel(
        Rect::new(20., 10., 804., 370.),
        crate::theme::BACKGROUND_DEEP,
    );
    text("SETTINGS", 40., 45., 25., crate::theme::BRASS);
    text(
        &format!(
            "Profile: {}  -  Cosmetics: {}/{}",
            state.profile_name,
            cosmetics::total_unlocked(summary.stamps),
            cosmetics::total_options()
        ),
        220.,
        43.,
        13.,
        WHITE,
    );
    text(
        &summary.progress_label(),
        220.,
        59.,
        10.,
        crate::theme::SECONDARY,
    );
    panel(Rect::new(450., 268., 160., 44.), crate::theme::SURFACE_DARK);
    text("EDIT NAME", 500., 296., 11., WHITE);
    for (index, row) in crate::settings_data::cosmetic_rows(state)
        .into_iter()
        .enumerate()
    {
        let rect = Rect::new(40., 68. + index as f32 * 48., 370., 44.);
        panel(rect, Color::new(0.16, 0.11, 0.24, 1.));
        text(row.kind.label(), rect.x + 12., rect.y + 20., 11., WHITE);
        text(
            row.option.name,
            rect.x + 190.,
            rect.y + 20.,
            11.,
            crate::theme::BRASS,
        );
        text(
            &format!(
                "{} / {} OPEN  ·  {}",
                row.unlocked,
                row.total,
                crate::settings_data::next_label(row)
            ),
            rect.x + 190.,
            rect.y + 36.,
            8.,
            crate::theme::SECONDARY,
        );
    }
    panel(Rect::new(450., 68., 160., 44.), crate::theme::SURFACE);
    text(
        &format!("SOUND {}", labels.sound.to_uppercase()),
        495.,
        96.,
        11.,
        WHITE,
    );
    panel(Rect::new(630., 68., 160., 44.), crate::theme::SURFACE);
    text(
        &format!("MOTION {}", labels.motion.to_uppercase()),
        670.,
        96.,
        11.,
        WHITE,
    );
    panel(
        Rect::new(450., 120., 160., 44.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("SAVE NOW", 500., 148., 11., WHITE);
    panel(Rect::new(630., 120., 160., 44.), crate::theme::SURFACE);
    text("LOAD", 690., 148., 11., WHITE);
    panel(
        Rect::new(450., 172., 160., 44.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!("CONTRAST {}", labels.contrast.to_uppercase()),
        475.,
        200.,
        10.,
        WHITE,
    );
    panel(
        Rect::new(630., 172., 160., 44.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!("LARGE TEXT {}", labels.text.to_uppercase()),
        650.,
        200.,
        10.,
        WHITE,
    );
    panel(Rect::new(450., 220., 340., 44.), crate::theme::SURFACE);
    text(
        &format!(
            "VOLUME {} {}",
            labels.volume.to_uppercase(),
            crate::settings_data::volume_meter(state)
        ),
        570.,
        248.,
        11.,
        WHITE,
    );
    panel(Rect::new(40., 268., 160., 44.), crate::theme::MOSS_DARK);
    text("BACK", 98., 297., 12., WHITE);
    panel(
        Rect::new(220., 268., 160., 44.),
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
            crate::theme::CREAM,
        );
        panel(
            Rect::new(285., 242., 110., 44.),
            Color::new(0.22, 0.18, 0.35, 1.),
        );
        text("CANCEL", 315., 270., 11., WHITE);
        panel(
            Rect::new(455., 242., 110., 44.),
            Color::new(0.45, 0.20, 0.24, 1.),
        );
        text("RESET", 490., 270., 11., WHITE);
    }
}
pub fn settings_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if state.confirm_reset {
        if crate::ui::hit(Rect::new(285., 242., 110., 44.), p) {
            return vec![UiAction::CancelResetData];
        }
        if crate::ui::hit(Rect::new(455., 242., 110., 44.), p) {
            return vec![UiAction::ConfirmResetData];
        }
        return vec![];
    }
    for (index, rect) in [
        Rect::new(40., 68., 370., 44.),
        Rect::new(40., 116., 370., 44.),
        Rect::new(40., 164., 370., 44.),
        Rect::new(40., 212., 370., 44.),
    ]
    .into_iter()
    .enumerate()
    {
        if crate::ui::hit(rect, p) {
            if let Some(action) = crate::settings_data::cosmetic_action(index) {
                return vec![action];
            }
        }
    }
    if crate::ui::hit(Rect::new(450., 68., 160., 44.), p) {
        return vec![UiAction::ToggleSound];
    }
    if crate::ui::hit(Rect::new(630., 68., 160., 44.), p) {
        return vec![UiAction::ToggleMotion];
    }
    if crate::ui::hit(Rect::new(450., 220., 340., 44.), p) {
        return vec![UiAction::CycleSoundVolume];
    }
    if crate::ui::hit(Rect::new(450., 172., 160., 44.), p) {
        return vec![UiAction::ToggleHighContrast];
    }
    if crate::ui::hit(Rect::new(630., 172., 160., 44.), p) {
        return vec![UiAction::ToggleLargeText];
    }
    if crate::ui::hit(Rect::new(450., 120., 160., 44.), p) {
        return vec![UiAction::Save];
    }
    if crate::ui::hit(Rect::new(630., 120., 160., 44.), p) {
        return vec![UiAction::Load];
    }
    if crate::ui::hit(Rect::new(40., 268., 160., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(Rect::new(220., 268., 160., 44.), p) {
        return vec![UiAction::ResetData];
    }
    if crate::ui::hit(Rect::new(450., 268., 160., 44.), p) {
        return vec![UiAction::Profile];
    }
    vec![]
}
