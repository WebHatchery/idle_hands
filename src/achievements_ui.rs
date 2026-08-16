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
            back: Rect::new(700., 330., 110., 38.),
            columns: 5,
            card_w: 145.,
            card_h: 24.,
            origin: vec2(35., 70.),
            gap_x: 155.,
            gap_y: 31.,
        }
    } else if crate::ui::is_portrait() {
        Layout {
            panel: Rect::new(8., 38., 344., 602.),
            back: Rect::new(10., 650., 150., 38.),
            columns: 3,
            card_w: 106.,
            card_h: 22.,
            origin: vec2(14., 145.),
            gap_x: 112.,
            gap_y: 29.,
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
            gap_y: 38.,
        }
    }
}

pub fn clicks(point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if l.back.contains(point) {
        vec![UiAction::Records]
    } else if let Some(filter) = filter_rects(l).iter().position(|rect| rect.contains(point)) {
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
            "{} earned of {}  •  showing {}",
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
    }
    for (slot, achievement) in AchievementId::ALL
        .iter()
        .filter(|achievement| visible(**achievement, state.achievement_filter, state))
        .enumerate()
    {
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
                7.
            } else if compact {
                9.
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
            rect.right() - if portrait { 39. } else { 47. },
            rect.y + rect.h * 0.64,
            crate::accessibility::text_size(if portrait { 5. } else { 7. }, state.large_text),
            if state.high_contrast {
                WHITE
            } else if earned {
                Color::new(0.55, 1., 0.72, 1.)
            } else {
                Color::new(0.55, 0.50, 0.64, 1.)
            },
        );
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
            Rect::new(500., 40., 60., 22.),
            Rect::new(565., 40., 90., 22.),
            Rect::new(660., 40., 90., 22.),
        ]
    } else if crate::ui::is_portrait() {
        [
            Rect::new(14., 112., 106., 26.),
            Rect::new(126., 112., 106., 26.),
            Rect::new(238., 112., 106., 26.),
        ]
    } else {
        [
            Rect::new(layout.origin.x, 170., 130., 32.),
            Rect::new(layout.origin.x + 140., 170., 130., 32.),
            Rect::new(layout.origin.x + 280., 170., 130., 32.),
        ]
    }
}

fn fitted_label(achievement: AchievementId, portrait: bool) -> String {
    let label = achievement_label(achievement);
    let limit = if portrait { 10 } else { 18 };
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
