//! Responsive touch presentation for Word Ladder.

use crate::{accessibility, state::AppState, ui::UiAction, word_ladder::{WordLadder, WordLadderPhase}};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout { keyboard: Rect, back: Rect, submit: Rect, hint: Rect, undo: Rect, new_game: Rect, columns: usize, key_w: f32 }

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() { Layout { keyboard: Rect::new(35., 290., 754., 84.), back: Rect::new(380., 85., 105., 42.), submit: Rect::new(495., 85., 105., 42.), hint: Rect::new(610., 85., 80., 42.), undo: Rect::new(700., 85., 80., 42.), new_game: Rect::new(610., 135., 170., 42.), columns: 13, key_w: 58. } }
    else if crate::ui::is_portrait() { Layout { keyboard: Rect::new(15., 520., 315., 126.), back: Rect::new(15., 425., 95., 42.), submit: Rect::new(115., 425., 95., 42.), hint: Rect::new(215., 425., 95., 42.), undo: Rect::new(15., 670., 95., 42.), new_game: Rect::new(115., 670., 145., 42.), columns: 9, key_w: 35. } }
    else { Layout { keyboard: Rect::new(790., 145., 468., 126.), back: Rect::new(790., 300., 105., 44.), submit: Rect::new(905., 300., 105., 44.), hint: Rect::new(1020., 300., 100., 44.), undo: Rect::new(1130., 300., 100., 44.), new_game: Rect::new(790., 360., 150., 44.), columns: 13, key_w: 36. } }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if Rect::new(0., 0., 110., 42.).contains(point) { return vec![UiAction::Cabinet]; }
    if l.back.contains(point) { return vec![UiAction::WordLadderBackspace]; }
    if l.submit.contains(point) { return vec![UiAction::WordLadderSubmit]; }
    if l.hint.contains(point) { return vec![UiAction::WordLadderHint]; }
    if l.undo.contains(point) { return vec![UiAction::WordLadderUndo]; }
    if l.new_game.contains(point) { return vec![UiAction::WordLadderNew]; }
    if l.keyboard.contains(point) { let index = ((point.y - l.keyboard.y) / 42.) as usize * l.columns + ((point.x - l.keyboard.x) / l.key_w) as usize; if index < 26 { return vec![UiAction::WordLadderLetter(index as u8)]; } }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout(); let game = &state.word_ladder; let portrait = crate::ui::is_portrait(); let compact = crate::ui::is_compact_landscape();
    let title_x = if compact { 70. } else if portrait { 25. } else { 390. }; let title_y = if compact { 28. } else if portrait { 62. } else { 58. };
    draw_text("‹ CABINET", 8., 30., accessibility::text_size(13., state.large_text), muted());
    draw_text("WORD LADDER", title_x, title_y, accessibility::text_size(if compact { 22. } else { 28. }, state.large_text), accent());
    draw_text(format!("{} moves  •  {} → {}", game.moves, game.start, game.target), if compact { 300. } else { title_x }, if compact { 28. } else { title_y + 24. }, accessibility::text_size(14., state.large_text), muted());
    if let Some(best) = state.records.word_ladder_best_moves {
        let best_x = if compact { 400. } else if portrait { 210. } else { title_x };
        draw_text(format!("BEST {} MOVES", best), best_x, if compact { 47. } else if portrait { 88. } else { title_y + 45. }, accessibility::text_size(12., state.large_text), Color::new(0.98, 0.75, 0.30, 1.));
    }
    draw_words(game, if compact { 55. } else if portrait { 15. } else { 390. }, 105., 300.);
    draw_keyboard(l, game, state.large_text); button(l.back, "DELETE", state.large_text); button(l.submit, "SUBMIT", state.large_text); button(l.hint, "HINT", state.large_text); button(l.undo, "UNDO", state.large_text); button(l.new_game, "NEW LADDER", state.large_text);
    draw_text(state.card_hint.as_deref().unwrap_or(&game.message), if compact { 40. } else { title_x }, if compact { 275. } else if portrait { 405. } else { 455. }, accessibility::text_size(15., state.large_text), muted());
}

fn draw_words(game: &WordLadder, x: f32, y: f32, width: f32) {
    let cell = (width / 5.).min(62.);
    let input_row = game.guesses.len() + 1;
    for row in 0..=input_row {
        let word = if row == 0 {
            Some(game.start.as_str())
        } else if row <= game.guesses.len() {
            Some(game.guesses[row - 1].as_str())
        } else {
            Some(game.current.as_str())
        };
        let yy = y + row as f32 * (cell + 7.);
        for col in 0..5 {
            let rect = Rect::new(x + col as f32 * cell, yy, cell - 5., cell - 5.);
            let current = row == input_row;
            draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if current {
                    Color::new(0.10, 0.18, 0.22, 1.)
                } else {
                    Color::new(0.12, 0.28, 0.28, 1.)
                },
            );
            draw_rectangle_lines(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                1.,
                Color::new(0.35, 0.65, 0.62, 1.),
            );
            if let Some(value) = word.and_then(|word| word.as_bytes().get(col)) {
                draw_text(
                    (*value as char).to_string(),
                    rect.x + rect.w * 0.35,
                    rect.y + rect.h * 0.68,
                    22.,
                    WHITE,
                );
            }
        }
    }
    if game.phase == WordLadderPhase::Won { draw_text("LADDER COMPLETE", x, y + 7. * (cell + 7.), 18., Color::new(0.55, 1., 0.72, 1.)); }
}

fn draw_keyboard(l: Layout, _game: &WordLadder, large_text: bool) { for index in 0..26 { let rect = Rect::new(l.keyboard.x + (index % l.columns) as f32 * l.key_w, l.keyboard.y + (index / l.columns) as f32 * 42., l.key_w - 3., 39.); draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.12, 0.18, 0.22, 1.)); draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., Color::new(0.35, 0.65, 0.62, 1.)); draw_text((char::from(b'A' + index as u8)).to_string(), rect.x + rect.w * 0.34, rect.y + rect.h * 0.68, accessibility::text_size(13., large_text), WHITE); } }
fn button(rect: Rect, label: &str, large_text: bool) { draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.16, 0.32, 0.34, 1.)); draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., Color::new(0.42, 0.78, 0.72, 1.)); draw_text(label, rect.x + 10., rect.y + rect.h * 0.64, accessibility::text_size(12., large_text), WHITE); }
fn accent() -> Color { Color::new(0.55, 1., 0.72, 1.) }
fn muted() -> Color { Color::new(0.58, 0.68, 0.68, 1.) }


