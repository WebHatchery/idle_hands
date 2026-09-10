use crate::state::AppState;

pub fn snake(state: &AppState) -> String {
    let game = &state.games.snake;
    match game.status {
        crate::snake::SnakeStatus::Won => {
            return "The coil is complete — tap NEW BOARD to play again.".into()
        }
        crate::snake::SnakeStatus::Lost => {
            return "The coil is resting — tap NEW BOARD to begin again.".into()
        }
        crate::snake::SnakeStatus::Playing => {}
    }
    game.hint_direction().map_or_else(
        || "No safe turn remains — tap NEW BOARD to begin again.".into(),
        |direction| {
            format!(
                "Try {} toward the {}.",
                snake_direction_label(direction),
                if game.food_kind == crate::snake::FoodKind::Gold {
                    "gold food"
                } else {
                    "food"
                }
            )
        },
    )
}

pub fn breakout(state: &AppState) -> String {
    let game = &state.games.breakout;
    match game.status {
        crate::breakout::BreakoutStatus::Won => {
            return "All three walls are clear — tap NEW BOARD to play again.".into()
        }
        crate::breakout::BreakoutStatus::Lost => {
            return "No balls remain — tap NEW BOARD to begin again.".into()
        }
        crate::breakout::BreakoutStatus::Playing => {}
    }
    if game.serve_ready {
        return format!("Wall {} is ready — tap LAUNCH.", game.level);
    }
    game.hint_move().map_or_else(
        || "No paddle move is available — tap NEW BOARD to begin again.".into(),
        |movement| {
            format!(
                "Move the paddle {} to track the ball.",
                movement_label(movement)
            )
        },
    )
}

pub fn higher_lower(state: &AppState) -> String {
    let game = &state.games.higher_lower;
    match game.status {
        crate::higher_lower::HigherLowerStatus::Won => {
            return "The run is yours — tap NEW ROUND to play again.".into()
        }
        crate::higher_lower::HigherLowerStatus::Lost => {
            return "The next card slipped away — tap NEW ROUND to begin again.".into()
        }
        crate::higher_lower::HigherLowerStatus::Playing => {}
    }
    game.hint_guess().map_or_else(
        || "No odds hint is available — tap NEW ROUND to begin again.".into(),
        |guess| {
            format!(
                "Best odds: {} at {}%; card hidden.{}",
                guess_label(guess),
                game.chance(guess),
                if game.score >= 2 {
                    " CASH OUT banks the current pot."
                } else {
                    ""
                }
            )
        },
    )
}

pub fn blackjack(state: &AppState) -> String {
    let game = &state.games.blackjack;
    match game.status {
        crate::blackjack::BlackjackStatus::Won => {
            return "You win — tap NEW ROUND to deal again.".into()
        }
        crate::blackjack::BlackjackStatus::Lost => {
            return "Dealer wins — tap NEW ROUND to deal again.".into()
        }
        crate::blackjack::BlackjackStatus::Push => {
            return "Push — tap NEW ROUND to deal again.".into()
        }
        crate::blackjack::BlackjackStatus::Playing => {}
    }
    game.hint_action().map_or_else(
        || "No strategy hint is available — tap NEW ROUND.".into(),
        |action| format!("Basic odds suggest {}.", blackjack_hint_label(action)),
    )
}

pub fn dungeon_sweeper(state: &AppState) -> String {
    let game = &state.games.dungeon_sweeper;
    match game.status {
        crate::dungeon_sweeper::DungeonStatus::Won => {
            return "The exit is found — tap NEW DUNGEON to play again.".into()
        }
        crate::dungeon_sweeper::DungeonStatus::Lost => {
            return "A trap closed the path — tap NEW DUNGEON to begin again.".into()
        }
        crate::dungeon_sweeper::DungeonStatus::Ready
        | crate::dungeon_sweeper::DungeonStatus::Playing => {}
    }
    game.hint_cell().map_or_else(
        || "No safe room remains — tap NEW DUNGEON to begin again.".into(),
        |index| {
            if index == game.exit {
                "All relics are held — tap EXIT to leave the dungeon.".into()
            } else if game.relics.contains(&index) && !game.collected_relics.contains(&index) {
                format!("Room {} holds a safe relic.", index + 1)
            } else {
                format!("Room {} is safe to reveal.", index + 1)
            }
        },
    )
}

