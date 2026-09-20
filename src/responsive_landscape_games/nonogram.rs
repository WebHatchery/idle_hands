use super::{back, panel, text};
use crate::{
    accessibility,
    grid::GridLayout,
    nonogram::{NonogramMark, NonogramMode, NonogramStatus},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

const NONO_BOARD: Rect = Rect {
    x: 10.,
    y: 28.,
    w: 360.,
    h: 360.,
};
pub fn draw_nonogram(state: &AppState) {
    let game = &state.games.nonogram;
    back();
    text("NONOGRAM", 100., 20., 19., crate::theme::BRASS);
    panel(NONO_BOARD, accessibility::board_fill(state.high_contrast));
    let layout = nonogram_grid(state);
    let visible = crate::nonogram::visible_size(game.size, state.games.nonogram_zoomed);
    let (origin_x, origin_y) = nonogram_origin(state);
    for local in 0..visible * visible {
        let index = nonogram_global_index(state, local);
        let Some(cell) = layout.cell_rect(local) else {
            continue;
        };
        let fill = accessibility::nonogram_cell(game.marks[index] as u8, state.high_contrast);
        draw_rectangle(cell.x, cell.y, cell.w - 1., cell.h - 1., fill);
        draw_rectangle_lines(
            cell.x,
            cell.y,
            cell.w - 1.,
            cell.h - 1.,
            1.,
            accessibility::grid_line(state.high_contrast),
        );
        if game.marks[index] == NonogramMark::Crossed {
            text(
                "x",
                cell.x + cell.w * 0.35,
                cell.y + cell.h * 0.68,
                (cell.w * 0.55).min(16.),
                Color::new(0.65, 0.58, 0.76, 1.),
            );
        }
    }
    let clue_size = accessibility::text_size((layout.cell_width * 0.32).min(11.), state.large_text);
    let clue_line_height = clue_size + 1.;
    let clue_color = if state.high_contrast {
        WHITE
    } else {
        crate::theme::CREAM
    };
    for (local, clue) in game
        .row_clues
        .iter()
        .skip(origin_y)
        .take(visible)
        .enumerate()
    {
        text(
            &clue
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            15.,
            layout.bounds.y + local as f32 * layout.cell_height + layout.cell_height * 0.68,
            clue_size,
            clue_color,
        );
    }
    for (local, clue) in game
        .column_clues
        .iter()
        .skip(origin_x)
        .take(visible)
        .enumerate()
    {
        let x = layout.bounds.x + local as f32 * layout.cell_width + layout.cell_width * 0.5
            - clue_size * 0.2;
        for (index, value) in clue.iter().enumerate() {
            let from_bottom = clue.len() - index - 1;
            text(
                &value.to_string(),
                x,
                layout.bounds.y - 5. - from_bottom as f32 * clue_line_height,
                clue_size,
                clue_color,
            );
        }
    }
    text(
        if game.status == NonogramStatus::Won {
            "Picture complete"
        } else if state.games.nonogram_zoomed && game.size > visible {
            "Zoomed 9 × 9 focus"
        } else {
            "Fill or cross from the clues"
        },
        400.,
        50.,
        14.,
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    panel(
        Rect::new(400., 135., 160., 44.),
        if game.mode == NonogramMode::Fill {
            Color::new(0.45, 0.25, 0.42, 1.)
        } else {
            crate::theme::SURFACE
        },
    );
    text(
        if game.mode == NonogramMode::Fill {
            "FILL MODE"
        } else {
            "CROSS MODE"
        },
        445.,
        163.,
        12.,
        WHITE,
    );
    panel(
        Rect::new(590., 135., 160., 44.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("UNDO", 650., 163., 12., WHITE);
    panel(Rect::new(400., 190., 160., 44.), crate::theme::SURFACE);
    text(
        if state.games.nonogram_zoomed && game.size > visible {
            "FULL BOARD"
        } else {
            "ZOOM 9 × 9"
        },
        445.,
        218.,
        11.,
        WHITE,
    );
    for (rect, label) in [
        (Rect::new(590., 190., 50., 44.), "LEFT"),
        (Rect::new(646., 190., 50., 44.), "RIGHT"),
        (Rect::new(702., 190., 50., 44.), "UP"),
        (Rect::new(758., 190., 50., 44.), "DOWN"),
    ] {
        panel(rect, Color::new(0.18, 0.26, 0.34, 1.));
        text(label, rect.x + 5., 217., 9., WHITE);
    }
    text(
        &format!("Moves {}", game.moves),
        400.,
        260.,
        13.,
        crate::theme::SECONDARY,
    );
}

fn nonogram_grid(state: &AppState) -> GridLayout {
    let visible =
        crate::nonogram::visible_size(state.games.nonogram.size, state.games.nonogram_zoomed);
    GridLayout::new(Rect::new(70., 88., 280., 280.), visible, visible)
}

fn nonogram_origin(state: &AppState) -> (usize, usize) {
    crate::nonogram::focus_origin(
        state.games.nonogram.size,
        state.games.nonogram_zoomed,
        state.games.nonogram_focus,
    )
}

fn nonogram_global_index(state: &AppState, local: usize) -> usize {
    let visible =
        crate::nonogram::visible_size(state.games.nonogram.size, state.games.nonogram_zoomed);
    let (origin_x, origin_y) = nonogram_origin(state);
    origin_y * state.games.nonogram.size
        + origin_x
        + (local / visible) * state.games.nonogram.size
        + local % visible
}

pub fn nonogram_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(Rect::new(400., 135., 160., 44.), p) {
        return vec![UiAction::NonogramMode];
    }
    if crate::ui::hit(Rect::new(590., 135., 160., 44.), p) {
        return vec![UiAction::NonogramUndo];
    }
    if crate::ui::hit(Rect::new(400., 190., 160., 44.), p) {
        return vec![UiAction::NonogramZoom];
    }
    for (rect, delta) in [
        (Rect::new(590., 190., 50., 44.), (-1, 0)),
        (Rect::new(646., 190., 50., 44.), (1, 0)),
        (Rect::new(702., 190., 50., 44.), (0, -1)),
        (Rect::new(758., 190., 50., 44.), (0, 1)),
    ] {
        if rect.contains(p) {
            return vec![UiAction::NonogramPan(delta.0, delta.1)];
        }
    }
    if let Some(local) = nonogram_grid(state).index_at(p) {
        return vec![UiAction::NonogramCell(nonogram_global_index(state, local))];
    }
    vec![]
}

pub fn nonogram_drag_actions(state: &AppState, start: Vec2, end: Vec2) -> Vec<UiAction> {
    let layout = nonogram_grid(state);
    let (Some(start), Some(end)) = (layout.coordinate_at(start), layout.coordinate_at(end)) else {
        return vec![];
    };
    let (origin_x, origin_y) = nonogram_origin(state);
    crate::nonogram::stroke_indices(
        state.games.nonogram.size,
        (start.0 + origin_x, start.1 + origin_y),
        (end.0 + origin_x, end.1 + origin_y),
    )
    .into_iter()
    .map(UiAction::NonogramCell)
    .collect()
}
