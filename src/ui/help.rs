use super::{panel, text};
use crate::state::AppState;
use macroquad::prelude::*;

pub(crate) fn draw_help(state: &AppState) {
    let paragraphs = crate::help_data::paragraphs(&state.content);
    let navigation = crate::help_data::navigation(&state.content);
    panel(
        Rect::new(120., 80., 1040., 560.),
        crate::theme::BACKGROUND_DEEP,
    );
    draw_rectangle_lines(
        120.,
        80.,
        1040.,
        560.,
        3.,
        if state.high_contrast {
            WHITE
        } else {
            crate::theme::BORDER
        },
    );
    text(
        "HOW TO PLAY",
        170.,
        145.,
        crate::accessibility::text_size(42., state.large_text),
        crate::theme::BRASS,
    );
    let mut y = 200.;
    for (index, paragraph) in paragraphs.iter().enumerate() {
        let size =
            crate::accessibility::text_size(if index == 0 { 24. } else { 19. }, state.large_text);
        for line in macroquad_toolkit::ui::wrap_text(paragraph, 900., size) {
            text(
                &line,
                170.,
                y,
                size,
                if index == 0 || state.high_contrast {
                    WHITE
                } else {
                    Color::new(0.75, 0.70, 0.84, 1.)
                },
            );
            y += size + 10.;
        }
        y += 4.;
    }
    panel(Rect::new(400., 635., 180., 48.), crate::theme::SURFACE);
    text(
        navigation[0].as_str(),
        450.,
        666.,
        crate::accessibility::text_size(16., state.large_text),
        WHITE,
    );
    panel(Rect::new(600., 635., 180., 48.), crate::theme::SURFACE);
    text(
        navigation[1].as_str(),
        660.,
        666.,
        crate::accessibility::text_size(18., state.large_text),
        WHITE,
    );
    panel(Rect::new(800., 635., 180., 48.), crate::theme::SURFACE);
    text(
        navigation[2].as_str(),
        850.,
        666.,
        crate::accessibility::text_size(18., state.large_text),
        WHITE,
    );
    panel(Rect::new(1030., 635., 180., 48.), crate::theme::MOSS_DARK);
    text(
        navigation[3].as_str(),
        1090.,
        666.,
        crate::accessibility::text_size(18., state.large_text),
        WHITE,
    )
}
