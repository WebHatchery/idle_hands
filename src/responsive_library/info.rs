use super::{back_button, panel, text};
use crate::{state::AppState, ui::UiAction};
use macroquad::prelude::*;

pub fn draw_credits(state: &AppState) {
    let paragraphs = crate::credits_data::paragraphs(&state.content);
    let credits_panel = Rect::new(8., 70., 344., 520.);
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
        20.,
        115.,
        crate::accessibility::text_size(29., state.large_text),
        crate::theme::BRASS,
    );
    text(
        crate::credits_data::title(&state.content),
        22.,
        165.,
        crate::accessibility::text_size(22., state.large_text),
        WHITE,
    );
    let mut y = 205.;
    for (index, paragraph) in paragraphs.iter().enumerate() {
        let base_size = if index == paragraphs.len() - 1 {
            14.
        } else if index == 0 {
            15.
        } else {
            12.
        };
        let size = crate::accessibility::text_size(base_size, state.large_text);
        let color = if state.high_contrast {
            WHITE
        } else if index == 0 || index == paragraphs.len() - 1 {
            crate::theme::CREAM
        } else {
            crate::theme::SECONDARY
        };
        for line in macroquad_toolkit::ui::wrap_text(paragraph, 310., size) {
            text(&line, 22., y, size, color);
            y += size + 7.;
        }
        y += 7.;
    }
    back_button(650.);
    if state.high_contrast {
        draw_rectangle_lines(10., 650., 150., 44., 3., WHITE);
    }
}
pub fn credits_clicks(p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(10., 650., 150., 44.), p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}

pub fn draw_help(state: &AppState) {
    let paragraphs = crate::help_data::paragraphs(&state.content);
    let navigation = crate::help_data::navigation(&state.content);
    panel(
        Rect::new(8., 38., 344., 602.),
        crate::theme::BACKGROUND_DEEP,
    );
    draw_rectangle_lines(
        8.,
        38.,
        344.,
        602.,
        2.,
        if state.high_contrast {
            WHITE
        } else {
            crate::theme::BORDER
        },
    );
    text(
        "HOW TO PLAY",
        20.,
        80.,
        crate::accessibility::text_size(26., state.large_text),
        crate::theme::BRASS,
    );
    let mut y = 112.;
    for (index, paragraph) in paragraphs.iter().enumerate() {
        let size = crate::accessibility::text_size(14., state.large_text);
        for line in macroquad_toolkit::ui::wrap_text(paragraph, 315., size) {
            text(
                &line,
                20.,
                y,
                size,
                if index == 0 || state.high_contrast {
                    WHITE
                } else {
                    Color::new(0.75, 0.70, 0.84, 1.)
                },
            );
            y += size + 8.;
        }
        y += 8.;
    }
    panel(Rect::new(10., 476., 105., 44.), crate::theme::SURFACE);
    text(
        navigation[0].as_str(),
        29.,
        504.,
        crate::accessibility::text_size(10., state.large_text),
        WHITE,
    );
    panel(Rect::new(10., 530., 105., 44.), crate::theme::SURFACE);
    panel(Rect::new(127., 530., 105., 44.), crate::theme::SURFACE);
    panel(Rect::new(244., 530., 106., 44.), crate::theme::MOSS_DARK);
    text(
        navigation[1].as_str(),
        42.,
        558.,
        crate::accessibility::text_size(12., state.large_text),
        WHITE,
    );
    text(
        navigation[2].as_str(),
        150.,
        558.,
        crate::accessibility::text_size(11., state.large_text),
        WHITE,
    );
    text(
        navigation[3].as_str(),
        277.,
        558.,
        crate::accessibility::text_size(12., state.large_text),
        WHITE,
    );
}
pub fn help_clicks(p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(10., 476., 105., 44.), p) {
        vec![UiAction::Tutorials]
    } else if crate::ui::hit(Rect::new(10., 530., 105., 44.), p) {
        vec![UiAction::Rules]
    } else if crate::ui::hit(Rect::new(127., 530., 105., 44.), p) {
        vec![UiAction::Credits]
    } else if crate::ui::hit(Rect::new(244., 530., 106., 44.), p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}
