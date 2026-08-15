//! Responsive touch presentation for TriPeaks Solitaire.

use crate::{accessibility, state::AppState, tri_peaks::TriPeaksStatus, ui::UiAction};
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
    undo: Rect,
    new_game: Rect,
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
            undo: Rect::new(650., 280., 105., 40.),
            new_game: Rect::new(765., 280., 105., 40.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            base_x: 20.,
            top: 158.,
            card_w: 30.,
            card_h: 48.,
            row_gap: 25.,
            gap: 3.,
            stock: Rect::new(18., 78., 46., 64.),
            waste: Rect::new(72., 78., 46., 64.),
            undo: Rect::new(20., 650., 145., 42.),
            new_game: Rect::new(185., 650., 145., 42.),
        }
    } else {
        Layout {
            base_x: 360.,
            top: 115.,
            card_w: 58.,
            card_h: 82.,
            row_gap: 45.,
            gap: 5.,
            stock: Rect::new(930., 120., 82., 100.),
            waste: Rect::new(1030., 120., 82., 100.),
            undo: Rect::new(930., 250., 100., 44.),
            new_game: Rect::new(1045., 250., 125., 44.),
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if back_rect().contains(point) {
        return vec![UiAction::Cabinet];
    }
    if l.stock.contains(point) {
        return vec![UiAction::TriPeaksStock];
    }
    if l.waste.contains(point) {
        return vec![];
    }
    if l.undo.contains(point) {
        return vec![UiAction::TriPeaksUndo];
    }
    if l.new_game.contains(point) {
        return vec![UiAction::TriPeaksNew];
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
    let game = &state.tri_peaks;
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
        status_text(game.status),
        if compact { 650. } else { title_x },
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
            crate::card_render::draw_card_accessible(
                l.card_rect(index),
                card,
                false,
                state.card_back,
                state.reduced_motion,
                state.high_contrast,
                state.large_text,
            );
        }
    }
    text(
        &format!(
            "Moves {}  •  Play one rank above or below the waste",
            game.moves
        ),
        if portrait { 10. } else { title_x },
        if portrait {
            585.
        } else if compact {
            340.
        } else {
            570.
        },
        scaled(12., state),
        muted(),
    );
    text(
        "Clear all three peaks before the stock runs out.",
        if portrait { 10. } else { title_x },
        if portrait {
            610.
        } else if compact {
            362.
        } else {
            595.
        },
        scaled(11., state),
        muted(),
    );
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW TRIPEAKS", state.large_text);
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
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    text(
        label,
        rect.x + 10.,
        rect.y + 28.,
        accessibility::text_size(10., large_text),
        WHITE,
    );
}

fn status_text(status: TriPeaksStatus) -> &'static str {
    match status {
        TriPeaksStatus::Playing => "Clear the three peaks",
        TriPeaksStatus::Won => "The peaks are clear",
        TriPeaksStatus::Stuck => "No peak card can play",
    }
}

fn scaled(size: f32, state: &AppState) -> f32 {
    accessibility::text_size(size, state.large_text)
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(value, x, y, size, color);
}

fn accent() -> Color {
    Color::new(0.98, 0.83, 0.45, 1.)
}

fn muted() -> Color {
    Color::new(0.70, 0.64, 0.78, 1.)
}

fn back_rect() -> Rect {
    Rect::new(0., 0., 110., 42.)
}
