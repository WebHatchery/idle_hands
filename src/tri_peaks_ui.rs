//! Responsive touch presentation for TriPeaks Solitaire.

use crate::{
    accessibility,
    state::AppState,
    tri_peaks::{TriPeaksRule, TriPeaksStatus},
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    base_x: f32,
    top: f32,
    card_w: f32,
    card_h: f32,
    row_gap: f32,
    gap: f32,
    stock: Rect,
    waste: Rect,
    hint: Rect,
    undo: Rect,
    new_game: Rect,
    bridge: Rect,
    rule: Rect,
}

impl Layout {
    fn card_rect(self, index: usize) -> Rect {
        let (row, column) = if index < 3 {
            (0, index)
        } else if index < 9 {
            (1, index - 3)
        } else if index < 18 {
            (2, index - 9)
        } else {
            (3, index - 18)
        };
        let step = self.card_w + self.gap;
        let x = match row {
            0 => self.base_x + column as f32 * step * 3.,
            1 => self.base_x + (column / 2) as f32 * step * 3. + (column % 2) as f32 * step,
            2 => self.base_x + (column / 3) as f32 * step * 3. + (column % 3) as f32 * step,
            _ => self.base_x - step * 0.5 + column as f32 * step,
        };
        Rect::new(
            x,
            self.top + row as f32 * self.row_gap,
            self.card_w,
            self.card_h,
        )
    }
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            base_x: 92.,
            top: 78.,
            card_w: 54.,
            card_h: 64.,
            row_gap: 31.,
            gap: 4.,
            stock: Rect::new(650., 72., 62., 62.),
            waste: Rect::new(722., 72., 62., 62.),
            hint: Rect::new(570., 280., 80., 40.),
            undo: Rect::new(658., 280., 80., 40.),
            new_game: Rect::new(746., 280., 90., 40.),
            bridge: Rect::new(650., 140., 62., 40.),
            rule: Rect::new(720., 140., 116., 40.),
        }
    } else if crate::ui::is_portrait() {
        let width = crate::ui::display_width();
        let bottom = crate::ui::display_height() - 54.;
        let bottom_button_w = (width - 32.) * 0.5;
        Layout {
            base_x: if width < 350. { 9. } else { 20. },
            top: 158.,
            card_w: 30.,
            card_h: 48.,
            row_gap: 25.,
            gap: 3.,
            stock: Rect::new(8., 78., 46., 64.),
            waste: Rect::new(60., 78., 46., 64.),
            hint: Rect::new(112., 78., 60., 64.),
            undo: Rect::new(10., bottom, bottom_button_w, 42.),
            new_game: Rect::new(22. + bottom_button_w, bottom, bottom_button_w, 42.),
            bridge: Rect::new(178., 100., 64., 50.),
            rule: Rect::new(248., 100., if width < 350. { 64. } else { 76. }, 50.),
        }
    } else {
        Layout {
            base_x: 360.,
            top: 115.,
            card_w: 58.,
            card_h: 82.,
            row_gap: 45.,
            gap: 5.,
            stock: Rect::new(970., 120., 82., 100.),
            waste: Rect::new(1070., 120., 82., 100.),
            hint: Rect::new(970., 250., 82., 44.),
            undo: Rect::new(1070., 250., 82., 44.),
            new_game: Rect::new(970., 370., 182., 44.),
            bridge: Rect::new(970., 310., 82., 44.),
            rule: Rect::new(1070., 310., 82., 44.),
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(back_rect(), point) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(l.stock, point) {
        return vec![UiAction::TriPeaksStock];
    }
    if l.waste.contains(point) {
        return vec![];
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::TriPeaksHint];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::TriPeaksUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::TriPeaksNew];
    }
    if crate::ui::hit(l.bridge, point) {
        return vec![UiAction::TriPeaksBridge];
    }
    if crate::ui::hit(l.rule, point) {
        return vec![UiAction::TriPeaksRule(match _state.games.tri_peaks.rule {
            TriPeaksRule::Strict => TriPeaksRule::Wrap,
            TriPeaksRule::Wrap => TriPeaksRule::Strict,
        })];
    }
    for index in (0..28).rev() {
        if l.card_rect(index).contains(point) {
            return vec![UiAction::TriPeaksTap(index)];
        }
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.tri_peaks;
    let portrait = crate::ui::is_portrait();
    let compact = crate::ui::is_compact_landscape();
    let title_x = if compact {
        90.
    } else if portrait {
        10.
    } else {
        400.
    };
    let title_y = if compact {
        30.
    } else if portrait {
        48.
    } else {
        72.
    };
    text("‹ CABINET", 8., 30., scaled(13., state), muted());
    text(
        "TRI-PEAKS",
        title_x,
        title_y,
        scaled(if portrait { 22. } else { 28. }, state),
        accent(),
    );
    text(
        if compact {
            compact_status_text(game.status)
        } else {
            status_text(game.status)
        },
        if compact { 280. } else { title_x },
        if compact { 30. } else { title_y + 24. },
        scaled(12., state),
        muted(),
    );
    draw_slot(l.stock, game.stock.last().copied(), state);
    draw_slot(l.waste, game.waste.last().copied(), state);
    text(
        "STOCK",
        l.stock.x,
        l.stock.bottom() + 15.,
        scaled(10., state),
        muted(),
    );
    text(
        "WASTE",
        l.waste.x,
        l.waste.bottom() + 15.,
        scaled(10., state),
        muted(),
    );
    for index in 0..28 {
        if let Some(card) = game.tableau[index] {
            let rect = l.card_rect(index);
            crate::card_render::draw_card_accessible(
                rect,
                card,
                false,
                state.card_back,
                state.reduced_motion,
                state.high_contrast,
                state.large_text,
            );
            if game.can_play(index) {
                draw_rectangle_lines(
                    rect.x + 2.,
                    rect.y + 2.,
                    rect.w - 4.,
                    rect.h - 4.,
                    3.,
                    playable(),
                );
            }
        }
    }
    text(
        &if compact || portrait {
            format!(
                "{} plays • P{} • R{} • B{}",
                game.playable_count(),
                game.points,
                game.run,
                game.bridges
            )
        } else {
            format!(
                "Moves {} • {} plays • Points {} • Run {} (best {}) • Bridge {}",
                game.moves,
                game.playable_count(),
                game.points,
                game.run,
                game.best_run,
                game.bridges
            )
        },
        if portrait { 10. } else { title_x },
        if portrait {
            l.undo.y - 48.
        } else if compact {
            340.
        } else {
            570.
        },
        scaled(12., state),
        muted(),
    );
    let detail = state
        .card_hint
        .as_deref()
        .unwrap_or("Green cards play. Chain clears; STOCK breaks the run.");
    text(
        detail,
        if portrait { 10. } else { title_x },
        if portrait {
            l.undo.y - 24.
        } else if compact {
            362.
        } else {
            595.
        },
        scaled(11., state),
        if state.card_hint.is_some() {
            accent()
        } else {
            muted()
        },
    );
    button(l.hint, "HINT", state.large_text);
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW TRIPEAKS", state.large_text);
    button_active(
        l.bridge,
        &format!("BRIDGE ×{}", game.bridges),
        game.bridge_armed,
        state.large_text,
    );
    button(
        l.rule,
        if portrait {
            match game.rule {
                TriPeaksRule::Strict => "STRICT",
                TriPeaksRule::Wrap => "A↔K",
            }
        } else {
            game.rule.label()
        },
        state.large_text,
    );
}

