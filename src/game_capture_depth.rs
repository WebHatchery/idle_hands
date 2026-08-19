//! Curated capture fixtures for the post-launch system-depth passes.

use crate::state::AppState;

pub(super) fn apply(state: &mut AppState, scene: &str) {
    match scene {
        "potion_catalyst" => potion_catalyst(state),
        "dots_tactics" => dots_tactics(state),
        "sokoban_deadlock" => sokoban_deadlock(state),
        "mancala_tactics" => mancala_tactics(state),
        "hanoi_master" => hanoi_master(state),
        "number_match_links" => number_match_links(state),
        "flood_surges" => flood_surges(state),
        "color_sort_runs" => color_sort_runs(state),
        "battleship_sonar" => battleship_sonar(state),
        "word_grid_deduction" => word_grid_deduction(state),
        "pipe_network" => pipe_network(state),
        "maze_beacons" => maze_beacons(state),
        "nim_tactics" => nim_tactics(state),
        "word_ladder_routes" => word_ladder_routes(state),
        "pyramid_chains" => pyramid_chains(state),
        _ => {}
    }
}

fn potion_catalyst(state: &mut AppState) {
    use crate::potion_2048::{Potion2048, PotionDifficulty};
    let game = &mut state.potion_2048;
    *game = Potion2048::new_with_difficulty(0xB071_2050, PotionDifficulty::Expert);
    game.cells = vec![0; 36];
    for (index, value) in [
        (5, 2),
        (10, 4),
        (11, 8),
        (14, 1),
        (16, 16),
        (17, 32),
        (20, 64),
        (21, 128),
        (26, 256),
        (32, 512),
    ] {
        game.cells[index] = value;
    }
    game.score = 1_840;
    game.best = 2_460;
    game.combo = 4;
    game.best_combo = 6;
    game.catalysts_brewed = 2;
    game.last_merges = 2;
}

fn dots_tactics(state: &mut AppState) {
    use crate::dots_boxes::{DotsBoxes, DotsDifficulty};
    let game = &mut state.dots_boxes;
    *game = DotsBoxes::new_with_difficulty(0x00D0_7B12, DotsDifficulty::Hard);
    game.scores = [1, 1];
    game.moves = 18;
    game.boxes[0] = 1;
    game.boxes[1] = 2;
    for (index, owner) in [(0, 1), (1, 2), (5, 2), (6, 1), (12, 1), (17, 2)] {
        game.horizontal[index] = true;
        game.horizontal_owners[index] = owner;
    }
    for (index, owner) in [(0, 1), (1, 2), (2, 2), (14, 1)] {
        game.vertical[index] = true;
        game.vertical_owners[index] = owner;
    }
}

fn sokoban_deadlock(state: &mut AppState) {
    use crate::sokoban::{Sokoban, SokobanPhase, HEIGHT, WIDTH};
    let game = &mut state.sokoban;
    *game = Sokoban::new_with_level(0x50C0_BA0B, 4);
    game.tiles = vec![1; WIDTH * HEIGHT];
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            if row == 0 || col == 0 || row + 1 == HEIGHT || col + 1 == WIDTH {
                game.tiles[row * WIDTH + col] = 0;
            }
        }
    }
    game.player = 4 * WIDTH + 4;
    game.crates = 2;
    game.tiles[WIDTH + 1] = 3;
    game.tiles[WIDTH + 5] = 2;
    game.tiles[2 * WIDTH + 4] = 3;
    game.tiles[2 * WIDTH + 5] = 2;
    game.moves = 11;
    game.pushes = 4;
    game.phase = SokobanPhase::Stuck;
}

fn mancala_tactics(state: &mut AppState) {
    use crate::mancala::{AiLevel, Mancala, MancalaVariant};
    let game = &mut state.mancala;
    *game = Mancala::new_with_variant(0x4D41_4E43_4100, MancalaVariant::Grand);
    game.ai_level = AiLevel::Expert;
    game.pits = vec![0; 14];
    for (index, stones) in [
        (0, 1),
        (1, 0),
        (2, 7),
        (3, 2),
        (4, 1),
        (5, 1),
        (6, 16),
        (7, 4),
        (8, 2),
        (9, 6),
        (10, 0),
        (11, 5),
        (12, 3),
        (13, 12),
    ] {
        game.pits[index] = stones;
    }
    game.moves = 14;
    game.captured_stones = 11;
    game.extra_turns = 3;
}

fn hanoi_master(state: &mut AppState) {
    use crate::hanoi::Hanoi;
    let game = &mut state.hanoi;
    *game = Hanoi::new_with_disks(0x0048_414E_4F49, 7);
    game.stacks = [vec![7, 6, 5], vec![4, 3], vec![2, 1]];
    game.selected = Some(2);
    game.moves = 18;
}

fn number_match_links(state: &mut AppState) {
    use crate::number_match::{LinkRule, NumberMatch};
    let game = &mut state.number_match;
    *game = NumberMatch::new_with_rule(0x4E55_4D42_4552, LinkRule::Lines);
    game.cells = vec![0; 36];
    for (index, value) in [
        (0, 4),
        (5, 6),
        (7, 3),
        (10, 3),
        (12, 8),
        (13, 2),
        (20, 5),
        (26, 5),
        (30, 1),
        (31, 9),
    ] {
        game.cells[index] = value;
    }
    game.selected = Some(0);
    game.moves = 13;
    game.score = 13;
    game.points = 280;
    game.combo = 4;
    game.best_combo = 6;
    game.remixes_left = 1;
}

