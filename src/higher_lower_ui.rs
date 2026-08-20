//! Responsive presentation and touch routing for Higher or Lower.

use crate::{
    accessibility,
    higher_lower::{Guess, HigherLowerRule, HigherLowerStatus},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    higher: Rect,
    lower: Rect,
    hint: Rect,
    undo: Rect,
    new_game: Rect,
    cash_out: Rect,
    rule: Rect,
}
fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            higher: Rect::new(430., 210., 130., 44.),
            lower: Rect::new(570., 210., 130., 44.),
            hint: Rect::new(430., 335., 100., 40.),
            undo: Rect::new(430., 275., 100., 40.),
            new_game: Rect::new(540., 275., 130., 40.),
            cash_out: Rect::new(540., 335., 130., 40.),
            rule: Rect::new(680., 275., 145., 40.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            higher: Rect::new(20., 430., 155., 44.),
            lower: Rect::new(185., 430., 165., 44.),
            hint: Rect::new(20., 565., 145., 42.),
            undo: Rect::new(20., 510., 145., 42.),
            new_game: Rect::new(185., 510., 165., 42.),
            cash_out: Rect::new(185., 565., 165., 42.),
            rule: Rect::new(20., 620., 330., 42.),
        }
    } else {
        Layout {
            higher: Rect::new(650., 360., 160., 46.),
            lower: Rect::new(830., 360., 170., 46.),
            hint: Rect::new(650., 490., 120., 44.),
            undo: Rect::new(650., 430., 120., 44.),
            new_game: Rect::new(790., 430., 150., 44.),
            cash_out: Rect::new(790., 490., 150., 44.),
            rule: Rect::new(960., 430., 180., 44.),
        }
    }
}
pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(back_rect(), point) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(l.higher, point) {
        return vec![UiAction::HigherLowerGuess(Guess::Higher)];
    }
    if crate::ui::hit(l.lower, point) {
        return vec![UiAction::HigherLowerGuess(Guess::Lower)];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::HigherLowerUndo];
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::HigherLowerHint];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::HigherLowerNew];
    }
    if crate::ui::hit(l.cash_out, point) {
        return vec![UiAction::HigherLowerCashOut];
    }
    if crate::ui::hit(l.rule, point) {
        return vec![UiAction::HigherLowerRule(
            match state.games.higher_lower.rule {
                HigherLowerRule::Friendly => HigherLowerRule::House,
                HigherLowerRule::House => HigherLowerRule::Friendly,
            },
        )];
    }
    vec![]
}
pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.higher_lower;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let hx = if compact {
        120.
    } else if portrait {
        10.
    } else {
        360.
    };
    let hy = if compact {
        30.
    } else if portrait {
        68.
    } else {
        60.
    };
    text(
        "‹ CABINET",
        8.,
        30.,
        accessibility::text_size(13., state.large_text),
        muted(),
    );
    text(
        "HIGHER OR LOWER",
        hx,
        hy,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    text(
        &status_text(game),
        if compact { 430. } else { hx },
        if compact { 30. } else { hy + 25. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    let card = if compact {
        Rect::new(140., 85., 150., 180.)
    } else if portrait {
        Rect::new(80., 120., 200., 250.)
    } else {
        Rect::new(360., 100., 220., 280.)
    };
    draw_rectangle(
        card.x,
        card.y,
        card.w,
        card.h,
        if state.high_contrast {
            crate::theme::SURFACE_DARK
        } else {
            crate::theme::SURFACE
        },
    );
    draw_rectangle_lines(card.x, card.y, card.w, card.h, 2., accent());
    text(
        &format!("{}", game.current),
        card.x + card.w * 0.43,
        card.y + card.h * 0.58,
        accessibility::text_size(if portrait { 68. } else { 84. }, state.large_text),
        WHITE,
    );
    text(
        "GUESS THE NEXT CARD",
        if compact {
            140.
        } else if portrait {
            20.
        } else {
            360.
        },
        if compact {
            285.
        } else if portrait {
            400.
        } else {
            410.
        },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    text(
        &format!(
            "Run {} / 10 • Pot {} • {}",
            game.score,
            game.pot,
            state.card_hint.as_deref().unwrap_or("Next card hidden")
        ),
        if compact {
            140.
        } else if portrait {
            20.
        } else {
            360.
        },
        if compact {
            305.
        } else if portrait {
            420.
        } else {
            435.
        },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    choice_button(
        l.higher,
        &format!("HIGHER {}%", game.chance(Guess::Higher)),
        game.chance(Guess::Higher) >= game.chance(Guess::Lower),
        state.large_text,
    );
    choice_button(
        l.lower,
        &format!("LOWER {}%", game.chance(Guess::Lower)),
        game.chance(Guess::Lower) >= game.chance(Guess::Higher),
        state.large_text,
    );
    button(l.hint, "HINT", state.large_text);
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW ROUND", state.large_text);
    button(
        l.cash_out,
        &format!("CASH OUT {}", game.pot),
        state.large_text,
    );
    button(l.rule, game.rule.label(), state.large_text);
    draw_run_meter(
        game.score,
        if portrait { 20. } else { hx },
        if portrait {
            665.
        } else if compact {
            80.
        } else {
            560.
        },
    );
}
fn status_text(game: &crate::higher_lower::HigherLower) -> String {
    match game.status {
        HigherLowerStatus::Playing => format!("Run {} / 10 • Pot {}", game.score, game.pot),
        HigherLowerStatus::Won if game.cashed_out => format!("Cashed out {}", game.banked),
        HigherLowerStatus::Won => format!("Perfect run banked {}", game.banked),
        HigherLowerStatus::Lost => "The next card slipped away".into(),
    }
}
fn choice_button(rect: Rect, label: &str, safer: bool, large_text: bool) {
    button(rect, label, large_text);
    if safer {
        draw_rectangle_lines(
            rect.x + 2.,
            rect.y + 2.,
            rect.w - 4.,
            rect.h - 4.,
            3.,
            safe(),
        );
    }
}
fn draw_run_meter(score: u16, x: f32, y: f32) {
    for step in 0..10 {
        let fill = if step < score as usize {
            safe()
        } else {
            crate::theme::SURFACE
        };
        draw_rectangle(x + step as f32 * 18., y, 14., 8., fill);
    }
}
fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    text(
        label,
        rect.x + 12.,
        rect.y + 29.,
        accessibility::text_size(11., large_text),
        WHITE,
    );
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        23.
    } else {
        29.
    }
}
fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        11.
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
fn safe() -> Color {
    Color::from_rgba(80, 224, 126, 255)
}
fn back_rect() -> Rect {
    Rect::new(0., 0., 110., 42.)
}
