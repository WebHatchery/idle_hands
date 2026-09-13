use super::*;

pub(super) fn free_card_rect(x: f32, y: f32) -> Rect {
    Rect::new(x, y, 40., 54.)
}

pub(super) fn free_card_x(slot: usize) -> f32 {
    3. + slot as f32 * 45.
}

pub(super) fn free_foundation_x(suit: usize) -> f32 {
    182. + suit as f32 * 45.
}

pub fn draw_freecell(state: &AppState) {
    let game = &state.games.freecell;
    back();
    text("FREECELL", 10., 72., 29., crate::theme::BRASS);
    text(
        if game.status == crate::freecell::FreeCellStatus::Won {
            "All foundations complete"
        } else {
            "Every card stays in view"
        },
        12.,
        94.,
        13.,
        crate::theme::SECONDARY,
    );
    for cell in 0..4 {
        let rect = free_card_rect(free_card_x(cell), 112.);
        panel(rect, Color::new(0.12, 0.09, 0.20, 1.));
        if cell >= game.variant.free_cell_limit() {
            text(
                "SEALED",
                rect.x + 5.,
                rect.y + 33.,
                8.,
                crate::theme::SECONDARY,
            );
        } else if let Some(card) = game.cells[cell] {
            draw_card(
                rect,
                card,
                game.selected == Some(FreeSource::Cell(cell)),
                state.card_back,
                state.reduced_motion,
            );
        }
    }
    for suit in 0..4 {
        let rect = free_card_rect(free_foundation_x(suit), 112.);
        panel(rect, Color::new(0.12, 0.09, 0.20, 1.));
        if game.foundations[suit] > 0 {
            draw_card(
                rect,
                Card {
                    rank: game.foundations[suit],
                    suit: suit as u8,
                    face_up: true,
                },
                false,
                state.card_back,
                state.reduced_motion,
            );
        } else {
            crate::card_render::draw_suit_symbol(
                rect.center(),
                18.,
                suit as u8,
                crate::theme::BRASS,
            );
        }
    }
    text("CELLS", 5., 181., 9., Color::new(0.63, 0.58, 0.72, 1.));
    text(
        "FOUNDATIONS",
        185.,
        181.,
        9.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
    for cascade in 0..8 {
        let x = free_card_x(cascade);
        text(
            &(cascade + 1).to_string(),
            x + 17.,
            200.,
            10.,
            Color::new(0.63, 0.58, 0.72, 1.),
        );
        for (depth, card) in game.cascades[cascade].iter().enumerate() {
            draw_card(
                free_card_rect(x, 205. + depth as f32 * 17.),
                *card,
                game.selected == Some(FreeSource::Cascade(cascade, depth)),
                state.card_back,
                state.reduced_motion,
            );
        }
        if game.cascades[cascade].is_empty() {
            panel(free_card_rect(x, 205.), Color::new(0.12, 0.09, 0.20, 1.));
        }
    }
    text(
        &format!("Moves: {}", game.moves),
        10.,
        680.,
        13.,
        crate::theme::SECONDARY,
    );
    panel(
        Rect::new(120., 650., 105., 44.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("UNDO", 153., 679., 12., WHITE);
    panel(Rect::new(235., 650., 115., 44.), crate::theme::SURFACE);
    text("NEW DEAL", 263., 679., 11., WHITE);
    text(
        state
            .card_hint
            .as_deref()
            .unwrap_or("Tap a card, then a pile; tap again to release."),
        10.,
        620.,
        11.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
    panel(Rect::new(5., 650., 105., 44.), crate::theme::SURFACE);
    text("HINT", 39., 679., 12., WHITE);
}

pub fn freecell_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(Rect::new(5., 650., 105., 44.), p) {
        return vec![UiAction::FreeCellHint];
    }
    if crate::ui::hit(Rect::new(120., 650., 105., 44.), p) {
        return vec![UiAction::FreeCellUndo];
    }
    if crate::ui::hit(Rect::new(235., 650., 115., 44.), p) {
        return vec![UiAction::FreeCellNew];
    }
    for cell in 0..4 {
        if cell < state.games.freecell.variant.free_cell_limit()
            && free_card_rect(free_card_x(cell), 112.).contains(p)
        {
            return vec![UiAction::FreeCellCell(cell)];
        }
    }
    for suit in 0..4 {
        if free_card_rect(free_foundation_x(suit), 112.).contains(p) {
            return vec![UiAction::FreeCellFoundation(suit)];
        }
    }
    for cascade in 0..8 {
        let x = free_card_x(cascade);
        if p.x >= x && p.x <= x + 40. && p.y >= 195. {
            let depth = if state.games.freecell.cascades[cascade].is_empty() {
                0
            } else {
                (((p.y - 205.) / 17.).floor().max(0.) as usize)
                    .min(state.games.freecell.cascades[cascade].len() - 1)
            };
            return vec![UiAction::FreeCellCascade(cascade, depth)];
        }
    }
    vec![]
}
