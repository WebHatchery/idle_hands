//! Shared, resume-safe result surfaces for the cabinet games.

use crate::{state::AppState, ui::UiAction};
use macroquad::prelude::*;

#[path = "game_result_entries.rs"]
mod entries;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultKind {
    Won,
    Lost,
    Stuck,
    Draw,
    Push,
}

#[derive(Debug, Clone)]
pub struct ResultInfo {
    pub kind: ResultKind,
    pub context: String,
    pub explanation: String,
    pub stats: String,
    pub primary_label: &'static str,
    pub primary_action: UiAction,
    pub secondary_label: &'static str,
    pub secondary_action: UiAction,
}

pub fn info(state: &AppState) -> Option<ResultInfo> {
    entries::info(state)
}

// Terminal surfaces always need a result kind, two copy lines, and two
// independently labelled actions. Keep that compact content contract flat.
#[allow(clippy::too_many_arguments)]
pub(super) fn make<T>(
    state: &AppState,
    _game: &T,
    kind: ResultKind,
    explanation: impl Into<String>,
    stats: String,
    primary_action: UiAction,
    primary_label: &'static str,
    secondary_action: UiAction,
    secondary_label: &'static str,
) -> ResultInfo {
    let game = state
        .screen
        .game()
        .expect("result info is only for game screens");
    ResultInfo {
        kind,
        context: format!(
            "{}  ·  {}",
            crate::game_descriptor::descriptor(game).title,
            crate::game_variants::label(state, game)
        ),
        explanation: explanation.into(),
        stats,
        primary_label,
        primary_action,
        secondary_label,
        secondary_action,
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Option<Vec<UiAction>> {
    let result = info(state)?;
    let layout = layout();
    if crate::ui::hit(layout.primary, point) {
        return Some(vec![result.primary_action]);
    }
    if crate::ui::hit(layout.secondary, point) {
        return Some(vec![result.secondary_action]);
    }
    Some(Vec::new())
}

pub fn draw(state: &AppState) {
    let Some(result) = info(state) else {
        return;
    };
    let layout = layout();
    let tone = match result.kind {
        ResultKind::Won => crate::theme::BRASS,
        ResultKind::Lost | ResultKind::Stuck => Color::new(1., 0.52, 0.28, 1.),
        ResultKind::Draw | ResultKind::Push => Color::new(0.68, 0.82, 1., 1.),
    };
    let headline = match result.kind {
        ResultKind::Won => "YOU WON",
        ResultKind::Lost => "GAME OVER",
        ResultKind::Stuck => "NO MOVES",
        ResultKind::Draw => "DRAW",
        ResultKind::Push => "PUSH",
    };
    draw_rectangle(
        0.,
        0.,
        layout.width,
        layout.height,
        Color::new(0.02, 0.03, 0.06, 0.78),
    );
    draw_rectangle(
        layout.panel.x,
        layout.panel.y,
        layout.panel.w,
        layout.panel.h,
        Color::new(0.10, 0.12, 0.17, 0.98),
    );
    draw_rectangle_lines(
        layout.panel.x,
        layout.panel.y,
        layout.panel.w,
        layout.panel.h,
        3.,
        tone,
    );
    centered(
        headline,
        layout.panel,
        layout.panel.y + layout.title_offset,
        layout.title_size,
        tone,
    );
    centered(
        &result.context,
        layout.panel,
        layout.panel.y + layout.context_offset,
        layout.context_size,
        crate::theme::CREAM,
    );
    centered_wrapped(
        &result.explanation,
        layout.panel,
        layout.panel.y + layout.explanation_offset,
        layout.body_size,
        crate::theme::SECONDARY,
        layout.text_width,
    );
    centered_wrapped(
        &result.stats,
        layout.panel,
        layout.panel.y + layout.stats_offset,
        layout.stats_size,
        crate::theme::CREAM,
        layout.text_width,
    );
    button(layout.primary, result.primary_label, tone);
    button(
        layout.secondary,
        result.secondary_label,
        crate::theme::SURFACE,
    );
}

#[derive(Clone, Copy)]
struct Layout {
    width: f32,
    height: f32,
    panel: Rect,
    primary: Rect,
    secondary: Rect,
    title_offset: f32,
    context_offset: f32,
    explanation_offset: f32,
    stats_offset: f32,
    title_size: f32,
    context_size: f32,
    body_size: f32,
    stats_size: f32,
    text_width: f32,
}

fn layout() -> Layout {
    let (width, height) = crate::ui::layout_size();
    if crate::ui::is_portrait() {
        let panel = Rect::new(14., 94., width - 28., height - 150.);
        let button_width = (panel.w - 36.) * 0.5;
        Layout {
            width,
            height,
            panel,
            primary: Rect::new(panel.x + 12., panel.bottom() - 68., button_width, 50.),
            secondary: Rect::new(
                panel.x + 24. + button_width,
                panel.bottom() - 68.,
                button_width,
                50.,
            ),
            title_offset: 58.,
            context_offset: 92.,
            explanation_offset: 145.,
            stats_offset: 210.,
            title_size: 28.,
            context_size: 11.,
            body_size: 15.,
            stats_size: 12.,
            text_width: panel.w - 28.,
        }
    } else if crate::ui::is_compact_landscape() {
        let panel = Rect::new(22., 30., width - 44., height - 60.);
        let button_width = (panel.w - 36.) * 0.5;
        Layout {
            width,
            height,
            panel,
            primary: Rect::new(panel.x + 12., panel.bottom() - 62., button_width, 46.),
            secondary: Rect::new(
                panel.x + 24. + button_width,
                panel.bottom() - 62.,
                button_width,
                46.,
            ),
            title_offset: 42.,
            context_offset: 70.,
            explanation_offset: 99.,
            stats_offset: 138.,
            title_size: 29.,
            context_size: 11.,
            body_size: 12.,
            stats_size: 11.,
            text_width: panel.w - 28.,
        }
    } else {
        let panel = Rect::new(230., 92., width - 460., 535.);
        let button_width = (panel.w - 56.) * 0.5;
        Layout {
            width,
            height,
            panel,
            primary: Rect::new(panel.x + 20., panel.bottom() - 76., button_width, 54.),
            secondary: Rect::new(
                panel.x + 36. + button_width,
                panel.bottom() - 76.,
                button_width,
                54.,
            ),
            title_offset: 68.,
            context_offset: 111.,
            explanation_offset: 162.,
            stats_offset: 224.,
            title_size: 42.,
            context_size: 14.,
            body_size: 20.,
            stats_size: 16.,
            text_width: panel.w - 60.,
        }
    }
}

fn centered(value: &str, panel: Rect, y: f32, size: f32, color: Color) {
    let readable = crate::ui::readable_text_size(size);
    let width = crate::ui::measure_text(value, None, readable as u16, 1.).width;
    crate::ui::draw_text(value, panel.center().x - width * 0.5, y, size, color);
}

fn centered_wrapped(value: &str, panel: Rect, y: f32, size: f32, color: Color, max_width: f32) {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in value.split_whitespace() {
        let candidate = if current.is_empty() {
            word.to_owned()
        } else {
            format!("{} {}", current, word)
        };
        if !current.is_empty()
            && crate::ui::measure_text(&candidate, None, size as u16, 1.).width > max_width
        {
            lines.push(current);
            current = word.to_owned();
        } else {
            current = candidate;
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    for (index, line) in lines.iter().take(2).enumerate() {
        centered(line, panel, y + index as f32 * (size + 5.), size, color);
    }
}

fn button(rect: Rect, label: &str, border: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., border);
    let size = if crate::ui::is_portrait() { 13. } else { 16. };
    let width = crate::ui::measure_text(label, None, size as u16, 1.).width;
    crate::ui::draw_text(
        label,
        rect.x + (rect.w - width) * 0.5,
        rect.y + rect.h * 0.64,
        size,
        WHITE,
    );
}
