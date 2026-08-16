//! Responsive touch presentation for Pyramid Solitaire.

use crate::{accessibility, pyramid::PyramidStatus, state::AppState, ui::UiAction};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    base_x: f32,
    top: f32,
    card_w: f32,
    card_h: f32,
    row_gap: f32,
    card_gap: f32,
    stock: Rect,
    waste: Rect,
    hint: Rect,
    undo: Rect,
    new_game: Rect,
}

impl Layout {
    fn row_x(self, row: usize, column: usize) -> f32 {
        self.base_x
            + (3. - row as f32 / 2.) * (self.card_w + self.card_gap)
            + column as f32 * (self.card_w + self.card_gap)
    }

    fn card_rect(self, index: usize) -> Rect {
        let row = row_for(index);
        let column = index - row_start(row);
        Rect::new(
            self.row_x(row, column),
            self.top + row as f32 * self.row_gap,
            self.card_w,
            self.card_h,
        )
    }
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            base_x: 105.,
            top: 82.,
            card_w: 60.,
            card_h: 58.,
            row_gap: 28.,
            card_gap: 4.,
            stock: Rect::new(650., 72., 62., 62.),
            waste: Rect::new(722., 72., 62., 62.),
            hint: Rect::new(530., 280., 105., 40.),
            undo: Rect::new(650., 280., 105., 40.),
            new_game: Rect::new(765., 280., 105., 40.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            base_x: 18.,
            top: 168.,
            card_w: 44.,
            card_h: 58.,
            row_gap: 31.,
            card_gap: 3.,
            stock: Rect::new(18., 90., 58., 64.),
            waste: Rect::new(86., 90., 58., 64.),
            hint: Rect::new(154., 90., 100., 64.),
            undo: Rect::new(20., 650., 145., 42.),
            new_game: Rect::new(185., 650., 145., 42.),
        }
    } else {
        Layout {
            base_x: 360.,
            top: 115.,
            card_w: 70.,
            card_h: 88.,
            row_gap: 48.,
            card_gap: 5.,
            stock: Rect::new(930., 120., 82., 100.),
            waste: Rect::new(1030., 120., 82., 100.),
            hint: Rect::new(810., 250., 105., 44.),
            undo: Rect::new(930., 250., 100., 44.),
            new_game: Rect::new(1045., 250., 125., 44.),
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(back_rect(), point) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(l.stock, point) {
        return vec![UiAction::PyramidStock];
    }
    if l.waste.contains(point) {
        return vec![UiAction::PyramidTap(crate::pyramid::WASTE_INDEX)];
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::PyramidHint];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::PyramidUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::PyramidNew];
    }
    for index in (0..28).rev() {
        if l.card_rect(index).contains(point) {
            return vec![UiAction::PyramidTap(index)];
        }
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.pyramid;
    let portrait = crate::ui::is_portrait();
    let compact = crate::ui::is_compact_landscape();
    let title_x = if compact {
        110.
    } else if portrait {
        10.
    } else {
        400.
    };
    let title_y = if compact {
        30.
    } else if portrait {
        55.
    } else {
        72.
    };
    text(
        "‹ CABINET",
        8.,
        30.,
        accessibility::text_size(13., state.large_text),
        muted(),
    );
    text(
        "PYRAMID",
        title_x,
        title_y,
        accessibility::text_size(if portrait { 25. } else { 30. }, state.large_text),
        accent(),
    );
    text(
        status_text(game.status),
        if compact { 650. } else { title_x },
        if compact { 30. } else { title_y + 25. },
        accessibility::text_size(12., state.large_text),
        muted(),
    );
    draw_slot(l.stock, game.stock.last().copied(), false, state);
    draw_slot(l.waste, game.waste.last().copied(), true, state);
    text(
        "STOCK",
        l.stock.x,
        l.stock.bottom() + 15.,
        accessibility::text_size(10., state.large_text),
        muted(),
    );
    text(
        "WASTE",
        l.waste.x,
        l.waste.bottom() + 15.,
        accessibility::text_size(10., state.large_text),
        muted(),
    );
    for index in 0..28 {
        if let Some(card) = game.pyramid[index] {
            crate::card_render::draw_card_accessible(
                l.card_rect(index),
                card,
                game.selected == Some(index),
                state.card_back,
                state.reduced_motion,
                state.high_contrast,
                state.large_text,
            );
        }
    }
    text(
        &format!("Moves {}  •  Pair exposed cards to make 13", game.moves),
        if portrait { 10. } else { title_x },
        if portrait {
            595.
        } else if compact {
            345.
        } else {
            570.
        },
        accessibility::text_size(12., state.large_text),
        muted(),
    );
    let detail = state.card_hint.as_deref().unwrap_or(if portrait {
        "Tap a king, or tap two exposed cards that total 13."
    } else {
        "Tap a king, or select two exposed cards that total 13."
    });
    text(
        detail,
        if portrait { 10. } else { title_x },
        if portrait {
            620.
        } else if compact {
            365.
        } else {
            595.
        },
        accessibility::text_size(11., state.large_text),
        if state.card_hint.is_some() {
            accent()
        } else {
            muted()
        },
    );
    button(l.hint, "HINT", state.large_text);
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW PYRAMID", state.large_text);
}

fn draw_slot(rect: Rect, card: Option<crate::cards::Card>, back: bool, state: &AppState) {
    if let Some(card) = card {
        crate::card_render::draw_card_accessible(
            rect,
            card,
            state.pyramid.selected == Some(crate::pyramid::WASTE_INDEX) && back,
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

fn status_text(status: PyramidStatus) -> &'static str {
    match status {
        PyramidStatus::Playing => "Clear the pyramid",
        PyramidStatus::Won => "The pyramid is clear",
        PyramidStatus::Stuck => "No pairing move remains",
    }
}

fn row_start(row: usize) -> usize {
    row * (row + 1) / 2
}

fn row_for(index: usize) -> usize {
    (0..7).find(|&row| index < row_start(row + 1)).unwrap_or(6)
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
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
