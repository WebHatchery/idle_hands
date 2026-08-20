//! Responsive touch presentation for Pyramid Solitaire.

use crate::{
    accessibility,
    pyramid::{PyramidDraw, PyramidStatus},
    state::AppState,
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
    card_gap: f32,
    stock: Rect,
    waste: Rect,
    hint: Rect,
    undo: Rect,
    new_game: Rect,
    draw_rule: Rect,
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
            hint: Rect::new(570., 280., 80., 40.),
            undo: Rect::new(658., 280., 80., 40.),
            new_game: Rect::new(746., 280., 90., 40.),
            draw_rule: Rect::new(650., 140., 134., 40.),
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
            draw_rule: Rect::new(264., 90., 100., 64.),
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
            draw_rule: Rect::new(930., 315., 140., 44.),
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
    if crate::ui::hit(l.draw_rule, point) {
        return vec![UiAction::PyramidDrawRule(
            match _state.games.pyramid.draw_rule {
                PyramidDraw::One => PyramidDraw::Three,
                PyramidDraw::Three => PyramidDraw::One,
            },
        )];
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
    let game = &state.games.pyramid;
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
    let scoreline = if compact || portrait {
        format!(
            "{} pairs • P{} • C{} • R{}",
            game.available_pair_count(),
            game.points,
            game.combo,
            game.redeals_remaining
        )
    } else {
        format!(
            "{} • {} pairs • P{} • Chain {} • Recycle {}",
            status_text(game.status),
            game.available_pair_count(),
            game.points,
            game.combo,
            game.redeals_remaining
        )
    };
    text(
        &scoreline,
        if compact { 450. } else { title_x },
        if compact { 30. } else { title_y + 25. },
        accessibility::text_size(12., state.large_text),
        muted(),
    );
    draw_slot(l.stock, game.stock.last().copied(), false, state);
    draw_slot(l.waste, game.waste.last().copied(), true, state);
    text(
        if game.stock.is_empty() && game.redeals_remaining > 0 {
            "RECYCLE"
        } else {
            "STOCK"
        },
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
            if preview_target(game, index) {
                let rect = l.card_rect(index);
                draw_rectangle_lines(
                    rect.x + 2.,
                    rect.y + 2.,
                    rect.w - 4.,
                    rect.h - 4.,
                    3.,
                    Color::new(0.45, 0.95, 0.60, 1.),
                );
            }
        }
    }
    text(
        &format!(
            "Moves {}  •  {}  •  Pair exposed cards to make 13",
            game.moves,
            game.draw_rule.label()
        ),
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
    mode_button(l.draw_rule, game.draw_rule.label(), state.large_text);

    if preview_target(game, crate::pyramid::WASTE_INDEX) {
        draw_rectangle_lines(
            l.waste.x + 2.,
            l.waste.y + 2.,
            l.waste.w - 4.,
            l.waste.h - 4.,
            3.,
            Color::new(0.45, 0.95, 0.60, 1.),
        );
    }
}

fn preview_target(game: &crate::pyramid::Pyramid, index: usize) -> bool {
    if !game.available(index) {
        return false;
    }
    if let Some(selected) = game.selected {
        game.legal_pair(selected, index)
    } else {
        let card = if index == crate::pyramid::WASTE_INDEX {
            game.waste.last().copied()
        } else {
            game.pyramid.get(index).copied().flatten()
        };
        card.is_some_and(|card| card.rank == 13)
    }
}

fn mode_button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.22, 0.30, 0.20, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    text(
        label,
        rect.x + 10.,
        rect.y + rect.h * 0.62,
        accessibility::text_size(10., large_text),
        WHITE,
    );
}

fn draw_slot(rect: Rect, card: Option<crate::cards::Card>, back: bool, state: &AppState) {
    if let Some(card) = card {
        crate::card_render::draw_card_accessible(
            rect,
            card,
            state.games.pyramid.selected == Some(crate::pyramid::WASTE_INDEX) && back,
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
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
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
    crate::theme::BRASS
}

fn muted() -> Color {
    crate::theme::SECONDARY
}

fn back_rect() -> Rect {
    Rect::new(0., 0., 110., 42.)
}