pub fn potion_2048(state: &AppState) -> String {
    let game = &state.games.potion_2048;
    if game.won() {
        return "The master potion is brewed — tap NEW BREW to play again.".into();
    }
    game.hint_direction().map_or_else(
        || {
            format!(
                "No merge remains — tap NEW BREW to reach {} again.",
                game.target()
            )
        },
        |direction| {
            let catalyst = if game.cells.contains(&1) {
                " CATALYST C reacts with any potion."
            } else {
                ""
            };
            format!(
                "Best move: {}. Chain {}/{} toward the next CATALYST.{}",
                potion_direction_label(direction),
                game.combo,
                game.difficulty.catalyst_chain(),
                catalyst
            )
        },
    )
}

pub fn tiny_tower_defence(state: &AppState) -> String {
    let game = &state.games.tiny_tower_defence;
    match game.phase {
        crate::tiny_tower_defence::TowerPhase::Won => {
            return "The tower holds — tap NEW TOWER to play again.".into()
        }
        crate::tiny_tower_defence::TowerPhase::Lost => {
            return "The gate fell — tap NEW TOWER to begin again.".into()
        }
        crate::tiny_tower_defence::TowerPhase::Build
        | crate::tiny_tower_defence::TowerPhase::Wave => {}
    }
    game.hint_action().map_or_else(
        || "No tower action is available — tap NEW TOWER to begin again.".into(),
        |hint| match hint {
            crate::tiny_tower_defence::TowerHint::Build(index, kind) => {
                format!(
                    "Select {}, then build or upgrade room {}.",
                    kind.label(),
                    index + 1
                )
            }
            crate::tiny_tower_defence::TowerHint::WaveControl => {
                if game.paused {
                    "The wave is paused — tap RESUME when you are ready.".into()
                } else {
                    "The wave is moving — tap PAUSE when you need a break.".into()
                }
            }
        },
    )
}

pub fn one_room_roguelike(state: &AppState) -> String {
    let game = &state.games.one_room_roguelike;
    match game.phase {
        crate::one_room_roguelike::RoomPhase::Won => {
            return "All five rooms are clear — tap NEW RUN or choose a hero to play again.".into()
        }
        crate::one_room_roguelike::RoomPhase::Lost => {
            return "The run claims you — tap NEW RUN to begin again.".into()
        }
        crate::one_room_roguelike::RoomPhase::Stairs => {}
        crate::one_room_roguelike::RoomPhase::Exploring => {}
    }
    game.hint_action().map_or_else(
        || "No room action is available — tap NEW RUN to begin again.".into(),
        |hint| match hint {
            crate::one_room_roguelike::RogueHint::Strike => format!(
                "Tap STRIKE. {} deals {} damage.",
                game.hero_class.label(),
                game.attack_damage()
            ),
            crate::one_room_roguelike::RogueHint::Potion => "DRINK POTION to recover.".into(),
            crate::one_room_roguelike::RogueHint::Move(direction) => {
                let target = if game.phase == crate::one_room_roguelike::RoomPhase::Stairs {
                    "STAIRS"
                } else {
                    "EXIT"
                };
                format!(
                    "Move {} toward {}.",
                    rogue_direction_label(direction),
                    target
                )
            }
        },
    )
}

