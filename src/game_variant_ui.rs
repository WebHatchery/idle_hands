//! Shared rule-card control and round-setup disclosure drawn above game drawers.

use crate::{
    game_variants,
    state::{AppState, GameId, Screen},
    ui::UiAction,
    variant_card_data,
};
use macroquad::prelude::*;

pub fn button_rect() -> Rect {
    let (width, _) = crate::ui::layout_size();
    variant_card_data::layout(
        width,
        crate::ui::is_portrait(),
        crate::ui::is_compact_landscape(),
    )
    .rect
}

pub fn clicks(state: &AppState, point: Vec2) -> bool {
    matches!(state.screen, Screen::Game(_)) && crate::ui::hit(button_rect(), point)
}

pub fn setup_clicks(state: &AppState, point: Vec2) -> Option<UiAction> {
    let Screen::Game(game) = state.screen else {
        return None;
    };
    let panel = setup_panel_rect(game);
    if crate::ui::hit(setup_close_rect(panel), point) {
        return Some(UiAction::ToggleGameSetup);
    }
    if crate::ui::hit(setup_next_rule_rect(panel), point) {
        return Some(UiAction::CycleGameVariant);
    }
    if let Some(action) = setup_choice_action(state, game, panel, point) {
        return Some(action);
    }
    if let Some(action) = setup_new_round_action(state, game, panel, point) {
        return Some(action);
    }
    (!panel.contains(point)).then_some(UiAction::ToggleGameSetup)
}

pub fn draw(state: &AppState) {
    let Screen::Game(game) = state.screen else {
        return;
    };
    if crate::dense_focus_ui::is_open(state) {
        return;
    }
    let rect = button_rect();
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let layout = variant_card_data::layout(crate::ui::layout_size().0, portrait, compact);
    let title_size = crate::accessibility::text_size(layout.title_size, state.large_text);
    let label_size = crate::accessibility::text_size(layout.label_size, state.large_text);
    let ink = if state.high_contrast {
        WHITE
    } else {
        crate::theme::BRASS
    };
    crate::ui::draw_rounded_panel(rect, 7., crate::theme::GAME_PANEL, ink);
    let label = macroquad_toolkit::ui::truncate_text_to_width(
        &game_variants::label(state, game),
        layout.label_rect.w,
        label_size,
    );
    crate::ui::draw_text(
        "RULE CARD  ›",
        rect.x + 9.,
        layout.title_baseline,
        title_size,
        ink,
    );
    crate::ui::draw_text(
        &label,
        layout.label_rect.x,
        layout.label_baseline,
        label_size,
        if state.high_contrast {
            WHITE
        } else {
            crate::theme::CREAM
        },
    );
    draw_icon(game, rect, state.high_contrast);
    if state.game_setup_open && !(state.confirm_restart && state.pending_restart.is_some()) {
        draw_setup(state, game);
    }
}

pub fn setup_panel_rect(game: GameId) -> Rect {
    let (width, height) = crate::ui::layout_size();
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let panel_width = if portrait {
        (width - 24.).min(420.)
    } else if compact {
        (width - 24.).min(430.)
    } else {
        430.
    };
    let panel_height: f32 = match setup_option_count(game) {
        4 => 370.,
        3 => 370.,
        2 => 320.,
        _ if game == GameId::Solitaire => 300.,
        _ => 276.,
    };
    let top = if compact {
        10.
    } else {
        ((height - panel_height) * 0.5).max(48.)
    };
    Rect::new(
        (width - panel_width) * 0.5,
        top,
        panel_width,
        panel_height.min(height - top - 8.),
    )
}

pub fn setup_next_rule_rect(panel: Rect) -> Rect {
    Rect::new(panel.x + 20., panel.y + 92., (panel.w - 52.) * 0.5, 46.)
}

pub fn setup_close_rect(panel: Rect) -> Rect {
    Rect::new(
        setup_next_rule_rect(panel).right() + 12.,
        panel.y + 92.,
        (panel.w - 52.) * 0.5,
        46.,
    )
}

pub fn setup_option_count(game: GameId) -> usize {
    match game {
        GameId::Game2048 => crate::game_2048::Game2048Size::ALL.len(),
        GameId::Sudoku => crate::sudoku::SudokuDifficulty::ALL.len(),
        GameId::Nonogram => crate::nonogram::NonogramPreset::ALL.len(),
        _ => 0,
    }
}