fn flood_surges(state: &mut AppState) {
    use crate::flood_it::{FloodDifficulty, FloodIt};
    let game = &mut state.flood_it;
    *game = FloodIt::new_with_difficulty(0x0046_4C4F_4F44_4954, FloodDifficulty::Expert);
    let side = game.side();
    for row in 0..side {
        for col in 0..side {
            game.cells[row * side + col] = ((row / 2 + col / 3) % 8) as u8;
        }
    }
    game.active_color = game.cells[0];
    game.moves = 17;
    game.last_gain = 9;
    game.combo = 5;
    game.best_combo = 7;
    game.points = 486;
    game.momentum = 2;
    game.surges = 1;
}

fn color_sort_runs(state: &mut AppState) {
    use crate::color_sort::{ColorSort, ColorSortDifficulty};
    let game = &mut state.color_sort;
    *game = ColorSort::new_with_difficulty(0x0043_4F4C_4F52, ColorSortDifficulty::Expert);
    game.tubes = vec![
        vec![0, 1, 1],
        vec![2, 3, 3],
        vec![4, 5],
        vec![],
        vec![0, 0, 0, 0],
        vec![2, 2],
        vec![4, 4],
        vec![5, 5],
    ];
    game.selected = Some(0);
    game.moves = 22;
    game.last_poured = 2;
    game.combo = 3;
    game.best_combo = 5;
    game.points = 118;
}

fn battleship_sonar(state: &mut AppState) {
    use crate::battleship::{Battleship, Shot};
    let game = &mut state.battleship;
    *game = Battleship::new(0xBA77_1E50);
    game.shots[1] = Shot::Hit;
    game.shots[2] = Shot::Hit;
    game.shots[14] = Shot::Hit;
    game.shots[20] = Shot::Hit;
    game.shots[0] = Shot::Miss;
    game.shots[6] = Shot::Miss;
    game.scanned = vec![false; crate::battleship::CELLS];
    for cell in [7, 8, 9, 13, 14, 15, 19, 20, 21, 27, 28, 29, 33, 34, 35] {
        game.scanned[cell] = true;
    }
    game.moves = 6;
    game.streak = 2;
    game.best_streak = 3;
    game.score = 105;
    game.sonar_charges = 1;
}

fn word_grid_deduction(state: &mut AppState) {
    use crate::word_grid::{WordGrid, WordGridMode};
    let game = &mut state.word_grid;
    *game = WordGrid::new_with_mode(0, WordGridMode::Hard);
    for word in ["SHELF", "SMALL"] {
        for letter in word.bytes() {
            game.tap_letter(letter - b'A');
        }
        game.submit();
    }
    for letter in "STI".bytes() {
        game.tap_letter(letter - b'A');
    }
}

fn pipe_network(state: &mut AppState) {
    use crate::pipe_loop::{PipeLoop, PipePattern, PipePhase};
    let game = &mut state.pipe_loop;
    *game = PipeLoop::new_with_pattern(0x715E, PipePattern::Trunk);
    game.pipes = game.solution.clone();
    game.phase = PipePhase::Playing;
    for index in [13, 18, 24] {
        game.rotate(index);
    }
    game.moves = 17;
}

fn maze_beacons(state: &mut AppState) {
    use crate::maze_walk::{MazeMode, MazeWalk};
    let game = &mut state.maze_walk;
    *game = MazeWalk::new_with_mode(0, MazeMode::Fog);
    for _ in 0..7 {
        let Some(direction) = game.hint_direction() else {
            break;
        };
        game.step(direction);
    }
    game.moves = 11;
}

fn nim_tactics(state: &mut AppState) {
    use crate::nim::{Nim, NimRule};
    let game = &mut state.nim;
    *game = Nim::new(0x4E1D);
    game.rule = NimRule::Misere;
    for heaps in [[5, 5, 5], [5, 4, 3], [4, 4, 3], [4, 3, 2], [0, 1, 2]] {
        game.heaps = heaps;
        let Some((heap, amount)) = game.hint_move() else {
            continue;
        };
        if game.move_is_winning(heap, amount) == Some(true) {
            game.selected_heap = Some(heap);
            break;
        }
    }
    game.moves = 3;
    game.last_player_take = 1;
    game.last_ai_take = 2;
}

fn word_ladder_routes(state: &mut AppState) {
    use crate::word_ladder::{LadderMode, WordLadder};
    let game = &mut state.word_ladder;
    *game = WordLadder::new_with_mode(2, LadderMode::Scenic);
    for word in ["MIGHT", "RIGHT"] {
        for letter in word.bytes() {
            game.tap_letter(letter - b'A');
        }
        game.submit();
    }
    for letter in "NIG".bytes() {
        game.tap_letter(letter - b'A');
    }
}

fn pyramid_chains(state: &mut AppState) {
    use crate::pyramid::{Pyramid, PyramidDraw};
    let game = &mut state.pyramid;
    *game = Pyramid::new(0x51A0_2200);
    game.draw_rule = PyramidDraw::Three;
    if let Some(card) = game.pyramid[21].as_mut() {
        card.rank = 5;
    }
    if let Some(card) = game.pyramid[22].as_mut() {
        card.rank = 8;
    }
    game.selected = Some(21);
    game.points = 140;
    game.combo = 3;
    game.best_combo = 4;
    game.moves = 12;
}
