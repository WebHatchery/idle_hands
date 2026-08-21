use crate::blackjack::{Blackjack, BlackjackHint};
use crate::freecell::FreeCell;
use crate::klondike_golf::KlondikeGolf;
use crate::pyramid::Pyramid;
use crate::solitaire::Solitaire;
use crate::spider::Spider;
use crate::spider_solitaire::is_run as is_spider_solitaire_run;
use crate::spider_solitaire::SpiderSolitaire;
use crate::state::GameId;
use crate::tri_peaks::TriPeaks;

use super::{assert_serializable, SESSION_SEED};

pub(super) fn run(game: GameId) {
    match game {
        GameId::Solitaire => solitaire(),
        GameId::FreeCell => freecell(),
        GameId::Spider => spider(),
        GameId::KlondikeGolf => klondike_golf(),
        GameId::Blackjack => blackjack(),
        GameId::SpiderSolitaire => spider_solitaire(),
        GameId::Pyramid => pyramid(),
        GameId::TriPeaks => tri_peaks(),
        _ => panic!("card session was assigned the wrong game"),
    }
}

fn solitaire() {
    let mut game = Solitaire::new(SESSION_SEED);
    for _ in 0..160 {
        game.draw_stock();
    }
    assert!(game.moves >= 160);
    assert_eq!(
        game.tableau.iter().map(Vec::len).sum::<usize>() + game.stock.len() + game.waste.len(),
        52
    );
    assert_serializable(&game);
}

fn freecell() {
    let mut game = FreeCell::new(SESSION_SEED);
    let mut moved = 0;
    for _ in 0..128 {
        let mut action_taken = false;
        for source in 0..game.cascades.len() {
            let Some(depth) = game.cascades[source].len().checked_sub(1) else {
                continue;
            };
            if !game.select_cascade(source, depth) {
                continue;
            }
            for destination in 0..game.cascades.len() {
                if game.move_selected_to_cascade(destination) {
                    moved += 1;
                    action_taken = true;
                    break;
                }
            }
            if action_taken {
                break;
            }
            for suit in 0..4 {
                if game.move_selected_to_foundation(suit) {
                    moved += 1;
                    action_taken = true;
                    break;
                }
            }
            if action_taken {
                break;
            }
        }
        if !action_taken {
            break;
        }
    }
    assert!(
        moved > 0,
        "FreeCell should keep accepting legal desktop moves"
    );
    assert_serializable(&game);
}

fn spider() {
    let mut game = Spider::new(SESSION_SEED);
    let mut actions = 0;
    for _ in 0..160 {
        if let Some((source, depth, destination)) = game.hint_move() {
            assert!(game.select_column(source, depth));
            assert!(game.move_selected(destination));
            actions += 1;
        } else if game.deal_stock() {
            actions += 1;
        } else {
            break;
        }
    }
    assert!(
        actions >= 16,
        "Spider should survive a long sequence of moves"
    );
    assert_serializable(&game);
}

fn klondike_golf() {
    let mut game = KlondikeGolf::new(SESSION_SEED);
    let mut actions = 0;
    for round in 0..6 {
        for _ in 0..64 {
            let mut played = false;
            for column in 0..game.tableau.len() {
                if game.tap_column(column) {
                    played = true;
                    actions += 1;
                    break;
                }
            }
            if !played {
                if game.draw_stock() {
                    actions += 1;
                } else {
                    break;
                }
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        actions >= 60,
        "Golf should keep accepting tableau and stock actions"
    );
    assert_serializable(&game);
}

fn blackjack() {
    let mut game = Blackjack::new(SESSION_SEED);
    let mut rounds = 0;
    for round in 0..32 {
        while let Some(BlackjackHint::Hit) = game.hint_action() {
            if !game.hit() {
                break;
            }
        }
        if matches!(game.hint_action(), Some(BlackjackHint::Stand)) {
            assert!(game.stand());
        }
        rounds += 1;
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(rounds >= 32);
    assert!(game.rounds >= 32);
    assert_serializable(&game);
}

fn spider_solitaire() {
    let mut game = SpiderSolitaire::new(SESSION_SEED);
    let mut actions = 0;
    for _ in 0..160 {
        if let Some((source, depth, destination)) = find_spider_solitaire_move(&game) {
            assert!(game.select_column(source, depth));
            assert!(game.move_selected(destination));
            actions += 1;
        } else if game.deal_stock() {
            actions += 1;
        } else {
            break;
        }
    }
    assert!(
        actions >= 16,
        "Spider Solitaire should survive a long sequence"
    );
    assert_serializable(&game);
}

fn find_spider_solitaire_move(game: &SpiderSolitaire) -> Option<(usize, usize, usize)> {
    for source in 0..game.tableau.len() {
        for depth in 0..game.tableau[source].len() {
            let Some(card) = game.tableau[source].get(depth) else {
                continue;
            };
            if !is_spider_solitaire_run(&game.tableau[source][depth..]) {
                continue;
            }
            for destination in 0..game.tableau.len() {
                if source == destination {
                    continue;
                }
                let playable = game.tableau[destination]
                    .last()
                    .is_none_or(|top| top.rank == card.rank + 1);
                if playable {
                    return Some((source, depth, destination));
                }
            }
        }
    }
    None
}

fn pyramid() {
    let mut game = Pyramid::new(SESSION_SEED);
    let mut actions = 0;
    for round in 0..6 {
        for _ in 0..64 {
            let mut paired = false;
            for first in 0..game.pyramid.len() {
                for second in first + 1..game.pyramid.len() {
                    if game.legal_pair(first, second) {
                        assert!(game.tap(first));
                        assert!(game.tap(second));
                        actions += 1;
                        paired = true;
                        break;
                    }
                }
                if paired {
                    break;
                }
            }
            if !paired && game.draw_stock() {
                actions += 1;
            } else if !paired {
                break;
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        actions >= 48,
        "Pyramid should continue through stock and tableau actions"
    );
    assert_serializable(&game);
}

fn tri_peaks() {
    let mut game = TriPeaks::new(SESSION_SEED);
    let mut actions = 0;
    for round in 0..6 {
        for _ in 0..72 {
            let mut played = false;
            for index in 0..game.tableau.len() {
                if game.can_play(index) && game.tap(index) {
                    actions += 1;
                    played = true;
                    break;
                }
            }
            if !played {
                if game.draw_stock() {
                    actions += 1;
                } else {
                    break;
                }
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        actions >= 48,
        "TriPeaks should continue through stock and tableau actions"
    );
    assert_serializable(&game);
}
