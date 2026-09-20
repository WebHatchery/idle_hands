use super::super::Game;
use crate::{
    card_hints,
    state::{GameId, Screen},
};
use macroquad_toolkit::notifications::NotificationManager;

impl Game {
    pub fn begin_capture_scene(&mut self, scene: &str) {
        self.state = crate::state::AppState::new(&self.data);
        self.capture_solitaire_peek = false;
        self.notifications = NotificationManager::new();
        let scene = scene
            .strip_prefix("portrait_")
            .or_else(|| scene.strip_prefix("landscape_"))
            .unwrap_or(scene);
        let tutorial = scene.starts_with("tutorial_");
        let scene = scene.strip_prefix("tutorial_").unwrap_or(scene);
        self.state.screen = crate::capture_registry::screen_for_scene(scene)
            .unwrap_or_else(|| super::super::game_capture_scenes::special_screen_for_scene(scene));
        if scene == "settings_reset" {
            self.state.confirm_reset = true;
        }
        if scene == "cabinet_scrolled" {
            self.state.cabinet_filter = 9;
            self.state.cabinet_scroll = 6;
        }
        if matches!(scene, "records_scrolled" | "rules_scrolled") {
            self.state.library_scroll = 12;
        }
        if matches!(scene, "statistics" | "statistics_accessible") {
            self.state.records.ensure_time_slots();
            for (game, seconds) in [
                (GameId::Solitaire, 142),
                (GameId::Game2048, 86),
                (GameId::Snake, 64),
                (GameId::Sudoku, 31),
            ] {
                let index = game.index();
                self.state.records.elapsed_seconds[index] = seconds;
                self.state.records.best_time_seconds[index] = Some(seconds.saturating_sub(18));
                self.state.favorites[index] = matches!(game, GameId::Solitaire | GameId::Snake);
            }
            self.state.records.best_2048 = 2048;
            self.state.records.record_time(GameId::Game2048.index());
            self.state.records.solitaire_best_moves = Some(42);
            self.state.records.snake_best_score = Some(120);
            self.state.recent_games = vec![GameId::Solitaire, GameId::Snake, GameId::Game2048];
            for day in 1..=6 {
                self.state.records.record_daily_result(
                    day,
                    40 + day as u32 * 7,
                    day.is_multiple_of(2),
                );
            }
        }
        if matches!(scene, "tutorials" | "tutorials_accessible") {
            for index in [0, 2, 5, 8, 13, 21, 34] {
                self.state.tutorial_seen[index] = true;
            }
            self.state.tutorial_filter = scene == "tutorials_accessible";
        }
        if matches!(scene, "finder" | "finder_filtered" | "finder_scrolled") {
            self.state.cabinet_filter = if scene == "finder_filtered" { 1 } else { 0 };
            self.state.library_scroll = if scene == "finder_scrolled" { 12 } else { 0 };
            self.state.recent_games = vec![GameId::Solitaire, GameId::Snake, GameId::WordLadder];
        }
        if matches!(scene, "profile" | "profile_accessible") {
            self.state.profile_name = crate::profile_data::name(&self.state.content, 2).to_owned();
        }
        if matches!(scene, "drawer_info" | "drawer_info_accessible") {
            self.state.favorites[GameId::Solitaire.index()] = true;
            self.state.records.solitaire_best_moves = Some(42);
        }
        if matches!(
            scene,
            "daily_archive" | "daily_archive_accessible" | "daily_archive_scrolled"
        ) {
            for day in 1..=12 {
                self.state.records.record_daily_result(
                    day,
                    40 + day as u32 * 5,
                    day.is_multiple_of(3),
                );
            }
            self.state.daily_archive_view = true;
            if scene == "daily_archive_scrolled" {
                self.state.daily_archive_scroll = 6;
            }
        }
        if scene == "2048_confirm" {
            self.state.confirm_restart = true;
            self.state.pending_restart = Some(crate::ui::UiAction::Restart);
        }
        if matches!(
            scene,
            "2048_setup"
                | "2048_setup_accessible"
                | "sudoku_setup"
                | "sudoku_setup_accessible"
                | "nonogram_setup"
                | "nonogram_setup_accessible"
                | "solitaire_setup"
                | "solitaire_setup_accessible"
        ) {
            self.state.game_setup_open = true;
        }
        if matches!(scene, "sudoku_focus" | "sudoku_focus_accessible") {
            self.state.games.sudoku.selected = Some(40);
            self.state.sudoku_focus_open = true;
        }
        if matches!(scene, "nonogram_focus" | "nonogram_focus_accessible") {
            let center = self.state.games.nonogram.size / 2;
            self.state.games.nonogram.selected =
                Some(center * self.state.games.nonogram.size + center);
            self.state.nonogram_focus_open = true;
        }
        if scene == "nonogram_large" {
            self.state.games.nonogram =
                crate::nonogram::Nonogram::new(crate::nonogram::NonogramPreset::Large);
            self.state.games.nonogram_zoomed = true;
            self.state.games.nonogram_focus = (6, 6);
        }
        if scene == "solitaire_selected" {
            self.state.games.solitaire.select_tableau(0, 0);
        }
        if scene == "breakout_wall_two" {
            let game = &mut self.state.games.breakout;
            game.level = 2;
            game.lives = 2;
            game.score = 64;
            game.paused = true;
            game.serve_ready = true;
            game.bricks = vec![false; 64];
            game.brick_health = vec![0; 64];
            for index in 0_usize..64 {
                let row = index / 16;
                let column = index % 16;
                if !(row + column).is_multiple_of(5) {
                    let health = if row == 0 || column.is_multiple_of(4) {
                        2
                    } else {
                        1
                    };
                    game.bricks[index] = true;
                    game.brick_health[index] = health;
                }
            }
        }
        if scene == "snake_garden" {
            use crate::snake::{FoodKind, Snake, SnakeMode};
            let game = &mut self.state.games.snake;
            *game = Snake::new_with_mode(0x5A4D_0001, SnakeMode::Garden);
            game.body = vec![104, 103, 102, 101, 100, 99, 98];
            game.score = 9;
            game.food = 72;
            game.food_kind = FoodKind::Gold;
            game.obstacles.retain(|cell| *cell != game.food);
            game.paused = true;
        }
        if scene == "tiny_tower_roles" {
            use crate::tiny_tower_defence::{Enemy, EnemyKind, TowerKind, TowerPhase};
            let game = &mut self.state.games.tiny_tower_defence;
            game.wave = 5;
            game.gold = 4;
            game.score = 180;
            game.phase = TowerPhase::Wave;
            game.paused = true;
            game.selected_kind = TowerKind::Burst;
            game.towers = vec![0; 35];
            game.tower_kinds = vec![TowerKind::Bolt; 35];
            for (index, kind) in [
                (9, TowerKind::Bolt),
                (17, TowerKind::Frost),
                (25, TowerKind::Burst),
            ] {
                game.towers[index] = 2;
                game.tower_kinds[index] = kind;
            }
            game.enemies = vec![
                Enemy {
                    row: 0,
                    column: 2,
                    health: 2,
                    kind: EnemyKind::Grunt,
                    slow_ticks: 0,
                },
                Enemy {
                    row: 2,
                    column: 3,
                    health: 2,
                    kind: EnemyKind::Swift,
                    slow_ticks: 1,
                },
                Enemy {
                    row: 4,
                    column: 4,
                    health: 5,
                    kind: EnemyKind::Armored,
                    slow_ticks: 0,
                },
            ];
        }
        if matches!(
            scene,
            "tiny_tower_inspection"
                | "tiny_tower_inspection_accessible"
                | "tiny_tower_insufficient"
                | "tiny_tower_maxed"
        ) {
            let game = &mut self.state.games.tiny_tower_defence;
            game.selected_kind = crate::tiny_tower_defence::TowerKind::Burst;
            self.state.tower_inspection = Some(8);
            if scene == "tiny_tower_insufficient" {
                game.gold = 2;
            }
            if scene == "tiny_tower_maxed" {
                game.towers[8] = 3;
                game.tower_kinds[8] = crate::tiny_tower_defence::TowerKind::Burst;
            }
        }
        if scene == "tiny_tower_wave3" {
            let game = &mut self.state.games.tiny_tower_defence;
            game.wave = 3;
            game.gold = 7;
            game.selected_kind = crate::tiny_tower_defence::TowerKind::Frost;
        }
        if scene == "tiny_tower_loss" {
            let game = &mut self.state.games.tiny_tower_defence;
            game.wave = 5;
            game.gold = 1;
            game.lives = 0;
            game.score = 210;
            game.phase = crate::tiny_tower_defence::TowerPhase::Lost;
            game.paused = false;
        }
        if scene == "rogue_roles" {
            use crate::one_room_roguelike::{EnemyKind, HeroClass, OneRoomRoguelike, RoomEnemy};
            let game = &mut self.state.games.one_room_roguelike;
            *game = OneRoomRoguelike::new_with_class(0x0E_700005, HeroClass::Warden);
            game.room = 4;
            game.health = 9;
            game.potions = 1;
            game.score = 224;
            game.turns = 19;
            game.player = 24;
            game.treasure = 38;
            game.enemies = vec![
                RoomEnemy {
                    position: 9,
                    health: 4,
                    damage: 2,
                    kind: EnemyKind::Guard,
                },
                RoomEnemy {
                    position: 17,
                    health: 4,
                    damage: 2,
                    kind: EnemyKind::Stalker,
                },
                RoomEnemy {
                    position: 31,
                    health: 8,
                    damage: 3,
                    kind: EnemyKind::Brute,
                },
            ];
        }
        if scene == "daily_scouting" {
            use crate::daily_dungeon::{DailyDungeon, DailyRule, DailyTile};
            let game = &mut self.state.games.daily_dungeon;
            *game = DailyDungeon::new(1);
            game.rule = DailyRule::Forager;
            game.player = 14;
            game.hearts = 2;
            game.scouts = 1;
            game.runes_found = 1;
            game.moves = 8;
            game.score = 37;
            game.tiles = vec![DailyTile::Floor; 36];
            game.tiles[8] = DailyTile::Rune;
            game.tiles[15] = DailyTile::Trap;
            game.tiles[20] = DailyTile::Spring;
            game.tiles[35] = DailyTile::Exit;
            game.revealed = vec![false; 36];
            for index in [0, 1, 6, 7, 8, 14, 15, 20] {
                game.revealed[index] = true;
            }
        }
        if scene == "dungeon_relics" {
            use crate::dungeon_sweeper::{
                DungeonCell, DungeonDifficulty, DungeonStatus, DungeonSweeper,
            };
            let game = &mut self.state.games.dungeon_sweeper;
            *game = DungeonSweeper::new_with_difficulty(0xD0A6_0004, DungeonDifficulty::Explorer);
            game.first_reveal = true;
            game.status = DungeonStatus::Playing;
            game.hearts = 2;
            game.moves = 11;
            game.relics = vec![18, 45];
            game.collected_relics = vec![18];
            game.required_relics = 2;
            game.cells = vec![DungeonCell::Hidden; 64];
            for (index, clue) in [
                (0, 0),
                (1, 1),
                (8, 0),
                (9, 1),
                (10, 2),
                (16, 1),
                (17, 2),
                (18, 2),
                (24, 1),
                (25, 2),
                (63, 1),
            ] {
                game.cells[index] = DungeonCell::Revealed(clue);
            }
            game.cells[27] = DungeonCell::Revealed(9);
            game.cells[35] = DungeonCell::FlaggedTrap;
            game.cells[45] = DungeonCell::Hidden;
        }
        if scene == "fling_fury_won" {
            let game = &mut self.state.games.fling_fury;
            game.status = crate::fling_fury::FlingStatus::Won;
            game.moves = 3;
            game.shot = None;
            for target in &mut game.targets {
                target.alive = false;
                target.falling = false;
            }
        }
        if scene == "fling_fury_lost" {
            let game = &mut self.state.games.fling_fury;
            game.status = crate::fling_fury::FlingStatus::Lost;
            game.moves = 5;
            game.shots_remaining = 0;
            game.shot = None;
            for target in &mut game.targets {
                target.alive = true;
                target.falling = false;
            }
        }
        super::super::game_capture_depth::apply(&mut self.state, scene);
        if scene == "solitaire_peek" {
            self.capture_solitaire_peek = true;
            self.state.games.solitaire.tableau[0] = (1..=13)
                .map(|rank| crate::cards::Card {
                    rank,
                    suit: 3,
                    face_up: true,
                })
                .collect();
            self.state.games.solitaire_peek = Some(crate::solitaire::CardSource::Tableau(0, 6));
        }
        if scene == "cabinet_favorites" {
            for index in [0, GameId::Spider.index(), GameId::Nim.index()] {
                self.state.favorites[index] = true;
            }
        }
        if scene == "cabinet_recent" {
            self.state.recent_games = vec![
                GameId::WordLadder,
                GameId::Spider,
                GameId::Minesweeper,
                GameId::Solitaire,
            ];
            self.state.cabinet_sort = crate::cabinet_status::CabinetSort::Recent.index();
        }
        if scene == "cabinet_sorted" {
            self.state.records.best_2048 = 2048;
            self.state.records.record_time(GameId::Game2048.index());
            self.state.records.solitaire_best_moves = Some(42);
            self.state.cabinet_filter = 9;
            self.state.cabinet_sort = crate::cabinet_status::CabinetSort::Progress.index();
        }
        if matches!(scene, "cabinet_open" | "cabinet_done" | "cabinet_empty") {
            self.state.records.best_2048 = 2048;
            self.state.records.record_time(GameId::Game2048.index());
            self.state.records.solitaire_best_moves = Some(42);
            self.state.cabinet_filter = if scene == "cabinet_empty" {
                self.state.records = crate::state::CollectionRecords::default();
                2
            } else if scene == "cabinet_done" {
                2
            } else {
                1
            };
        }
        if scene == "match_three_confirm" {
            self.state.confirm_restart = true;
            self.state.pending_restart = Some(crate::ui::UiAction::MatchThreeNew);
        }
        if scene == "match_three_specials" {
            use crate::match_three::MatchThreeSpecial;
            let game = &mut self.state.games.match_three;
            game.specials = vec![MatchThreeSpecial::None; game.cells.len()];
            for (index, special) in [
                (16, MatchThreeSpecial::Row),
                (24, MatchThreeSpecial::Burst),
                (32, MatchThreeSpecial::Column),
            ] {
                game.specials[index] = special;
            }
            game.score = 90;
            game.moves = 6;
            game.last_cascade = 3;
            game.best_cascade = 3;
        }
        if scene == "match_three_result_won" {
            let game = &mut self.state.games.match_three;
            game.phase = crate::match_three::MatchThreePhase::Won;
            game.score = game.target_score();
            game.moves = 11;
            game.last_cascade = 4;
            game.best_cascade = 5;
        }
        if scene == "match_three_result_lost" {
            let game = &mut self.state.games.match_three;
            game.phase = crate::match_three::MatchThreePhase::Lost;
            game.score = game.target_score().saturating_sub(18);
            game.moves = game.move_limit();
            game.last_cascade = 1;
            game.best_cascade = 3;
        }
        if scene == "match_three_result_won" || scene == "match_three_result_lost" {
            self.state.tutorial = None;
            self.state.tutorial_seen[GameId::MatchThree.index()] = true;
        }
        if scene == "minesweeper_confirm" {
            self.state.confirm_restart = true;
            self.state.pending_restart = Some(crate::ui::UiAction::MineRestart);
        }
        if scene == "word_ladder_best" {
            self.state.records.word_ladder_best_moves = Some(5);
        }
        if scene == "word_ladder_progress" {
            self.state.games.word_ladder.reset(0);
            for letter in b"PLATE" {
                self.state.games.word_ladder.tap_letter(letter - b'A');
            }
            self.state.games.word_ladder.submit();
        }
        if scene == "word_ladder_confirm" {
            self.state.confirm_restart = true;
            self.state.pending_restart = Some(crate::ui::UiAction::WordLadderNew);
        }
        if scene == "records_word_ladder" {
            self.state.records.word_ladder_best_moves = Some(5);
        }
        super::super::game_capture_records::apply(&mut self.state, scene);
        super::super::game_capture_rules::apply(&mut self.state, scene);
        if matches!(
            scene,
            "achievements"
                | "achievements_accessible"
                | "achievements_earned"
                | "achievements_locked"
        ) {
            self.state.records.best_2048 = 2048;
            self.state.records.record_time(GameId::Game2048.index());
            self.state.records.solitaire_best_moves = Some(42);
            self.state.records.word_ladder_best_moves = Some(5);
            let _ = crate::progression::sync_with_content(
                &mut self.state.achievements,
                &mut self.state.stamps,
                &self.state.records,
                &self.state.content,
            );
            self.state.achievements_view = true;
            self.state.achievement_filter = if scene == "achievements_earned" {
                1
            } else if scene == "achievements_locked" {
                2
            } else {
                0
            };
        }
        if matches!(scene, "favorites_browse" | "favorites_info") {
            for index in [
                0,
                GameId::Spider.index(),
                GameId::Nim.index(),
                GameId::WordLadder.index(),
            ] {
                self.state.favorites[index] = true;
            }
            self.state.records.best_2048 = 2048;
            self.state.records.record_time(GameId::Game2048.index());
            self.state.records.solitaire_best_moves = Some(42);
            self.state.favorites_view = true;
        }
        if scene == "favorites_all" {
            self.state.favorites.fill(true);
            self.state.favorites_view = true;
        }
        if matches!(scene, "recent_browse" | "recent_info") {
            self.state.recent_games = vec![
                GameId::WordLadder,
                GameId::Spider,
                GameId::Minesweeper,
                GameId::Solitaire,
                GameId::Game2048,
            ];
            self.state.records.best_2048 = 2048;
            self.state.records.record_time(GameId::Game2048.index());
            self.state.records.solitaire_best_moves = Some(42);
            self.state.recent_view = true;
        }
        if scene == "freecell_selected" {
            self.state.games.freecell.select_cascade(0, 0);
        }
        if scene.ends_with("_accessible") {
            self.state.high_contrast = true;
            self.state.large_text = true;
        }
        if scene == "solitaire_hint" {
            self.state.card_hint = Some(card_hints::solitaire(&self.state));
        } else if scene == "freecell_hint" {
            self.state.card_hint = Some(card_hints::freecell(&self.state));
        } else if scene == "pyramid_hint" || scene == "pyramid_hint_accessible" {
            self.state.card_hint = Some(card_hints::pyramid(&self.state));
        } else if scene == "tri_peaks_hint" || scene == "tri_peaks_hint_accessible" {
            self.state.card_hint = Some(card_hints::tri_peaks(&self.state));
        } else if scene == "klondike_golf_hint" || scene == "klondike_golf_hint_accessible" {
            self.state.card_hint = Some(card_hints::klondike_golf(&self.state));
        } else if scene == "spider_solitaire_hint" || scene == "spider_solitaire_hint_accessible" {
            self.state.card_hint = Some(card_hints::spider_solitaire(&self.state));
        } else if scene == "nim_hint" || scene == "nim_hint_accessible" {
            self.state.card_hint = Some(card_hints::nim(&self.state));
        } else if scene == "snake_hint" || scene == "snake_hint_accessible" {
            self.state.card_hint = Some(card_hints::snake(&self.state));
        } else if scene == "breakout_hint" || scene == "breakout_hint_accessible" {
            self.state.card_hint = Some(card_hints::breakout(&self.state));
        } else if scene == "higher_lower_hint" || scene == "higher_lower_hint_accessible" {
            self.state.card_hint = Some(card_hints::higher_lower(&self.state));
        } else if scene == "blackjack_hint" || scene == "blackjack_hint_accessible" {
            self.state.card_hint = Some(card_hints::blackjack(&self.state));
        } else if scene == "dungeon_sweeper_hint" || scene == "dungeon_sweeper_hint_accessible" {
            self.state.card_hint = Some(card_hints::dungeon_sweeper(&self.state));
        } else if scene == "potion_2048_hint" || scene == "potion_2048_hint_accessible" {
            self.state.card_hint = Some(card_hints::potion_2048(&self.state));
        } else if scene == "tiny_tower_defence_hint"
            || scene == "tiny_tower_defence_hint_accessible"
        {
            self.state.card_hint = Some(card_hints::tiny_tower_defence(&self.state));
        } else if scene == "one_room_roguelike_hint"
            || scene == "one_room_roguelike_hint_accessible"
        {
            self.state.card_hint = Some(card_hints::one_room_roguelike(&self.state));
        } else if scene == "daily_dungeon_hint" || scene == "daily_dungeon_hint_accessible" {
            self.state.card_hint = Some(card_hints::daily_dungeon(&self.state));
        } else if scene == "dots_boxes_hint" || scene == "dots_boxes_hint_accessible" {
            self.state.card_hint = Some(card_hints::dots_boxes(&self.state));
        } else if scene == "sokoban_hint" || scene == "sokoban_hint_accessible" {
            self.state.card_hint = Some(card_hints::sokoban(&self.state));
        } else if scene == "mancala_hint" || scene == "mancala_hint_accessible" {
            self.state.card_hint = Some(card_hints::mancala(&self.state));
        } else if scene == "hanoi_hint" || scene == "hanoi_hint_accessible" {
            self.state.card_hint = Some(card_hints::hanoi(&self.state));
        } else if scene == "number_match_hint" || scene == "number_match_hint_accessible" {
            self.state.card_hint = Some(card_hints::number_match(&self.state));
        } else if scene == "flood_it_hint" || scene == "flood_it_hint_accessible" {
            self.state.card_hint = Some(card_hints::flood_it(&self.state));
        } else if scene == "color_sort_hint" || scene == "color_sort_hint_accessible" {
            self.state.card_hint = Some(card_hints::color_sort(&self.state));
        } else if scene == "battleship_hint" || scene == "battleship_hint_accessible" {
            self.state.card_hint = Some(card_hints::battleship(&self.state));
        } else if scene == "word_grid_hint" || scene == "word_grid_hint_accessible" {
            self.state.card_hint = Some(card_hints::word_grid(&self.state));
        } else if scene == "word_ladder_hint" || scene == "word_ladder_hint_accessible" {
            self.state.card_hint = Some(card_hints::word_ladder(&self.state));
        } else if scene == "pipe_loop_hint" || scene == "pipe_loop_hint_accessible" {
            self.state.card_hint = Some(card_hints::pipe_loop(&self.state));
        } else if scene == "maze_walk_hint" || scene == "maze_walk_hint_accessible" {
            self.state.card_hint = Some(card_hints::maze_walk(&self.state));
        } else if scene == "fivefold_hint" || scene == "fivefold_hint_accessible" {
            self.state.card_hint = Some(card_hints::fivefold(&self.state));
        } else if scene == "spider_hint" || scene == "spider_hint_accessible" {
            self.state.card_hint = Some(card_hints::spider(&self.state));
        } else if scene == "match_three_hint" || scene == "match_three_hint_accessible" {
            self.state.card_hint = Some(card_hints::match_three(&self.state));
        } else if scene == "2048_hint" || scene == "2048_hint_accessible" {
            self.state.card_hint = Some(card_hints::game_2048(&self.state));
        } else if scene == "tic_tac_toe_hint" || scene == "tic_tac_toe_hint_accessible" {
            self.state.card_hint = Some(card_hints::tic_tac_toe(&self.state));
        } else if scene == "lights_out_hint" || scene == "lights_out_hint_accessible" {
            self.state.card_hint = Some(card_hints::lights_out(&self.state));
        } else if scene == "memory_pairs_hint" || scene == "memory_pairs_hint_accessible" {
            self.state.card_hint = Some(card_hints::memory_pairs(&self.state));
        } else if scene == "sliding_puzzle_hint" || scene == "sliding_puzzle_hint_accessible" {
            self.state.card_hint = Some(card_hints::sliding_puzzle(&self.state));
        } else if scene == "mastermind_hint" || scene == "mastermind_hint_accessible" {
            self.state.card_hint = Some(card_hints::mastermind(&self.state));
        } else if scene == "sudoku_hint" || scene == "sudoku_hint_accessible" {
            self.state.card_hint = Some(card_hints::sudoku(&self.state));
        } else if scene == "minesweeper_hint" || scene == "minesweeper_hint_accessible" {
            self.state.card_hint = Some(card_hints::minesweeper(&self.state));
        } else if scene == "nonogram_hint" || scene == "nonogram_hint_accessible" {
            self.state.card_hint = Some(card_hints::nonogram(&self.state));
        } else if scene == "word_search_hint" || scene == "word_search_hint_accessible" {
            self.state.card_hint = Some(card_hints::word_search(&self.state));
        } else if scene == "hangman_hint" || scene == "hangman_hint_accessible" {
            self.state.card_hint = Some(card_hints::hangman(&self.state));
        } else if scene == "connect_four_hint" || scene == "connect_four_hint_accessible" {
            self.state.card_hint = Some(card_hints::connect_four(&self.state));
        } else if scene == "checkers_hint" || scene == "checkers_hint_accessible" {
            self.state.card_hint = Some(card_hints::checkers(&self.state));
        } else if scene == "reversi_hint" || scene == "reversi_hint_accessible" {
            self.state.card_hint = Some(card_hints::reversi(&self.state));
        } else if scene == "peg_solitaire_hint" || scene == "peg_solitaire_hint_accessible" {
            self.state.card_hint = Some(card_hints::peg_solitaire(&self.state));
        } else if scene == "mahjong_solitaire_hint" || scene == "mahjong_solitaire_hint_accessible"
        {
            self.state.card_hint = Some(card_hints::mahjong_solitaire(&self.state));
        }
        if tutorial {
            if let Screen::Game(game) = self.state.screen {
                self.state.tutorial = Some(game);
            }
        }
        self.transition = 0.;
    }
}
