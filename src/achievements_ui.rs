//! Responsive achievement shelf opened from the Records screen.

use crate::{progression::AchievementId, state::AppState, ui::UiAction};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    panel: Rect,
    back: Rect,
    columns: usize,
    card_w: f32,
    card_h: f32,
    origin: Vec2,
    gap_x: f32,
    gap_y: f32,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            panel: Rect::new(20., 12., 804., 365.),
            back: Rect::new(700., 330., 110., 44.),
            columns: 2,
            card_w: 360.,
            card_h: 44.,
            origin: vec2(35., 70.),
            gap_x: 380.,
            gap_y: 50.,
        }
    } else if crate::ui::is_portrait() {
        Layout {
            panel: Rect::new(8., 38., 344., 602.),
            back: Rect::new(10., 714., 150., 44.),
            columns: 1,
            card_w: 324.,
            card_h: 54.,
            origin: vec2(18., 155.),
            gap_x: 0.,
            gap_y: 60.,
        }
    } else {
        Layout {
            panel: Rect::new(120., 55., 1040., 610.),
            back: Rect::new(930., 590., 180., 48.),
            columns: 5,
            card_w: 185.,
            card_h: 34.,
            origin: vec2(160., 215.),
            gap_x: 205.,
            gap_y: 35.,
        }
    }
}

pub fn clicks(point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if l.back.contains(point) {
        return vec![UiAction::Records];
    }
    if let Some((previous, next)) = scroll_rects() {
        if previous.contains(point) {
            return vec![UiAction::LibraryScroll(-1)];
        }
        if next.contains(point) {
            return vec![UiAction::LibraryScroll(1)];
        }
    }
    if let Some(filter) = filter_rects(l).iter().position(|rect| rect.contains(point)) {
        vec![UiAction::AchievementFilter(filter as u8)]
    } else {
        Vec::new()
    }
}

pub fn draw(state: &AppState) {
    let l = layout();
    panel(
        l.panel,
        Color::new(0.08, 0.06, 0.14, 1.),
        state.high_contrast,
    );
    let portrait = crate::ui::is_portrait();
    let compact = crate::ui::is_compact_landscape();
    let title_y = if compact {
        l.panel.y + 35.
    } else if portrait {
        l.panel.y + 42.
    } else {
        l.panel.y + 70.
    };
    let subtitle_y = if compact {
        l.panel.y + 52.
    } else if portrait {
        l.panel.y + 66.
    } else {
        l.panel.y + 98.
    };
    let title_size = if portrait { 28. } else { 38. };
    draw_text(
        "ACHIEVEMENTS",
        l.panel.x + 50.,
        title_y,
        crate::accessibility::text_size(title_size, state.large_text),
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    let earned = AchievementId::ALL
        .iter()
        .filter(|achievement| {
            state
                .achievements
                .get(achievement.index())
                .copied()
                .unwrap_or(false)
        })
        .count();
    draw_text(
        format!(
            "{} earned of {}  -  showing {}",
            earned,
            AchievementId::ALL.len(),
            filter_label(state.achievement_filter)
        ),
        l.panel.x + 52.,
        subtitle_y,
        crate::accessibility::text_size(if portrait { 10. } else { 15. }, state.large_text),
        Color::new(0.72, 0.68, 0.82, 1.),
    );
    for (filter, rect) in filter_rects(l).iter().enumerate() {
        let active = state.achievement_filter == filter as u8;
        panel(
            *rect,
            if active {
                Color::new(0.25, 0.17, 0.34, 1.)
            } else {
                Color::new(0.13, 0.10, 0.20, 1.)
            },
            state.high_contrast,
        );
        draw_text(
            filter_label(filter as u8),
            rect.x + if portrait { 10. } else { 16. },
            rect.y + rect.h * 0.68,
            crate::accessibility::text_size(if portrait { 8. } else { 10. }, state.large_text),
            if active {
                WHITE
            } else {
                Color::new(0.72, 0.68, 0.82, 1.)
            },
        );
        if active {
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 3., WHITE);
        }
    }
    for (slot, achievement) in visible_achievements(state).iter().enumerate() {
        let rect = card_rect(l, slot);
        let earned = state
            .achievements
            .get(achievement.index())
            .copied()
            .unwrap_or(false);
        let fill = if state.high_contrast {
            if earned {
                Color::new(0.28, 0.28, 0.30, 1.)
            } else {
                Color::new(0.02, 0.02, 0.03, 1.)
            }
        } else if earned {
            Color::new(0.20, 0.15, 0.28, 1.)
        } else {
            Color::new(0.12, 0.09, 0.19, 1.)
        };
        panel(rect, fill, state.high_contrast);
        draw_circle(
            rect.x + 10.,
            rect.y + rect.h * 0.5,
            if portrait { 3. } else { 4. },
            if state.high_contrast {
                WHITE
            } else if earned {
                Color::new(0.98, 0.75, 0.30, 1.)
            } else {
                Color::new(0.35, 0.31, 0.44, 1.)
            },
        );
        let size = crate::accessibility::text_size(
            if portrait {
                13.
            } else if compact {
                12.
            } else {
                10.
            },
            state.large_text,
        );
        draw_text(
            fitted_label(*achievement, portrait),
            rect.x + 20.,
            rect.y + rect.h * 0.64,
            size,
            if state.high_contrast {
                WHITE
            } else if earned {
                Color::new(0.98, 0.82, 0.42, 1.)
            } else {
                Color::new(0.68, 0.64, 0.76, 1.)
            },
        );
        let status = if earned { "EARNED" } else { "LOCKED" };
        draw_text(
            status,
            rect.right() - if portrait { 58. } else { 55. },
            rect.y + rect.h * 0.64,
            crate::accessibility::text_size(if portrait { 9. } else { 8. }, state.large_text),
            if state.high_contrast {
                WHITE
            } else if earned {
                Color::new(0.55, 1., 0.72, 1.)
            } else {
                Color::new(0.55, 0.50, 0.64, 1.)
            },
        );
    }
    if let Some((previous, next)) = scroll_rects() {
        panel(
            previous,
            Color::new(0.18, 0.12, 0.28, 1.),
            state.high_contrast,
        );
        panel(next, Color::new(0.18, 0.12, 0.28, 1.), state.high_contrast);
        draw_text("PREV", previous.x + 22., previous.y + 28., 11., WHITE);
        draw_text("NEXT", next.x + 22., next.y + 28., 11., WHITE);
    }
    panel(
        l.back,
        Color::new(0.25, 0.16, 0.32, 1.),
        state.high_contrast,
    );
    draw_text(
        "BACK",
        l.back.x + if portrait { 52. } else { 60. },
        l.back.y + l.back.h * 0.64,
        if portrait { 12. } else { 18. },
        WHITE,
    );
}

