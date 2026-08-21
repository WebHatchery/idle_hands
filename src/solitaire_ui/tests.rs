use super::*;

#[test]
fn tableau_peek_maps_top_middle_and_last_cards() {
    let mut game = Solitaire::default();
    game.tableau[0] = (0..13)
        .map(|rank| Card {
            rank: rank + 1,
            suit: 3,
            face_up: true,
        })
        .collect();
    let gap = tableau_gap(&game);

    assert_eq!(tableau_card_at(&game, vec2(40., TABLEAU_TOP)), Some((0, 0)));
    assert_eq!(
        tableau_card_at(&game, vec2(40., TABLEAU_TOP + gap * 6. + 1.)),
        Some((0, 6))
    );
    assert_eq!(
        tableau_card_at(&game, vec2(40., TABLEAU_BOTTOM)),
        Some((0, 12))
    );
    assert_eq!(tableau_card_at(&game, vec2(40., TABLEAU_BOTTOM + 1.)), None);
    assert_eq!(tableau_card_at(&game, vec2(10., TABLEAU_TOP)), None);

    let short_game = Solitaire::default();
    assert_eq!(tableau_card_at(&short_game, vec2(40., 500.)), None);
}

#[test]
fn long_desktop_tableau_keeps_its_bottom_card_targetable() {
    let mut game = Solitaire::default();
    game.tableau[0] = (0..30)
        .map(|rank| Card {
            rank: (rank % 13) + 1,
            suit: rank % 4,
            face_up: true,
        })
        .collect();

    let gap = tableau_gap(&game);
    let bottom_y = TABLEAU_TOP + gap * 29. + 1.;
    assert_eq!(tableau_card_at(&game, vec2(40., bottom_y)), Some((0, 29)));
}