pub fn setup_option_rect(panel: Rect, index: usize) -> Rect {
    let gap = 12.;
    let width = (panel.w - 40. - gap) * 0.5;
    Rect::new(
        panel.x + 20. + (index % 2) as f32 * (width + gap),
        panel.y + 182. + (index / 2) as f32 * 58.,
        width,
        46.,
    )
}

pub fn setup_new_round_rect(panel: Rect, option_count: usize) -> Rect {
    let rows = option_count.div_ceil(2);
    Rect::new(
        panel.x + 20.,
        panel.y + 200. + rows as f32 * 58.,
        panel.w - 40.,
        48.,
    )
}

pub fn setup_choice_action(
    state: &AppState,
    game: GameId,
    panel: Rect,
    point: Vec2,
) -> Option<UiAction> {
    for index in 0..setup_option_count(game) {
        if !crate::ui::hit(setup_option_rect(panel, index), point) {
            continue;
        }
        return match game {
            GameId::Game2048 => Some(UiAction::Game2048Size(
                crate::game_2048::Game2048Size::ALL[index],
            )),
            GameId::Sudoku => Some(UiAction::SudokuDifficulty(
                crate::sudoku::SudokuDifficulty::ALL[index],
            )),
            GameId::Nonogram => Some(UiAction::NonogramPreset(
                crate::nonogram::NonogramPreset::ALL[index],
            )),
            _ => None,
        };
    }
    let _ = state;
    None
}

pub fn setup_new_round_action(
    state: &AppState,
    game: GameId,
    panel: Rect,
    point: Vec2,
) -> Option<UiAction> {
    if !crate::ui::hit(setup_new_round_rect(panel, setup_option_count(game)), point) {
        return None;
    }
    match game {
        GameId::Game2048 => Some(UiAction::Restart),
        GameId::Solitaire => Some(UiAction::SolitaireNew),
        GameId::Sudoku => Some(UiAction::SudokuDifficulty(state.games.sudoku.difficulty)),
        GameId::Nonogram => Some(UiAction::NonogramPreset(state.games.nonogram.preset)),
        _ => None,
    }
}

pub fn draw_setup(state: &AppState, game: GameId) {
    let panel = setup_panel_rect(game);
    let (width, height) = crate::ui::layout_size();
    draw_rectangle(0., 0., width, height, Color::new(0.02, 0.01, 0.04, 0.72));
    crate::ui::draw_rounded_panel(
        panel,
        10.,
        if state.high_contrast {
            crate::theme::SURFACE_DARK
        } else {
            Color::new(0.12, 0.07, 0.18, 0.99)
        },
        if state.high_contrast {
            WHITE
        } else {
            crate::theme::BRASS
        },
    );
    text_setup(
        "ROUND SETUP",
        panel.x + 20.,
        panel.y + 40.,
        22.,
        crate::theme::BRASS,
        state,
    );
    let active = macroquad_toolkit::ui::truncate_text_to_width(
        &game_variants::label(state, game),
        panel.w - 40.,
        crate::accessibility::text_size(15., state.large_text),
    );
    text_setup(
        &format!("ACTIVE RULE  {active}"),
        panel.x + 20.,
        panel.y + 70.,
        15.,
        crate::theme::CREAM,
        state,
    );
    setup_button(
        setup_next_rule_rect(panel),
        "NEXT RULE",
        crate::theme::SURFACE,
        state,
    );
    setup_button(
        setup_close_rect(panel),
        "DONE",
        crate::theme::SURFACE,
        state,
    );
    let count = setup_option_count(game);
    if count > 0 {
        let heading = match game {
            GameId::Game2048 => "BOARD SIZE",
            GameId::Sudoku => "DIFFICULTY",
            GameId::Nonogram => "PUZZLE SIZE",
            _ => "",
        };
        text_setup(
            heading,
            panel.x + 20.,
            panel.y + 164.,
            13.,
            crate::theme::BRASS,
            state,
        );
        for index in 0..count {
            let (label, selected) = match game {
                GameId::Game2048 => {
                    let value = crate::game_2048::Game2048Size::ALL[index];
                    (
                        value.label().to_owned(),
                        value == state.games.game.board_size,
                    )
                }
                GameId::Sudoku => {
                    let value = crate::sudoku::SudokuDifficulty::ALL[index];
                    (
                        value.label().to_owned(),
                        value == state.games.sudoku.difficulty,
                    )
                }
                GameId::Nonogram => {
                    let value = crate::nonogram::NonogramPreset::ALL[index];
                    (
                        value.label().to_owned(),
                        value == state.games.nonogram.preset,
                    )
                }
                _ => (String::new(), false),
            };
            setup_button(
                setup_option_rect(panel, index),
                &label,
                if selected {
                    crate::theme::LEATHER
                } else {
                    crate::theme::GAME_PANEL
                },
                state,
            );
        }
    } else if game == GameId::Solitaire {
        text_setup(
            "RULES ARE SHOWN HERE BEFORE THE DEAL.",
            panel.x + 20.,
            panel.y + 166.,
            13.,
            crate::theme::SECONDARY,
            state,
        );
    }
    if let Some(label) = new_round_label(game) {
        let rect = setup_new_round_rect(panel, count);
        setup_button(rect, label, crate::theme::SURFACE_DARK, state);
    }
}

