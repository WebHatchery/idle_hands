use super::*;

const REVERSI_BOARD: Rect = Rect {
    x: 20.,
    y: 118.,
    w: 320.,
    h: 320.,
};

pub fn draw_reversi(state: &AppState) {
    let game = &state.games.reversi;
    back();
    text(
        "REVERSI",
        10.,
        72.,
        accessibility::text_size(29., state.large_text),
        crate::theme::BRASS,
    );
    text(
        reversi_subtitle(),
        accessibility::text_size(12., state.large_text),
        94.,
        accessibility::text_size(12., state.large_text),
        crate::theme::SECONDARY,
    );
    text(
        &format!("DARK {}  -  LIGHT {}", game.score(1), game.score(2)),
        20.,
        110.,
        12.,
        crate::theme::BRASS,
    );
    panel(
        REVERSI_BOARD,
        if state.high_contrast {
            Color::new(0.02, 0.20, 0.16, 1.)
        } else {
            Color::new(0.10, 0.30, 0.24, 1.)
        },
    );
    let cell = REVERSI_BOARD.w / 8.;
    let legal = if game.status == ReversiStatus::Playing {
        game.legal_moves(game.turn)
    } else {
        Vec::new()
    };
    for index in 0..64 {
        let row = index / 8;
        let col = index % 8;
        let rect = Rect::new(
            REVERSI_BOARD.x + col as f32 * cell,
            REVERSI_BOARD.y + row as f32 * cell,
            cell,
            cell,
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            accessibility::grid_line(state.high_contrast),
        );
        if game.board[index] == 0 && legal.contains(&index) {
            draw_circle(
                rect.center().x,
                rect.center().y,
                5.,
                if state.high_contrast {
                    WHITE
                } else {
                    Color::new(0.72, 0.95, 0.72, 0.75)
                },
            );
        }
        if game.board[index] != 0 {
            draw_circle(
                rect.center().x,
                rect.center().y,
                cell * 0.34,
                if game.board[index] == 1 {
                    accessibility::board_fill(state.high_contrast)
                } else {
                    if state.high_contrast {
                        WHITE
                    } else {
                        Color::new(0.92, 0.85, 0.66, 1.)
                    }
                },
            );
            draw_circle_lines(
                rect.center().x,
                rect.center().y,
                cell * 0.34,
                2.,
                accessibility::grid_line(state.high_contrast),
            );
        }
    }
    text(
        match game.status {
            ReversiStatus::Playing if game.ai_level == AiLevel::TwoPlayer => {
                if game.turn == 1 {
                    "Player 1: tap a glowing square"
                } else {
                    "Player 2: tap a glowing square"
                }
            }
            ReversiStatus::Playing if game.turn == 1 => "Your turn: tap a glowing square",
            ReversiStatus::Playing => "Opponent is thinking",
            ReversiStatus::Won => match game.winner {
                Some(1) => "Dark wins the board",
                Some(2) => "Light wins the board",
                _ => "The board is tied",
            },
        },
        accessibility::text_size(12., state.large_text),
        465.,
        accessibility::text_size(12., state.large_text),
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    panel(Rect::new(10., 485., 160., 44.), crate::theme::SURFACE_DARK);
    text(
        "PASS TURN",
        56.,
        514.,
        accessibility::text_size(12., state.large_text),
        WHITE,
    );
    panel(Rect::new(185., 485., 165., 44.), crate::theme::SURFACE);
    text(
        "NEW BOARD",
        229.,
        514.,
        accessibility::text_size(12., state.large_text),
        WHITE,
    );
    panel(
        Rect::new(10., 540., 105., 44.),
        if game.ai_level == AiLevel::Gentle {
            Color::new(0.45, 0.25, 0.42, 1.)
        } else {
            crate::theme::SURFACE_DARK
        },
    );
    panel(
        Rect::new(127., 540., 105., 44.),
        if game.ai_level == AiLevel::Sharp {
            Color::new(0.45, 0.25, 0.42, 1.)
        } else {
            crate::theme::SURFACE_DARK
        },
    );
    panel(
        Rect::new(244., 540., 106., 44.),
        if game.ai_level == AiLevel::TwoPlayer {
            Color::new(0.45, 0.25, 0.42, 1.)
        } else {
            crate::theme::SURFACE_DARK
        },
    );
    text(
        "GENTLE",
        40.,
        568.,
        accessibility::text_size(11., state.large_text),
        WHITE,
    );
    text(
        "SHARP",
        160.,
        568.,
        accessibility::text_size(11., state.large_text),
        WHITE,
    );
    text(
        "2 PLAYER",
        263.,
        568.,
        accessibility::text_size(11., state.large_text),
        WHITE,
    );
    text(
        "Pass is available when no legal move remains.",
        10.,
        620.,
        accessibility::text_size(11., state.large_text),
        Color::new(0.63, 0.58, 0.72, 1.),
    );
}

pub(super) fn reversi_subtitle() -> &'static str {
    if crate::ui::is_portrait() {
        "Tap a glowing square"
    } else {
        "Turn the board, one careful move at a time"
    }
}

pub fn reversi_clicks(_state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(Rect::new(10., 485., 160., 44.), p) {
        return vec![UiAction::ReversiPass];
    }
    if crate::ui::hit(Rect::new(185., 485., 165., 44.), p) {
        return vec![UiAction::ReversiNew];
    }
    if crate::ui::hit(Rect::new(10., 540., 105., 44.), p) {
        return vec![UiAction::ReversiLevel(AiLevel::Gentle)];
    }
    if crate::ui::hit(Rect::new(127., 540., 105., 44.), p) {
        return vec![UiAction::ReversiLevel(AiLevel::Sharp)];
    }
    if crate::ui::hit(Rect::new(244., 540., 106., 44.), p) {
        return vec![UiAction::ReversiLevel(AiLevel::TwoPlayer)];
    }
    if !REVERSI_BOARD.contains(p) {
        return vec![];
    }
    let cell = REVERSI_BOARD.w / 8.;
    let col = ((p.x - REVERSI_BOARD.x) / cell) as usize;
    let row = ((p.y - REVERSI_BOARD.y) / cell) as usize;
    if row < 8 && col < 8 {
        vec![UiAction::ReversiPlace(row * 8 + col)]
    } else {
        vec![]
    }
}