pub fn daily_dungeon(state: &AppState) -> String {
    let game = &state.games.daily_dungeon;
    match game.phase {
        crate::daily_dungeon::DailyPhase::Won => {
            return "The daily route is clear — tap REPLAY DAY to play again.".into()
        }
        crate::daily_dungeon::DailyPhase::Lost => {
            return "The traps closed in — tap REPLAY DAY to begin again.".into()
        }
        crate::daily_dungeon::DailyPhase::Exploring => {}
    }
    game.hint_action().map_or_else(
        || "No route hint is available — tap REPLAY DAY to begin again.".into(),
        |hint| match hint {
            crate::daily_dungeon::DailyHint::Scout => {
                "Tap SCOUT to reveal the neighboring rooms without entering them.".into()
            }
            crate::daily_dungeon::DailyHint::Move(direction) => {
                let target = if game.hearts < 3
                    && game.tiles.iter().enumerate().any(|(index, tile)| {
                        game.revealed[index]
                            && matches!(tile, crate::daily_dungeon::DailyTile::Spring)
                    }) {
                    "the revealed spring"
                } else if game.runes_found < crate::daily_dungeon::DailyDungeon::rune_total() {
                    "a rune"
                } else {
                    "EXIT"
                };
                format!(
                    "Move {} toward {} while avoiding revealed traps.",
                    daily_direction_label(direction),
                    target
                )
            }
        },
    )
}

pub fn dots_boxes(state: &AppState) -> String {
    let game = &state.games.dots_boxes;
    match game.phase {
        crate::dots_boxes::DotsPhase::Won => {
            return "The red boxes hold — tap NEW BOARD to play again.".into()
        }
        crate::dots_boxes::DotsPhase::Lost => {
            return "The blue boxes hold — tap NEW BOARD to play again.".into()
        }
        crate::dots_boxes::DotsPhase::Playing => {}
    }
    game.hint_edge().map_or_else(
        || "No edge remains — tap NEW BOARD to begin again.".into(),
        |edge| {
            let purpose = if game.would_complete(edge) {
                "Close a box with"
            } else if game.edge_risk(edge) == 0 {
                "Keep the board safe with"
            } else {
                "No safe line remains; sacrifice"
            };
            match edge {
                crate::dots_boxes::Edge::Horizontal(index) => {
                    format!("{} horizontal edge {}.", purpose, index + 1)
                }
                crate::dots_boxes::Edge::Vertical(index) => {
                    format!("{} vertical edge {}.", purpose, index + 1)
                }
            }
        },
    )
}

pub fn sokoban(state: &AppState) -> String {
    let game = &state.games.sokoban;
    if game.won() {
        return format!(
            "{} clear in {} moves — tap NEXT ROOM.",
            game.clear_rank(),
            game.moves
        );
    }
    if game.phase == crate::sokoban::SokobanPhase::Stuck {
        return "A crate is cornered — tap UNDO or RESTART.".into();
    }
    game.hint_direction().map_or_else(
        || "No route remains — tap UNDO or RESTART.".into(),
        |direction| {
            format!(
                "Move {} to place the next crate.",
                daily_direction_label(direction)
            )
        },
    )
}

pub fn mancala(state: &AppState) -> String {
    let game = &state.games.mancala;
    match game.phase {
        crate::mancala::MancalaPhase::Won => {
            return "Your store leads — tap NEW BOARD to play again.".into()
        }
        crate::mancala::MancalaPhase::Lost => {
            return "The cabinet leads — tap NEW BOARD to begin again.".into()
        }
        crate::mancala::MancalaPhase::Playing => {}
    }
    game.hint_pit().map_or_else(
        || "No stones remain — tap NEW BOARD to begin again.".into(),
        |pit| {
            let preview = game.move_preview(pit);
            if preview.is_some_and(|move_| move_.extra_turn) {
                format!("Sow pit {} to land in your store and play again.", pit + 1)
            } else if let Some(captured) = preview.map(|move_| move_.captured).filter(|&n| n > 0) {
                format!("Sow pit {} to capture {} stones.", pit + 1, captured)
            } else {
                format!("Sow pit {} for the strongest next turn.", pit + 1)
            }
        },
    )
}

pub fn hanoi(state: &AppState) -> String {
    let game = &state.games.hanoi;
    if game.won() {
        return format!(
            "{} tower in {} moves — choose 3, 5, or 7 DISKS, or tap RESTART.",
            game.clear_rank(),
            game.moves
        );
    }
    game.hint_move().map_or_else(
        || "No legal route remains — tap RESTART to begin again.".into(),
        |(source, destination)| {
            format!(
                "Move a disk from peg {} to peg {}.",
                source + 1,
                destination + 1
            )
        },
    )
}

