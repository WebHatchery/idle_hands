//! Touch-first Fivefold scorecard presentation.

use crate::{
    fivefold::{Category, FivefoldStatus},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

fn panel(rect: Rect, fill: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.,
        Color::new(0.45, 0.38, 0.65, 0.65),
    );
}
fn button(rect: Rect, label: &str, active: bool) {
    panel(
        rect,
        if active {
            Color::new(0.45, 0.25, 0.42, 1.)
        } else {
            Color::new(0.18, 0.12, 0.28, 1.)
        },
    );
    let width = crate::ui::measure_text(label, None, 16, 1.).width;
    crate::ui::draw_text(
        label,
        rect.x + (rect.w - width) / 2.,
        rect.y + 30.,
        16.,
        WHITE,
    );
}

pub fn draw_fivefold(state: &AppState) {
    let game = &state.fivefold;
    crate::ui::draw_text("‹ CABINET", 40., 55., 20., Color::new(0.78, 0.70, 0.92, 1.));
    crate::ui::draw_text("FIVEFOLD", 40., 105., 44., Color::new(0.98, 0.83, 0.45, 1.));
    crate::ui::draw_text(
        "Five dice, thirteen calls",
        44.,
        132.,
        18.,
        Color::new(0.70, 0.64, 0.78, 1.),
    );
    panel(
        Rect::new(40., 165., 700., 205.),
        Color::new(0.10, 0.07, 0.16, 1.),
    );
    for index in 0..5 {
        let rect = Rect::new(65. + index as f32 * 125., 205., 100., 100.);
        panel(
            rect,
            if game.held[index] {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                Color::new(0.20, 0.14, 0.30, 1.)
            },
        );
        let value = if game.dice[index] == 0 {
            "—".to_owned()
        } else {
            game.dice[index].to_string()
        };
        let width = crate::ui::measure_text(&value, None, 48, 1.).width;
        crate::ui::draw_text(
            &value,
            rect.x + (rect.w - width) / 2.,
            rect.y + 66.,
            48.,
            Color::new(0.98, 0.83, 0.45, 1.),
        );
        crate::ui::draw_text(
            if game.held[index] {
                "HELD"
            } else {
                "TAP TO HOLD"
            },
            rect.x + 11.,
            rect.y + 122.,
            12.,
            Color::new(0.68, 0.63, 0.78, 1.),
        );
    }
    button(
        Rect::new(270., 400., 180., 52.),
        if game.roll_number == 0 {
            "ROLL DICE"
        } else {
            "ROLL AGAIN"
        },
        game.roll_number < 3 && game.status != FivefoldStatus::Complete,
    );
    crate::ui::draw_text(
        format!("Roll {}/3", game.roll_number),
        475.,
        433.,
        19.,
        Color::new(0.72, 0.68, 0.82, 1.),
    );
    crate::ui::draw_text(
        format!("Total  {}", game.total()),
        475.,
        463.,
        19.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    crate::ui::draw_text(
        format!(
            "Upper  {} / 63   Bonus {}",
            game.upper_total(),
            game.bonus()
        ),
        475.,
        493.,
        16.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
    panel(
        Rect::new(790., 75., 430., 585.),
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    crate::ui::draw_text(
        "SCORECARD",
        830.,
        120.,
        28.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    for (index, category) in Category::ALL.iter().enumerate() {
        let y = 145. + index as f32 * 34.;
        let rect = Rect::new(815., y - 24., 380., 30.);
        let score = game.scores[index].map_or_else(
            || {
                if game.roll_number > 0 {
                    game.score_for(*category).to_string()
                } else {
                    "—".to_owned()
                }
            },
            |value| value.to_string(),
        );
        let selected = game.selected_category == Some(*category);
        if game.scores[index].is_none() && game.roll_number > 0 {
            panel(
                rect,
                if selected {
                    Color::new(0.35, 0.24, 0.38, 1.)
                } else {
                    Color::new(0.14, 0.10, 0.22, 1.)
                },
            );
        }
        crate::ui::draw_text(
            category.label(),
            rect.x + 12.,
            y,
            15.,
            if game.scores[index].is_some() {
                Color::new(0.52, 0.48, 0.60, 1.)
            } else {
                WHITE
            },
        );
        crate::ui::draw_text(
            &score,
            rect.right() - 48.,
            y,
            16.,
            Color::new(0.98, 0.83, 0.45, 1.),
        );
    }
    crate::ui::draw_text(
        state.card_hint.as_deref().unwrap_or(match game.status {
            FivefoldStatus::Ready => "Roll, hold, and choose a call",
            FivefoldStatus::Rolling => "Tap a score to record this roll",
            FivefoldStatus::Complete => "Scorecard complete",
        }),
        45.,
        555.,
        18.,
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    button(Rect::new(45., 610., 150., 44.), "NEW SCORECARD", false);
    button(Rect::new(215., 610., 150., 44.), "HINT", false);
    crate::ui::draw_text(
        "Tap ROLL, then tap dice to hold them.",
        390.,
        637.,
        16.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
}

pub fn fivefold_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(20., 20., 180., 50.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    if Rect::new(270., 400., 180., 52.).contains(p) {
        return vec![UiAction::FivefoldRoll];
    }
    if Rect::new(45., 610., 150., 44.).contains(p) {
        return vec![UiAction::FivefoldNew];
    }
    if Rect::new(215., 610., 150., 44.).contains(p) {
        return vec![UiAction::FivefoldHint];
    }
    for index in 0..5 {
        if Rect::new(65. + index as f32 * 125., 205., 100., 100.).contains(p) {
            return vec![UiAction::FivefoldHold(index)];
        }
    }
    for (index, category) in Category::ALL.iter().enumerate() {
        if Rect::new(815., 121. + index as f32 * 34., 380., 30.).contains(p)
            && state.fivefold.scores[index].is_none()
        {
            return vec![UiAction::FivefoldCategory(*category)];
        }
    }
    vec![]
}
