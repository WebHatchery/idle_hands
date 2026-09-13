use super::{back, panel, text};
use crate::{state::AppState, ui::UiAction};
use macroquad::prelude::*;

pub fn draw_credits(state: &AppState) {
    let paragraphs = crate::credits_data::paragraphs(&state.content);
    let credits_panel = Rect::new(170., 20., 504., 350.);
    panel(credits_panel, crate::theme::BACKGROUND_DEEP);
    if state.high_contrast {
        draw_rectangle_lines(
            credits_panel.x,
            credits_panel.y,
            credits_panel.w,
            credits_panel.h,
            3.,
            WHITE,
        );
    }
    text(
        "CREDITS",
        205.,
        62.,
        crate::accessibility::text_size(28., state.large_text),
        crate::theme::BRASS,
    );
    text(
        crate::credits_data::title(&state.content),
        205.,
        115.,
        crate::accessibility::text_size(20., state.large_text),
        WHITE,
    );
    let mut y = 155.;
    for (index, paragraph) in paragraphs.iter().enumerate() {
        let base_size = if index == paragraphs.len() - 1 {
            13.
        } else if index == 0 {
            14.
        } else {
            10.
        };
        let size = crate::accessibility::text_size(base_size, state.large_text);
        let color = if state.high_contrast {
            WHITE
        } else if index == 0 || index == paragraphs.len() - 1 {
            crate::theme::CREAM
        } else {
            crate::theme::SECONDARY
        };
        for line in macroquad_toolkit::ui::wrap_text(paragraph, 440., size) {
            text(&line, 205., y, size, color);
            y += size + 4.;
        }
        y += 4.;
    }
    back(Rect::new(365., 315., 110., 44.));
    if state.high_contrast {
        draw_rectangle_lines(365., 315., 110., 44., 3., WHITE);
    }
}
pub fn credits_clicks(p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(365., 315., 110., 44.), p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}

pub fn draw_help(state: &AppState) {
    let paragraphs = crate::help_data::paragraphs(&state.content);
    let navigation = crate::help_data::navigation(&state.content);
    panel(
        Rect::new(20., 12., 804., 365.),
        crate::theme::BACKGROUND_DEEP,
    );
    text(
        "HOW TO PLAY",
        40.,
        52.,
        crate::accessibility::text_size(26., state.large_text),
        crate::theme::BRASS,
    );
    let mut y = 92.;
    for (index, paragraph) in paragraphs.iter().enumerate() {
        let size = crate::accessibility::text_size(14., state.large_text);
        for line in macroquad_toolkit::ui::wrap_text(paragraph, 760., size) {
            text(
                &line,
                40.,
                y,
                if index == 0 {
                    crate::accessibility::text_size(16., state.large_text)
                } else {
                    size
                },
                if index == 0 || state.high_contrast {
                    WHITE
                } else {
                    Color::new(0.75, 0.70, 0.84, 1.)
                },
            );
            y += size + 8.;
        }
        y += 5.;
    }
    for (rect, label) in [
        Rect::new(300., 288., 110., 44.),
        Rect::new(430., 288., 110., 44.),
        Rect::new(555., 288., 110., 44.),
        Rect::new(680., 288., 130., 44.),
    ]
    .into_iter()
    .zip(navigation.iter())
    {
        panel(rect, crate::theme::SURFACE);
        text(
            label,
            rect.x + 30.,
            rect.y + 29.,
            crate::accessibility::text_size(11., state.large_text),
            WHITE,
        );
    }
}
pub fn help_clicks(p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(300., 288., 110., 44.), p) {
        vec![UiAction::Tutorials]
    } else if crate::ui::hit(Rect::new(430., 288., 110., 44.), p) {
        vec![UiAction::Rules]
    } else if crate::ui::hit(Rect::new(555., 288., 110., 44.), p) {
        vec![UiAction::Credits]
    } else if crate::ui::hit(Rect::new(680., 288., 130., 44.), p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}