pub fn number_match(state: &AppState) -> String {
    let game = &state.games.number_match;
    match game.phase {
        crate::number_match::NumberMatchPhase::Won => {
            return "Every number has found its pair — tap NEW BOARD to play again.".into()
        }
        crate::number_match::NumberMatchPhase::Stuck => {
            return "No links remain — tap UNDO or REMIX to recover.".into()
        }
        crate::number_match::NumberMatchPhase::Playing => {}
    }
    game.hint_pair().map_or_else(
        || "No adjacent pair remains — tap NEW BOARD to begin again.".into(),
        |(first, second)| {
            format!(
                "Pair cells {} and {}.",
                cell_label(first),
                cell_label(second)
            )
        },
    )
}

pub fn flood_it(state: &AppState) -> String {
    let game = &state.games.flood_it;
    match game.phase {
        crate::flood_it::FloodPhase::Won => {
            return "The field is one color — tap NEW FIELD to play again.".into()
        }
        crate::flood_it::FloodPhase::Lost => {
            return "The field held out — tap NEW FIELD to begin again.".into()
        }
        crate::flood_it::FloodPhase::Playing => {}
    }
    if game.surges > 0 {
        return "Tap SURGE to take the strongest forecast without spending a move.".into();
    }
    game.hint_color().map_or_else(
        || "No color change remains — tap NEW FIELD to begin again.".into(),
        |color| {
            format!(
                "Choose {} to gain {} cells.",
                flood_color_label(color),
                game.preview_gain(color)
            )
        },
    )
}

pub fn color_sort(state: &AppState) -> String {
    let game = &state.games.color_sort;
    if game.won() {
        return "The color tubes are complete — tap NEW BOARD to play again.".into();
    }
    game.hint_move().map_or_else(
        || "No legal tube move remains — tap NEW BOARD to begin again.".into(),
        |(source, destination)| {
            let preview = game.pour_preview(source, destination).unwrap();
            format!(
                "Pour {} layer{} from tube {} to tube {}{}.",
                preview.count,
                if preview.count == 1 { "" } else { "s" },
                source + 1,
                destination + 1,
                if preview.completes_tube {
                    " to seal it"
                } else {
                    ""
                }
            )
        },
    )
}

pub fn battleship(state: &AppState) -> String {
    let game = &state.games.battleship;
    if game.won() {
        return "The fleet is found — tap NEW FLEET to play again.".into();
    }
    if game.sonar_armed {
        return "SONAR armed — tap a water cell to sweep its 3 × 3 area.".into();
    }
    game.hint_cell().map_or_else(
        || "Every water cell is searched — tap NEW FLEET to begin again.".into(),
        |cell| {
            if game.is_scanned(cell) && game.ships[cell] != 0 {
                format!(
                    "SONAR contact confirmed at {} — fire there.",
                    battleship_cell_label(cell)
                )
            } else if game.sonar_charges > 0 && game.hits() == 0 {
                format!(
                    "Sweep near {} with SONAR, or fire there directly.",
                    battleship_cell_label(cell)
                )
            } else {
                format!("Fire at cell {}.", battleship_cell_label(cell))
            }
        },
    )
}

pub fn word_grid(state: &AppState) -> String {
    let game = &state.games.word_grid;
    match game.phase {
        crate::word_grid::WordGridPhase::Won => {
            return "The word is found — tap NEW WORD to play again.".into()
        }
        crate::word_grid::WordGridPhase::Lost => {
            return "The word is revealed — tap NEW WORD to begin again.".into()
        }
        crate::word_grid::WordGridPhase::Playing => {}
    }
    game.hint_word().map_or_else(
        || "No probe remains — tap NEW WORD to begin again.".into(),
        |word| {
            format!(
                "{} candidates remain — try {} to test the strongest letter spread.",
                game.remaining_words().len(),
                word
            )
        },
    )
}