fn achievement_label(achievement: AchievementId) -> String {
    match achievement {
        AchievementId::Game(game) => game.title().to_owned(),
        _ => achievement.title().to_owned(),
    }
}

fn visible(achievement: AchievementId, filter: u8, state: &AppState) -> bool {
    match filter {
        1 => state
            .achievements
            .get(achievement.index())
            .copied()
            .unwrap_or(false),
        2 => !state
            .achievements
            .get(achievement.index())
            .copied()
            .unwrap_or(false),
        _ => true,
    }
}

fn filter_label(filter: u8) -> &'static str {
    match filter {
        1 => "EARNED",
        2 => "LOCKED",
        _ => "ALL",
    }
}

fn filter_rects(layout: Layout) -> [Rect; 3] {
    if crate::ui::is_compact_landscape() {
        [
            Rect::new(500., 2., 60., 44.),
            Rect::new(565., 2., 90., 44.),
            Rect::new(660., 2., 90., 44.),
        ]
    } else if crate::ui::is_portrait() {
        [
            Rect::new(14., 108., 106., 44.),
            Rect::new(126., 108., 106., 44.),
            Rect::new(238., 108., 106., 44.),
        ]
    } else {
        [
            Rect::new(layout.origin.x, 170., 130., 44.),
            Rect::new(layout.origin.x + 140., 170., 130., 44.),
            Rect::new(layout.origin.x + 280., 170., 130., 44.),
        ]
    }
}

fn fitted_label(achievement: AchievementId, portrait: bool) -> String {
    let label = achievement_label(achievement);
    let limit = if portrait { 28 } else { 24 };
    if label.chars().count() <= limit {
        label
    } else {
        format!(
            "{}..",
            label
                .chars()
                .take(limit.saturating_sub(2))
                .collect::<String>()
        )
    }
}

fn card_rect(layout: Layout, slot: usize) -> Rect {
    Rect::new(
        layout.origin.x + (slot % layout.columns) as f32 * layout.gap_x,
        layout.origin.y + (slot / layout.columns) as f32 * layout.gap_y,
        layout.card_w,
        layout.card_h,
    )
}

fn visible_achievements(state: &AppState) -> Vec<AchievementId> {
    let matches: Vec<_> = AchievementId::ALL
        .iter()
        .copied()
        .filter(|achievement| visible(*achievement, state.achievement_filter, state))
        .collect();
    let capacity = if crate::ui::is_portrait() {
        8
    } else if crate::ui::is_compact_landscape() {
        10
    } else {
        matches.len()
    };
    let first = state
        .library_scroll
        .min(matches.len().saturating_sub(capacity));
    matches.into_iter().skip(first).take(capacity).collect()
}

fn scroll_rects() -> Option<(Rect, Rect)> {
    if crate::ui::is_portrait() {
        Some((
            Rect::new(10., 650., 100., 44.),
            Rect::new(250., 650., 100., 44.),
        ))
    } else if crate::ui::is_compact_landscape() {
        Some((
            Rect::new(430., 330., 100., 44.),
            Rect::new(545., 330., 100., 44.),
        ))
    } else {
        None
    }
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
