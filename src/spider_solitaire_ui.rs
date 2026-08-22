//! Responsive touch presentation for standard Spider Solitaire.

use crate::{
    accessibility, spider_solitaire::SpiderSolitaireStatus, state::AppState, ui::UiAction,
};
use macroquad::prelude::*;

const COMPACT_SUBTITLE_X: f32 = 250.;

#[derive(Clone, Copy)]
struct Layout {
    top: f32,
    card_w: f32,
    card_h: f32,
    overlap: f32,
    gap: f32,
    stock: Rect,
    hint: Rect,
    undo: Rect,
    new_game: Rect,
}
impl Layout {
    fn column_x(self, column: usize) -> f32 {
        4. + column as f32 * (self.card_w + self.gap)
    }
    fn card_rect(self, column: usize, depth: usize) -> Rect {
        Rect::new(
            self.column_x(column),
            self.top + depth as f32 * self.overlap,
            self.card_w,
            self.card_h,
        )
    }
}
fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            top: 88.,
            card_w: 76.,
            card_h: 98.,
            overlap: 13.,
            gap: 5.,
            stock: Rect::new(10., 32., 76., 50.),
            hint: Rect::new(490., 335., 105., 40.),
            undo: Rect::new(610., 335., 105., 40.),
            new_game: Rect::new(728., 335., 105., 40.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            top: 176.,
            card_w: 30.,
            card_h: 48.,
            overlap: 15.,
            gap: 4.,
            stock: Rect::new(8., 112., 38., 54.),
            hint: Rect::new(5., 700., 105., 38.),
            undo: Rect::new(120., 650., 105., 38.),
            new_game: Rect::new(235., 650., 115., 38.),
        }
    } else {
        Layout {
            top: 180.,
            card_w: 76.,
            card_h: 104.,
            overlap: 20.,
            gap: 8.,
            stock: Rect::new(30., 76., 76., 104.),
            hint: Rect::new(830., 625., 120., 42.),
            undo: Rect::new(970., 625., 120., 42.),
            new_game: Rect::new(1110., 625., 140., 42.),
        }
    }
}
pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(l.stock, point) {
        return vec![UiAction::SpiderSolitaireDeal];
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::SpiderSolitaireHint];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::SpiderSolitaireUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::SpiderSolitaireNew];
    }
    for column in 0..10 {
        let x = l.column_x(column);
        if point.x < x || point.x > x + l.card_w || point.y < l.top {
            continue;
        }
        let len = state.games.spider_solitaire.tableau[column].len();
        let depth = if len == 0 {
            0
        } else {
            (((point.y - l.top) / l.overlap).floor().max(0.) as usize).min(len - 1)
        };
        return vec![UiAction::SpiderSolitaireSelect(column, depth)];
    }
    vec![]
}
pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.spider_solitaire;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let (hx, hy) = if compact {
        (120., 28.)
    } else if portrait {
        (10., 72.)
    } else {
        (390., 58.)
    };
    text(
        "CABINET",
        8.,
        30.,
        accessibility::text_size(13., state.large_text),
        muted(),
    );
    text(
        "SPIDER SOLITAIRE",
        hx,
        hy,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    let subtitle = if compact && game.status == SpiderSolitaireStatus::Won {
        "Webs cleared"
    } else if compact {
        "Build one-suit runs"
    } else if game.status == SpiderSolitaireStatus::Won {
        "Eight suited webs cleared"
    } else {
        "Build descending runs in one suit"
    };
    text(
        subtitle,
        if compact { COMPACT_SUBTITLE_X } else { hx },
        if compact { 52. } else { hy + 24. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    draw_card_slot(l.stock, game.stock.last().copied(), state);
    if !compact {
        text(
            "STOCK",
            l.stock.x,
            l.stock.bottom() + 15.,
            accessibility::text_size(10., state.large_text),
            muted(),
        );
    }
    for complete in 0..8 {
        let x = if portrait {
            52. + complete as f32 * 34.
        } else if compact {
            105. + complete as f32 * 34.
        } else {
            145. + complete as f32 * 40.
        };
        let rect = Rect::new(x, l.stock.y, 28., l.stock.h);
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            accessibility::grid_line(state.high_contrast),
        );
        if complete < game.completed as usize {
            text(
                "✓",
                rect.x + 6.,
                rect.y + rect.h * 0.58,
                accessibility::text_size(18., state.large_text),
                accent(),
            );
        }
    }
    for (column, stack) in game.tableau.iter().enumerate() {
        if stack.is_empty() {
            draw_rectangle_lines(
                l.column_x(column),
                l.top,
                l.card_w,
                l.card_h,
                1.,
                Color::new(0.35, 0.28, 0.48, 1.),
            );
        }
        for (depth, card) in stack.iter().enumerate() {
            crate::card_render::draw_card_accessible(
                l.card_rect(column, depth),
                *card,
                game.selected == Some((column, depth)),
                state.card_back,
                state.reduced_motion,
                state.high_contrast,
                state.large_text,
            );
        }
    }
    text(
        &format!("Runs {} / 8  -  Moves {}", game.completed, game.moves),
        if compact { 10. } else { hx },
        if portrait {
            625.
        } else if compact {
            315.
        } else {
            590.
        },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW DEAL", state.large_text);
    let instruction = "Tap a suited run, then its destination; tap again to release.";
    let instruction_x = if portrait || compact { 10. } else { hx };
    let instruction_y = if portrait {
        645.
    } else if compact {
        280.
    } else {
        615.
    };
    if let Some(hint) = state.card_hint.as_deref() {
        text(
            hint,
            instruction_x,
            instruction_y,
            accessibility::text_size(11., state.large_text),
            accent(),
        );
    } else {
        text(
            instruction,
            instruction_x,
            instruction_y,
            accessibility::text_size(11., state.large_text),
            muted(),
        );
    }
    button(l.hint, "HINT", state.large_text);
}

fn draw_card_slot(rect: Rect, card: Option<crate::cards::Card>, state: &AppState) {
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
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., muted());
    }
}
fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    text(
        label,
        rect.x + 12.,
        rect.y + 26.,
        accessibility::text_size(11., large_text),
        WHITE,
    );
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        20.
    } else if crate::ui::is_compact_landscape() {
        22.
    } else {
        24.
    }
}
fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        10.
    } else {
        13.
    }
}
fn accent() -> Color {
    crate::theme::BRASS
}
fn muted() -> Color {
    crate::theme::SECONDARY
}

#[cfg(test)]
mod tests;