pub fn pipe_loop(state: &AppState) -> String {
    let game = &state.games.pipe_loop;
    if game.won() {
        return "The loop is joined — tap NEW LOOP to play again.".into();
    }
    game.hint_rotation().map_or_else(
        || "No rotation remains — tap NEW LOOP to begin again.".into(),
        |(index, count)| {
            format!(
                "Rotate tile {} {} ({} powered, {} leaks remain).",
                index + 1,
                if count == 1 { "once" } else { "multiple times" },
                game.connected_count(),
                game.leak_count()
            )
        },
    )
}

pub fn maze_walk(state: &AppState) -> String {
    let game = &state.games.maze_walk;
    if game.won() {
        return "The exit is found — tap NEW MAZE to play again.".into();
    }
    game.hint_direction().map_or_else(
        || "No route remains — tap NEW MAZE to begin again.".into(),
        |direction| {
            let objective = if game.collected.len() < game.beacons.len() {
                "the next beacon"
            } else {
                "the unlocked exit"
            };
            format!(
                "Walk {} toward {} ({} steps away).",
                direction_label(direction),
                objective,
                game.distance_to_objective()
            )
        },
    )
}

pub fn match_three(state: &AppState) -> String {
    let game = &state.games.match_three;
    if game.won() {
        return "The color field is clear — tap NEW BOARD to play again.".into();
    }
    if game.phase == crate::match_three::MatchThreePhase::Lost {
        return "No moves remain — tap NEW BOARD or choose a difficulty to try again.".into();
    }
    game.hint_swap().map_or_else(
        || "No matching swap remains — tap NEW BOARD to begin again.".into(),
        |(first, second)| format!("Swap tiles {} and {}.", first + 1, second + 1),
    )
}

fn direction_label(direction: crate::domain::Direction) -> &'static str {
    match direction {
        crate::domain::Direction::Up => "UP",
        crate::domain::Direction::Right => "RIGHT",
        crate::domain::Direction::Down => "DOWN",
        crate::domain::Direction::Left => "LEFT",
    }
}

fn battleship_cell_label(index: usize) -> String {
    format!(
        "{}{}",
        (b'A' + (index % crate::battleship::SIDE) as u8) as char,
        index / crate::battleship::SIDE + 1
    )
}

fn flood_color_label(color: u8) -> &'static str {
    [
        "RED", "AMBER", "GREEN", "BLUE", "VIOLET", "PINK", "MINT", "ORANGE",
    ][color as usize % 8]
}

fn cell_label(index: usize) -> String {
    format!(
        "{}{}",
        (b'A' + (index % crate::number_match::SIDE) as u8) as char,
        index / crate::number_match::SIDE + 1
    )
}

fn daily_direction_label(direction: crate::domain::Direction) -> &'static str {
    match direction {
        crate::domain::Direction::Up => "UP",
        crate::domain::Direction::Left => "LEFT",
        crate::domain::Direction::Down => "DOWN",
        crate::domain::Direction::Right => "RIGHT",
    }
}

fn rogue_direction_label(direction: crate::domain::Direction) -> &'static str {
    daily_direction_label(direction)
}

fn potion_direction_label(direction: crate::domain::Direction) -> &'static str {
    daily_direction_label(direction)
}

fn blackjack_hint_label(action: crate::blackjack::BlackjackHint) -> &'static str {
    match action {
        crate::blackjack::BlackjackHint::Hit => "HIT",
        crate::blackjack::BlackjackHint::Stand => "STAND",
    }
}

fn guess_label(guess: crate::higher_lower::Guess) -> &'static str {
    match guess {
        crate::higher_lower::Guess::Higher => "HIGHER",
        crate::higher_lower::Guess::Lower => "LOWER",
    }
}

fn movement_label(movement: crate::breakout::PaddleMove) -> &'static str {
    match movement {
        crate::breakout::PaddleMove::Left => "LEFT",
        crate::breakout::PaddleMove::Stay => "STAY",
        crate::breakout::PaddleMove::Right => "RIGHT",
    }
}

fn snake_direction_label(direction: crate::snake::SnakeDirection) -> &'static str {
    match direction {
        crate::snake::SnakeDirection::Up => "UP",
        crate::snake::SnakeDirection::Right => "RIGHT",
        crate::snake::SnakeDirection::Down => "DOWN",
        crate::snake::SnakeDirection::Left => "LEFT",
    }
}
