//! Responsive touch presentation for one-suit Spider Solitaire.

use crate::{spider::SpiderStatus, state::AppState, ui::UiAction};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Layout {
    pub top: f32,
    pub card_w: f32,
    pub card_h: f32,
    pub overlap: f32,
    pub gap: f32,
    pub stock: Rect,
    pub hint: Rect,
    pub undo: Rect,
    pub new_game: Rect,
}

impl Layout {
    pub fn column_x(self, column: usize) -> f32 {
        4. + column as f32 * (self.card_w + self.gap)
    }
    pub fn card_rect(self, column: usize, depth: usize) -> Rect {
        Rect::new(
            self.column_x(column),
            self.top + depth as f32 * self.overlap,
            self.card_w,
            self.card_h,
        )
    }
}

pub fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            top: 91.,
            card_w: 78.,
            card_h: 102.,
            overlap: 14.,
            gap: 7.,
            stock: Rect::new(10., 34., 78., 52.),
            hint: Rect::new(610., 265., 105., 40.),
            undo: Rect::new(610., 335., 105., 40.),
            new_game: Rect::new(728., 335., 105., 40.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            top: 178.,
            card_w: 39.,
            card_h: 54.,
            overlap: 17.,
            gap: 6.,
            stock: Rect::new(8., 112., 44., 58.),
            hint: Rect::new(5., 650., 105., 38.),
            undo: Rect::new(120., 650., 105., 38.),
            new_game: Rect::new(235., 650., 115., 38.),
        }
    } else {
        Layout {
            top: 194.,
            card_w: 94.,
            card_h: 126.,
            overlap: 22.,
            gap: 16.,
            stock: Rect::new(30., 88., 94., 126.),
            hint: Rect::new(830., 625., 120., 42.),
            undo: Rect::new(970., 625., 120., 42.),
            new_game: Rect::new(1110., 625., 140., 42.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let layout = layout();
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(layout.stock, point) {
        return vec![UiAction::SpiderDeal];
    }
    if crate::ui::hit(layout.hint, point) {
        return vec![UiAction::SpiderHint];
    }
    if crate::ui::hit(layout.undo, point) {
        return vec![UiAction::SpiderUndo];
    }
    if crate::ui::hit(layout.new_game, point) {
        return vec![UiAction::SpiderNew];
    }
    for column in 0..8 {
        let x = layout.column_x(column);
        if point.x < x || point.x > x + layout.card_w || point.y < layout.top {
            continue;
        }
        let stack_len = state.games.spider.tableau[column].len();
        let depth = if stack_len == 0 {
            0
        } else {
            (((point.y - layout.top) / layout.overlap).floor().max(0.) as usize).min(stack_len - 1)
        };
        return vec![UiAction::SpiderSelect(column, depth)];
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let layout = layout();
    let game = &state.games.spider;
    let (header_x, header_y) = if crate::ui::is_compact_landscape() {
        (112., 27.)
    } else if crate::ui::is_portrait() {
        (10., 72.)
    } else {
        (30., 58.)
    };
    text("‹ CABINET", 8., 30., 13., muted());
    text("SPIDER", header_x, header_y, title_size(), accent());
    let compact = crate::ui::is_compact_landscape();
    let subtitle_x = if compact { 250. } else { header_x };
    let subtitle_y = if compact { 28. } else { header_y + 24. };
    text(
        if compact && game.status == SpiderStatus::Won {
            "Webs cleared"
        } else if compact {
            "Build runs"
        } else if game.status == SpiderStatus::Won {
            "Eight webs cleared"
        } else {
            "Build descending runs in one suit"
        },
        subtitle_x,
        subtitle_y,
        body_size(),
        muted(),
    );
    draw_card_slot(layout.stock, game.stock.last().copied(), state);
    text(
        "STOCK",
        layout.stock.x,
        layout.stock.bottom() + 15.,
        10.,
        muted(),
    );
    for completed in 0..8 {
        let x = if crate::ui::is_portrait() {
            62. + completed as f32 * 35.
        } else if crate::ui::is_compact_landscape() {
            105. + completed as f32 * 35.
        } else {
            155. + completed as f32 * 42.
        };
        let rect = Rect::new(x, layout.stock.y, 30., layout.stock.h);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., crate::theme::BRASS);
        if completed < game.completed as usize {
            text("✓", rect.x + 7., rect.y + rect.h * 0.58, 18., accent());
        }
    }
    for (column, stack) in game.tableau.iter().enumerate() {
        if stack.is_empty() {
            draw_rectangle_lines(
                layout.column_x(column),
                layout.top,
                layout.card_w,
                layout.card_h,
                1.,
                crate::theme::BRASS,
            );
        }
        for (depth, card) in stack.iter().enumerate() {
            let selected = game.selected == Some((column, depth));
            crate::card_render::draw_card(
                layout.card_rect(column, depth),
                *card,
                selected,
                state.card_back,
                state.reduced_motion,
            );
        }
    }
    text(
        &format!("Runs {} / 8  •  Moves {}", game.completed, game.moves),
        if crate::ui::is_compact_landscape() {
            10.
        } else {
            header_x
        },
        if crate::ui::is_portrait() {
            625.
        } else if crate::ui::is_compact_landscape() {
            315.
        } else {
            590.
        },
        body_size(),
        muted(),
    );
    button(layout.undo, "UNDO");
    button(layout.new_game, "NEW DEAL");
    button(layout.hint, "HINT");
    text(
        state
            .card_hint
            .as_deref()
            .unwrap_or("Tap a run, then its destination; tap again to release."),
        if crate::ui::is_portrait() {
            10.
        } else if crate::ui::is_compact_landscape() {
            300.
        } else {
            header_x
        },
        if crate::ui::is_portrait() {
            700.
        } else if crate::ui::is_compact_landscape() {
            315.
        } else {
            615.
        },
        11.,
        muted(),
    );
}

pub fn draw_card_slot(rect: Rect, card: Option<crate::cards::Card>, state: &AppState) {
    if let Some(card) = card {
        crate::card_render::draw_card(rect, card, false, state.card_back, state.reduced_motion);
    } else {
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., muted());
    }
}

pub fn button(rect: Rect, label: &str) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    text(label, rect.x + 12., rect.y + 26., 11., WHITE);
}

pub fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
pub fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        29.
    } else {
        31.
    }
}
pub fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        11.
    } else {
        13.
    }
}
pub fn accent() -> Color {
    crate::theme::BRASS
}
pub fn muted() -> Color {
    crate::theme::SECONDARY
}
