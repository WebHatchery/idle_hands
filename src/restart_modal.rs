use crate::{
    state::{AppState, Screen},
    ui::UiAction,
};
use macroquad::prelude::*;

pub fn clicks(p: Vec2) -> Vec<UiAction> {
    let (cancel, start) = if crate::ui::is_compact_landscape() {
        (
            Rect::new(290., 195., 115., 44.),
            Rect::new(445., 195., 115., 44.),
        )
    } else if crate::ui::is_portrait() {
        (
            Rect::new(45., 360., 120., 44.),
            Rect::new(195., 360., 120., 44.),
        )
    } else {
        (
            Rect::new(450., 360., 160., 48.),
            Rect::new(670., 360., 160., 48.),
        )
    };
    if crate::ui::hit(cancel, p) {
        vec![UiAction::Cancel]
    } else if crate::ui::hit(start, p) {
        vec![UiAction::ConfirmRestart]
    } else {
        vec![]
    }
}

pub fn draw(state: &AppState) {
    let title = match state.screen {
        Screen::Game(game) => format!("Start a new {}?", game.title()),
        _ => "Start a new game?".into(),
    };
    let (panel_rect, cancel, start, title_pos, detail_pos) = if crate::ui::is_compact_landscape() {
        (
            Rect::new(270., 95., 320., 170.),
            Rect::new(290., 195., 115., 44.),
            Rect::new(445., 195., 115., 44.),
            vec2(305., 135.),
            vec2(305., 160.),
        )
    } else if crate::ui::is_portrait() {
        (
            Rect::new(25., 255., 310., 190.),
            Rect::new(45., 360., 120., 44.),
            Rect::new(195., 360., 120., 44.),
            vec2(55., 300.),
            vec2(55., 330.),
        )
    } else {
        (
            Rect::new(390., 250., 500., 200.),
            Rect::new(450., 360., 160., 48.),
            Rect::new(670., 360., 160., 48.),
            vec2(445., 305.),
            vec2(445., 335.),
        )
    };
    draw_rectangle(
        panel_rect.x,
        panel_rect.y,
        panel_rect.w,
        panel_rect.h,
        Color::new(0.16, 0.09, 0.20, 0.98),
    );
    draw_rectangle_lines(
        panel_rect.x,
        panel_rect.y,
        panel_rect.w,
        panel_rect.h,
        2.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    draw_text(
        &title,
        title_pos.x,
        title_pos.y,
        if crate::ui::is_portrait() { 18. } else { 21. },
        WHITE,
    );
    draw_text(
        "Current progress will be replaced.",
        detail_pos.x,
        detail_pos.y,
        if crate::ui::is_portrait() { 11. } else { 13. },
        Color::new(0.72, 0.68, 0.82, 1.),
    );
    for (rect, label, fill) in [
        (cancel, "CANCEL", Color::new(0.25, 0.16, 0.32, 1.)),
        (start, "START", Color::new(0.45, 0.22, 0.25, 1.)),
    ] {
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., WHITE);
        let measured = measure_text(label, None, 14, 1.);
        draw_text(
            label,
            rect.x + (rect.w - measured.width) * 0.5,
            rect.y + rect.h * 0.65,
            14.,
            WHITE,
        );
    }
}
