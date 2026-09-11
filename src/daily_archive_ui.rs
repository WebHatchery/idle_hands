//! Responsive history shelf for completed and attempted daily routes.

use crate::{state::AppState, ui::UiAction};
use macroquad::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy)]
struct Layout {
    panel: Rect,
    back: Rect,
    previous: Rect,
    next: Rect,
    columns: usize,
    row_size: Vec2,
    origin: Vec2,
    gap: Vec2,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            panel: Rect::new(20., 12., 804., 365.),
            back: Rect::new(680., 330., 130., 44.),
            previous: Rect::new(430., 330., 100., 44.),
            next: Rect::new(545., 330., 100., 44.),
            columns: 2,
            row_size: vec2(370., 44.),
            origin: vec2(35., 82.),
            gap: vec2(380., 52.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            panel: Rect::new(8., 38., 344., 602.),
            back: Rect::new(10., 650., 150., 44.),
            previous: Rect::new(10., 596., 100., 44.),
            next: Rect::new(250., 596., 100., 44.),
            columns: 1,
            row_size: vec2(324., 48.),
            origin: vec2(18., 128.),
            gap: vec2(0., 54.),
        }
    } else {
        Layout {
            panel: Rect::new(120., 55., 1040., 610.),
            back: Rect::new(930., 590., 180., 48.),
            previous: Rect::new(700., 590., 100., 48.),
            next: Rect::new(815., 590., 100., 48.),
            columns: 2,
            row_size: vec2(450., 48.),
            origin: vec2(160., 215.),
            gap: vec2(470., 60.),
        }
    }
}

