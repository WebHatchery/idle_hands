//! Portrait Fivefold scorecard with touch-sized paging.

use crate::{
    fivefold::{Category, FivefoldStatus},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

fn dice_rect(index: usize) -> Rect {
    Rect::new(9. + index as f32 * 69., 112., 60., 60.)
}

pub fn draw_fivefold(state: &AppState) {
    let game = &state.fivefold;
    back();
    text("FIVEFOLD", 10., 72., 29., crate::theme::BRASS);
    text(
        "Five dice, thirteen calls",
        12.,
        94.,
        13.,
        crate::theme::SECONDARY,
    );
    for index in 0..5 {
        let rect = dice_rect(index);
        panel(
            rect,
            if game.held[index] {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                Color::new(0.20, 0.14, 0.30, 1.)
            },
        );
        let value = if game.dice[index] == 0 {
            "-".to_owned()
        } else {
            game.dice[index].to_string()
        };
        let width = crate::ui::measure_text(&value, None, 30, 1.).width;
        text(
            &value,
            rect.x + (rect.w - width) / 2.,
            rect.y + 39.,
            30.,
            crate::theme::BRASS,
        );
        text(
            if game.held[index] { "HELD" } else { "HOLD" },
            rect.x + 15.,
            rect.y + 53.,
            8.,
            crate::theme::SECONDARY,
        );
    }
    panel(
        Rect::new(10., 190., 150., 42.),
        if game.roll_number < 3 && game.status != FivefoldStatus::Complete {
            Color::new(0.45, 0.25, 0.42, 1.)
        } else {
            Color::new(0.16, 0.11, 0.24, 1.)
        },
    );
    text(
        if game.roll_number == 0 {
            "ROLL DICE"
        } else {
            "ROLL AGAIN"
        },
        47.,
        217.,
        13.,
        WHITE,
    );
    text(
        &format!("Roll {}/3", game.roll_number),
        180.,
        211.,
        14.,
        crate::theme::SECONDARY,
    );
    text(
        &format!("Total {}", game.total()),
        180.,
        232.,
        14.,
        crate::theme::BRASS,
    );
    panel(
        Rect::new(8., 255., 344., 345.),
        crate::theme::BACKGROUND_DEEP,
    );
    text("SCORECARD", 18., 280., 20., crate::theme::BRASS);
    let page = state.fivefold_score_page.min(1);
    page_button(Rect::new(188., 258., 72., 40.), "PREV");
    page_button(Rect::new(270., 258., 72., 40.), "NEXT");
    text(&format!("{}/2", page + 1), 150., 282., 11., WHITE);
    for (slot, (index, category)) in Category::ALL
        .iter()
        .enumerate()
        .skip(page * 7)
        .take(7)
        .enumerate()
    {
        let y = 327. + slot as f32 * 42.;
        let rect = Rect::new(15., y - 28., 330., 38.);
        let score = game.scores[index].map_or_else(
            || {
                if game.roll_number > 0 {
                    game.score_for(*category).to_string()
                } else {
                    "-".to_owned()
                }
            },
            |value| value.to_string(),
        );
        if game.scores[index].is_none() && game.roll_number > 0 {
            panel(
                rect,
                if game.selected_category == Some(*category) {
                    Color::new(0.35, 0.24, 0.38, 1.)
                } else {
                    Color::new(0.14, 0.10, 0.22, 1.)
                },
            );
        }
        text(
            category.label(),
            rect.x + 9.,
            y,
            11.,
            if game.scores[index].is_some() {
                Color::new(0.52, 0.48, 0.60, 1.)
            } else {
                WHITE
            },
        );
        text(&score, 314., y, 11., crate::theme::BRASS);
    }
    text(
        match game.status {
            FivefoldStatus::Ready => "Roll, hold, then choose a call",
            FivefoldStatus::Rolling => "Tap a score to record this roll",
            FivefoldStatus::Complete => "Scorecard complete",
        },
        10.,
        625.,
        11.,
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    panel(Rect::new(10., 650., 150., 44.), crate::theme::SURFACE);
    text("NEW SCORECARD", 31., 679., 11., WHITE);
    panel(Rect::new(180., 650., 150., 44.), crate::theme::SURFACE);
    text("HINT", 235., 679., 11., WHITE);
    if let Some(hint) = state.card_hint.as_deref() {
        text(hint, 10., 705., 10., Color::new(0.63, 0.95, 0.72, 1.));
    }
}

pub fn fivefold_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(Rect::new(10., 190., 150., 42.), p) {
        return vec![UiAction::FivefoldRoll];
    }
    if crate::ui::hit(Rect::new(10., 650., 150., 44.), p) {
        return vec![UiAction::FivefoldNew];
    }
    if crate::ui::hit(Rect::new(180., 650., 150., 44.), p) {
        return vec![UiAction::FivefoldHint];
    }
    for index in 0..5 {
        if dice_rect(index).contains(p) {
            return vec![UiAction::FivefoldHold(index)];
        }
    }
    if crate::ui::hit(Rect::new(188., 258., 72., 40.), p) {
        return vec![UiAction::FivefoldScorePage(-1)];
    }
    if crate::ui::hit(Rect::new(270., 258., 72., 40.), p) {
        return vec![UiAction::FivefoldScorePage(1)];
    }
    let page = state.fivefold_score_page.min(1);
    for (slot, (index, category)) in Category::ALL
        .iter()
        .enumerate()
        .skip(page * 7)
        .take(7)
        .enumerate()
    {
        if crate::ui::hit(Rect::new(15., 299. + slot as f32 * 42., 330., 38.), p)
            && state.fivefold.scores[index].is_none()
        {
            return vec![UiAction::FivefoldCategory(*category)];
        }
    }
    vec![]
}

fn page_button(rect: Rect, label: &str) {
    panel(rect, crate::theme::SURFACE_DARK);
    text(label, rect.x + 18., rect.y + 25., 10., WHITE);
}

fn panel(rect: Rect, fill: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::drawer_surface(fill));
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::BORDER);
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}

fn back() {
    panel(Rect::new(0., 0., 110., 44.), crate::theme::SURFACE_DARK);
    text("CABINET", 8., 29., 13., crate::theme::BRASS);
}