pub fn new_round_label(game: GameId) -> Option<&'static str> {
    match game {
        GameId::Game2048 => Some("NEW BOARD"),
        GameId::Solitaire => Some("NEW DEAL"),
        GameId::Sudoku | GameId::Nonogram => Some("NEW PUZZLE"),
        _ => None,
    }
}

pub fn text_setup(value: &str, x: f32, y: f32, size: f32, color: Color, state: &AppState) {
    crate::ui::draw_text(
        value,
        x,
        y,
        crate::accessibility::text_size(size, state.large_text),
        if state.high_contrast { WHITE } else { color },
    );
}

pub fn setup_button(rect: Rect, label: &str, fill: Color, state: &AppState) {
    crate::ui::draw_rounded_panel(
        rect,
        6.,
        fill,
        if state.high_contrast {
            WHITE
        } else {
            crate::theme::BORDER
        },
    );
    let size = crate::accessibility::text_size(14., state.large_text);
    let width = crate::ui::measure_text(label, None, size.round() as u16, 1.).width;
    crate::ui::draw_text(
        label,
        rect.x + (rect.w - width) * 0.5,
        rect.y + rect.h * 0.64,
        size,
        WHITE,
    );
}

pub fn draw_icon(game: GameId, rect: Rect, high_contrast: bool) {
    let center = vec2(rect.right() - 17., rect.y + rect.h * 0.52);
    let ink = if high_contrast {
        WHITE
    } else {
        crate::theme::BRASS
    };
    match game {
        GameId::Solitaire
        | GameId::FreeCell
        | GameId::Yahtzee
        | GameId::KlondikeGolf
        | GameId::Blackjack
        | GameId::SpiderSolitaire
        | GameId::Pyramid
        | GameId::TriPeaks => {
            draw_rectangle_lines(center.x - 9., center.y - 11., 18., 22., 1.5, ink);
            draw_line(
                center.x - 5.,
                center.y - 4.,
                center.x + 5.,
                center.y - 4.,
                1.5,
                ink,
            );
            draw_circle(center.x, center.y + 5., 2.5, ink);
        }
        GameId::Hangman
        | GameId::WordSearch
        | GameId::WordGrid
        | GameId::WordLadder
        | GameId::Nonogram
        | GameId::Mastermind => {
            for row in 0..3 {
                draw_line(
                    center.x - 9.,
                    center.y - 7. + row as f32 * 7.,
                    center.x + 9.,
                    center.y - 7. + row as f32 * 7.,
                    1.5,
                    ink,
                );
            }
            draw_circle(center.x - 5., center.y - 7., 1.5, ink);
            draw_circle(center.x - 5., center.y, 1.5, ink);
            draw_circle(center.x - 5., center.y + 7., 1.5, ink);
        }
        GameId::Breakout
        | GameId::Snake
        | GameId::TinyTowerDefence
        | GameId::SpaceInvaders
        | GameId::Asteroids
        | GameId::Frogger
        | GameId::MunchMaze
        | GameId::BlockStack
        | GameId::TerrainCannon
        | GameId::FlingFury
        | GameId::PaddleDuel => {
            draw_rectangle(center.x - 9., center.y - 8., 18., 3., ink);
            draw_circle(center.x, center.y + 2., 3., ink);
            draw_line(
                center.x - 9.,
                center.y + 9.,
                center.x + 9.,
                center.y + 9.,
                2.,
                ink,
            );
        }
        _ => {
            draw_rectangle_lines(center.x - 9., center.y - 9., 18., 18., 1.5, ink);
            draw_line(center.x - 8., center.y, center.x + 8., center.y, 1.5, ink);
            draw_line(center.x, center.y - 8., center.x, center.y + 8., 1.5, ink);
        }
    }
}