pub fn page_size() -> usize {
    let layout = layout();
    layout.columns
        * if crate::ui::is_portrait() {
            7
        } else if crate::ui::is_compact_landscape() {
            3
        } else {
            5
        }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let layout = layout();
    let total = state.records.daily_results.len();
    let start =
        crate::daily_archive_data::page_start(total, state.daily_archive_scroll, page_size());
    if layout.back.contains(point) {
        return vec![UiAction::Records];
    }
    let step = page_size() as i8;
    if layout.previous.contains(point) && start > 0 {
        return vec![UiAction::DailyArchiveScroll(-step)];
    }
    if layout.next.contains(point) && start + page_size() < total {
        return vec![UiAction::DailyArchiveScroll(step)];
    }
    for (slot, row) in crate::daily_archive_data::page_rows(state, start, page_size())
        .into_iter()
        .enumerate()
    {
        if archive_rect(layout, slot).contains(point) {
            return vec![UiAction::DailyArchiveOpen(row.day)];
        }
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let layout = layout();
    panel(
        layout.panel,
        crate::theme::BACKGROUND_DEEP,
        state.high_contrast,
    );
    let portrait = crate::ui::is_portrait();
    let compact = crate::ui::is_compact_landscape();
    let title_size = if portrait { 28. } else { 38. };
    let title_y = if compact {
        layout.panel.y + 35.
    } else if portrait {
        layout.panel.y + 42.
    } else {
        layout.panel.y + 70.
    };
    let subtitle_y = if compact {
        layout.panel.y + 52.
    } else if portrait {
        layout.panel.y + 66.
    } else {
        layout.panel.y + 98.
    };
    crate::ui::draw_text(
        "DAILY ARCHIVE",
        layout.panel.x + 50.,
        title_y,
        crate::accessibility::text_size(title_size, state.large_text),
        crate::theme::BRASS,
    );
    crate::ui::draw_text(
        format!(
            "{} routes logged  ·  {} cleared  ·  best {}",
            state.records.daily_results.len(),
            state.records.daily_clear_count(),
            state
                .records
                .daily_best_score()
                .map_or_else(|| "—".into(), |score| score.to_string())
        ),
        layout.panel.x + 52.,
        subtitle_y,
        crate::accessibility::text_size(if portrait { 10. } else { 15. }, state.large_text),
        crate::theme::SECONDARY,
    );

    let total = state.records.daily_results.len();
    let start = state
        .daily_archive_scroll
        .min(total.saturating_sub(page_size()));
    for (slot, result) in crate::daily_archive_data::page_rows(state, start, page_size())
        .into_iter()
        .enumerate()
    {
        let rect = archive_rect(layout, slot);
        panel(
            rect,
            if result.won {
                Color::new(0.20, 0.15, 0.28, 1.)
            } else {
                Color::new(0.12, 0.09, 0.19, 1.)
            },
            state.high_contrast,
        );
        let label = format!(
            "{} · {}",
            crate::daily_challenge::label(result.day, result.challenge),
            result.rule.label()
        );
        crate::ui::draw_text(
            label,
            rect.x + 14.,
            rect.y + rect.h * 0.48,
            crate::accessibility::text_size(if portrait { 12. } else { 11. }, state.large_text),
            if state.high_contrast {
                WHITE
            } else {
                crate::theme::BRASS
            },
        );
        crate::ui::draw_text(
            if result.won { "CLEARED" } else { "ATTEMPTED" },
            rect.x + 14.,
            rect.y + rect.h - 7.,
            crate::accessibility::text_size(if portrait { 9. } else { 8. }, state.large_text),
            if result.won {
                Color::new(0.55, 1., 0.72, 1.)
            } else {
                crate::theme::SECONDARY
            },
        );
        crate::ui::draw_text(
            crate::daily_archive_data::action_label(),
            rect.right() - if portrait { 52. } else { 58. },
            rect.y + rect.h - 7.,
            crate::accessibility::text_size(if portrait { 9. } else { 8. }, state.large_text),
            crate::theme::SECONDARY,
        );
        crate::ui::draw_text(
            format!("SCORE {}", result.score),
            rect.right() - if portrait { 90. } else { 82. },
            rect.y + rect.h * 0.60,
            crate::accessibility::text_size(if portrait { 10. } else { 9. }, state.large_text),
            WHITE,
        );
    }
    if total == 0 {
        crate::ui::draw_text(
            "No routes have been recorded yet.",
            layout.origin.x,
            layout.origin.y + 30.,
            crate::accessibility::text_size(14., state.large_text),
            crate::theme::SECONDARY,
        );
    }
    let window = crate::daily_archive_data::window_label(start, total, page_size());
    archive_button(layout.previous, "PREV", start > 0, state.high_contrast);
    archive_button(
        layout.next,
        "NEXT",
        start + page_size() < total,
        state.high_contrast,
    );
    crate::ui::draw_text(
        window,
        layout.previous.right() + 14.,
        layout.previous.y + 29.,
        11.,
        crate::theme::CREAM,
    );
    panel(layout.back, crate::theme::MOSS_DARK, state.high_contrast);
    crate::ui::draw_text(
        "BACK",
        layout.back.x + if portrait { 52. } else { 60. },
        layout.back.y + layout.back.h * 0.64,
        if portrait { 12. } else { 18. },
        WHITE,
    );
}

fn archive_rect(layout: Layout, slot: usize) -> Rect {
    Rect::new(
        layout.origin.x + (slot % layout.columns) as f32 * layout.gap.x,
        layout.origin.y + (slot / layout.columns) as f32 * layout.gap.y,
        layout.row_size.x,
        layout.row_size.y,
    )
}

fn panel(rect: Rect, fill: Color, high_contrast: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.,
        crate::accessibility::grid_line(high_contrast),
    );
}

fn archive_button(rect: Rect, label: &str, enabled: bool, high_contrast: bool) {
    panel(
        rect,
        if enabled {
            crate::theme::SURFACE_DARK
        } else {
            crate::theme::BACKGROUND_DEEP
        },
        high_contrast,
    );
    crate::ui::draw_text(
        label,
        rect.x + 20.,
        rect.y + 29.,
        11.,
        if enabled {
            WHITE
        } else {
            crate::theme::SECONDARY
        },
    );
}