fn draw_slot(rect: Rect, card: Option<crate::cards::Card>, state: &AppState) {
    if let Some(card) = card {
        crate::card_render::draw_card_accessible(
            rect,
            card,
            false,
            state.card_back,
            state.reduced_motion,
            state.high_contrast,
            state.large_text,
        );
    } else {
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            accessibility::board_fill(state.high_contrast),
        );
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., muted());
    }
}

fn button(rect: Rect, label: &str, large_text: bool) {
    button_active(rect, label, false, large_text);
}

fn button_active(rect: Rect, label: &str, active: bool, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if active { 3. } else { 1. },
        if active { playable() } else { accent() },
    );
    text(
        label,
        rect.x + 6.,
        rect.y + rect.h * 0.62,
        accessibility::text_size(10., large_text),
        if active { playable() } else { WHITE },
    );
}

fn status_text(status: TriPeaksStatus) -> &'static str {
    match status {
        TriPeaksStatus::Playing => "Clear the three peaks",
        TriPeaksStatus::Won => "The peaks are clear",
        TriPeaksStatus::Stuck => "No peak card can play",
    }
}

fn compact_status_text(status: TriPeaksStatus) -> &'static str {
    match status {
        TriPeaksStatus::Playing => "Clear peaks",
        TriPeaksStatus::Won => "Peaks clear",
        TriPeaksStatus::Stuck => "No move",
    }
}

fn scaled(size: f32, state: &AppState) -> f32 {
    accessibility::text_size(size, state.large_text)
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}

fn accent() -> Color {
    crate::theme::BRASS
}

#[cfg(test)]
mod tests;

fn muted() -> Color {
    crate::theme::SECONDARY
}

fn playable() -> Color {
    Color::from_rgba(80, 224, 126, 255)
}

fn back_rect() -> Rect {
    Rect::new(0., 0., 110., 42.)
}
