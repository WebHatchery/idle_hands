//! Deterministic touch-first Match Three swap-and-clear puzzle.

use serde::{Deserialize, Serialize};

use crate::data::MatchThreeConfig;

pub const EMPTY: u8 = u8::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MatchThreeDifficulty {
    #[default]
    Standard,
    Hard,
    Expert,
}

impl MatchThreeDifficulty {
    pub const ALL: [Self; 3] = [Self::Standard, Self::Hard, Self::Expert];

    pub fn label(self) -> &'static str {
        match self {
            Self::Standard => "STANDARD",
            Self::Hard => "HARD",
            Self::Expert => "EXPERT",
        }
    }

    pub fn index(self) -> usize {
        match self {
            Self::Standard => 0,
            Self::Hard => 1,
            Self::Expert => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchThreePhase {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchThreeSpecial {
    #[default]
    None,
    Row,
    Column,
    Burst,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchThree {
    pub cells: Vec<u8>,
    pub selected: Option<usize>,
    pub score: u16,
    pub moves: u16,
    #[serde(default)]
    pub specials: Vec<MatchThreeSpecial>,
    #[serde(default)]
    pub last_cascade: u8,
    #[serde(default)]
    pub best_cascade: u8,
    #[serde(default)]
    pub reshuffles: u16,
    pub seed: u64,
    #[serde(default)]
    pub difficulty: MatchThreeDifficulty,
    #[serde(default = "default_side")]
    pub side: usize,
    #[serde(default = "default_colors")]
    pub colors: u8,
    #[serde(default = "default_target_score")]
    pub target_score: u16,
    #[serde(default)]
    pub move_limit: u16,
    pub phase: MatchThreePhase,
    #[serde(skip)]
    pub undo: Option<Box<Self>>,
}

impl Default for MatchThree {
    fn default() -> Self {
        Self::new(0x004D_4154_4348)
    }
}

impl MatchThree {
    pub fn new(seed: u64) -> Self {
        Self::new_with_difficulty(seed, MatchThreeDifficulty::Standard)
    }

    pub fn new_with_difficulty(seed: u64, difficulty: MatchThreeDifficulty) -> Self {
        Self::new_with_config(seed, difficulty, &MatchThreeConfig::default())
    }

    pub fn new_with_config(
        seed: u64,
        difficulty: MatchThreeDifficulty,
        config: &MatchThreeConfig,
    ) -> Self {
        // GameData validation guarantees one row for every difficulty variant.
        let settings = config
            .difficulties
            .get(difficulty.index())
            .expect("validated Match Three difficulty configuration");
        Self::new_with_settings(
            seed,
            difficulty,
            settings.side,
            settings.colors,
            settings.target_score,
            settings.move_limit,
        )
    }

    pub fn new_with_settings(
        mut seed: u64,
        difficulty: MatchThreeDifficulty,
        side: usize,
        colors: u8,
        target_score: u16,
        move_limit: u16,
    ) -> Self {
        let mut cells = vec![0; side * side];
        for index in 0..side * side {
            seed = next_seed(seed);
            let start = (seed % colors as u64) as u8;
            cells[index] = (0..colors)
                .map(|offset| (start + offset) % colors)
                .find(|&color| !creates_match_for_side(&cells, index, color, side))
                .unwrap_or(start);
        }
        let mut game = Self {
            cells,
            selected: None,
            score: 0,
            moves: 0,
            specials: vec![MatchThreeSpecial::None; side * side],
            last_cascade: 0,
            best_cascade: 0,
            reshuffles: 0,
            seed,
            difficulty,
            side,
            colors,
            target_score,
            move_limit,
            phase: MatchThreePhase::Playing,
            undo: None,
        };
        if !game.has_legal_swap() {
            game.reshuffle();
            game.reshuffles = 0;
        }
        game
    }

    pub fn tap(&mut self, index: usize) -> bool {
        self.normalize_specials();
        if self.phase != MatchThreePhase::Playing || index >= self.side() * self.side() {
            return false;
        }
        let Some(first) = self.selected else {
            self.selected = Some(index);
            return true;
        };
        if first == index {
            self.selected = None;
            return true;
        }
        if !adjacent_for_side(first, index, self.side()) {
            return false;
        }
        let previous = self.clone_without_undo();
        self.cells.swap(first, index);
        self.specials.swap(first, index);
        let activates_special = self.specials[first] != MatchThreeSpecial::None
            || self.specials[index] != MatchThreeSpecial::None;
        let initial_matches = find_matches_for_side(&self.cells, self.side());
        if initial_matches.iter().all(|&matched| !matched) && !activates_special {
            self.cells.swap(first, index);
            self.specials.swap(first, index);
            return false;
        }
        self.undo = Some(Box::new(previous));
        self.selected = None;
        self.moves = self.moves.saturating_add(1);
        self.resolve(first, index, activates_special);
        true
    }

    pub fn undo(&mut self) -> bool {
        let Some(previous) = self.undo.take() else {
            return false;
        };
        *self = *previous;
        true
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new_with_settings(
            seed,
            self.difficulty,
            self.side,
            self.colors,
            self.target_score,
            self.move_limit(),
        );
    }
    pub fn won(&self) -> bool {
        self.phase == MatchThreePhase::Won
    }

    pub fn hint_swap(&self) -> Option<(usize, usize)> {
        if self.phase != MatchThreePhase::Playing {
            return None;
        }
        let mut best: Option<(u16, usize, usize)> = None;
        for first in 0..self.cells.len() {
            for second in [first + 1, first + self.side()] {
                if second >= self.cells.len() || !adjacent_for_side(first, second, self.side()) {
                    continue;
                }
                let mut candidate = self.clone_without_undo();
                candidate.selected = None;
                let before = candidate.score;
                if !candidate.tap(first) || !candidate.tap(second) {
                    continue;
                }
                let gain = candidate.score.saturating_sub(before);
                if best.is_none_or(|(best_gain, _, _)| gain > best_gain) {
                    best = Some((gain, first, second));
                }
            }
        }
        best.map(|(_, first, second)| (first, second))
    }

    pub fn resolve(&mut self, first: usize, second: usize, activates_special: bool) {
        let mut cascade = 0_u8;
        let mut first_pass = true;
        loop {
            let mut matches = find_matches_for_side(&self.cells, self.side());
            if first_pass && activates_special {
                matches[first] = true;
                matches[second] = true;
            }
            let created = self.special_for_match(&matches, first_pass.then_some(second));
            self.expand_specials(&mut matches);
            if let Some((index, special)) = created {
                matches[index] = false;
                self.specials[index] = special;
            }
            let removed = matches.iter().filter(|&&matched| matched).count();
            if removed == 0 {
                break;
            }
            cascade = cascade.saturating_add(1);
            self.score = self
                .score
                .saturating_add((removed as u16).saturating_mul(10 * u16::from(cascade)));
            for (index, matched) in matches.into_iter().enumerate() {
                if matched {
                    self.cells[index] = EMPTY;
                    self.specials[index] = MatchThreeSpecial::None;
                }
            }
            self.collapse_columns();
            first_pass = false;
        }
        self.last_cascade = cascade;
        self.best_cascade = self.best_cascade.max(cascade);
        if self.score >= self.target_score() {
            self.phase = MatchThreePhase::Won;
        } else if self.moves >= self.move_limit() {
            self.phase = MatchThreePhase::Lost;
        } else if !self.has_legal_swap() {
            self.reshuffle();
        }
    }

    pub fn special_for_match(
        &self,
        matches: &[bool],
        preferred: Option<usize>,
    ) -> Option<(usize, MatchThreeSpecial)> {
        let side = self.side();
        let mut candidates = Vec::with_capacity(matches.len() + 1);
        if let Some(index) = preferred.filter(|&index| matches[index]) {
            candidates.push(index);
        }
        for (index, &matched) in matches.iter().enumerate() {
            if matched && !candidates.contains(&index) {
                candidates.push(index);
            }
        }
        candidates.into_iter().find_map(|candidate| {
            let color = self.cells[candidate];
            let row = candidate / side;
            let col = candidate % side;
            let horizontal = contiguous_run(col, side, |x| {
                matches[row * side + x] && self.cells[row * side + x] == color
            });
            let vertical = contiguous_run(row, side, |y| {
                matches[y * side + col] && self.cells[y * side + col] == color
            });
            let special = if horizontal >= 3 && vertical >= 3 || horizontal >= 5 || vertical >= 5 {
                MatchThreeSpecial::Burst
            } else if horizontal >= 4 {
                MatchThreeSpecial::Row
            } else if vertical >= 4 {
                MatchThreeSpecial::Column
            } else {
                return None;
            };
            Some((candidate, special))
        })
    }

    pub fn expand_specials(&self, matches: &mut [bool]) {
        let side = self.side();
        let mut pending: Vec<usize> = matches
            .iter()
            .enumerate()
            .filter_map(|(index, &matched)| matched.then_some(index))
            .collect();
        let mut visited = vec![false; matches.len()];
        while let Some(index) = pending.pop() {
            if visited[index] {
                continue;
            }
            visited[index] = true;
            let row = index / side;
            let col = index % side;
            let affected: Vec<usize> = match self.specials[index] {
                MatchThreeSpecial::None => Vec::new(),
                MatchThreeSpecial::Row => (0..side).map(|x| row * side + x).collect(),
                MatchThreeSpecial::Column => (0..side).map(|y| y * side + col).collect(),
                MatchThreeSpecial::Burst => {
                    let row_start = row.saturating_sub(1);
                    let row_end = (row + 1).min(side - 1);
                    let col_start = col.saturating_sub(1);
                    let col_end = (col + 1).min(side - 1);
                    (row_start..=row_end)
                        .flat_map(|y| (col_start..=col_end).map(move |x| y * side + x))
                        .collect()
                }
            };
            for affected_index in affected {
                if !matches[affected_index] {
                    matches[affected_index] = true;
                    pending.push(affected_index);
                }
            }
        }
    }

    pub fn collapse_columns(&mut self) {
        let side = self.side();
        for col in 0..side {
            // Read survivors from the top down, then place them from the
            // bottom up. This keeps each tile's vertical order as it falls.
            let mut filled: Vec<(u8, MatchThreeSpecial)> = (0..side)
                .filter_map(|row| {
                    let index = row * side + col;
                    let value = self.cells[index];
                    (value != EMPTY).then_some((value, self.specials[index]))
                })
                .collect();
            for row in (0..side).rev() {
                let index = row * side + col;
                let (color, special) = filled.pop().unwrap_or_else(|| {
                    self.seed = next_seed(self.seed);
                    (
                        (self.seed % self.color_count() as u64) as u8,
                        MatchThreeSpecial::None,
                    )
                });
                self.cells[index] = color;
                self.specials[index] = special;
            }
        }
    }

    pub fn reshuffle(&mut self) {
        self.specials.fill(MatchThreeSpecial::None);
        for attempt in 0..64_u64 {
            for index in (1..self.cells.len()).rev() {
                self.seed = next_seed(self.seed.wrapping_add(attempt));
                let swap = self.seed as usize % (index + 1);
                self.cells.swap(index, swap);
            }
            if find_matches_for_side(&self.cells, self.side())
                .iter()
                .all(|&matched| !matched)
                && self.has_legal_swap()
            {
                self.reshuffles = self.reshuffles.saturating_add(1);
                return;
            }
        }
        let replacement = Self::new_with_settings(
            self.seed.wrapping_add(1),
            self.difficulty,
            self.side,
            self.colors,
            self.target_score,
            self.move_limit(),
        );
        self.cells = replacement.cells;
        self.specials = replacement.specials;
        self.seed = replacement.seed;
        self.reshuffles = self.reshuffles.saturating_add(1);
    }

    pub fn has_legal_swap(&self) -> bool {
        let board_len = self.side() * self.side();
        for first in 0..board_len {
            for second in [first + 1, first + self.side()] {
                if second >= board_len || !adjacent_for_side(first, second, self.side()) {
                    continue;
                }
                if self.specials[first] != MatchThreeSpecial::None
                    || self.specials[second] != MatchThreeSpecial::None
                {
                    return true;
                }
                let mut cells = self.cells.clone();
                cells.swap(first, second);
                if find_matches_for_side(&cells, self.side())
                    .iter()
                    .any(|&matched| matched)
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn normalize_specials(&mut self) {
        if self.specials.len() != self.cells.len() {
            self.specials = vec![MatchThreeSpecial::None; self.cells.len()];
        }
    }

    pub fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.undo = None;
        copy
    }

    pub fn side(&self) -> usize {
        self.side
    }

    pub fn color_count(&self) -> u8 {
        self.colors
    }

    pub fn target_score(&self) -> u16 {
        self.target_score
    }

    pub fn move_limit(&self) -> u16 {
        if self.move_limit == 0 {
            MatchThreeConfig::default().difficulties[self.difficulty.index()].move_limit
        } else {
            self.move_limit
        }
    }

    pub fn moves_left(&self) -> u16 {
        self.move_limit.saturating_sub(self.moves)
    }

    pub fn special_at(&self, index: usize) -> MatchThreeSpecial {
        self.specials
            .get(index)
            .copied()
            .unwrap_or(MatchThreeSpecial::None)
    }
}

pub fn creates_match_for_side(cells: &[u8], index: usize, color: u8, side: usize) -> bool {
    let col = index % side;
    let row = index / side;
    (col >= 2 && cells[index - 1] == color && cells[index - 2] == color)
        || (row >= 2 && cells[index - side] == color && cells[index - side * 2] == color)
}

pub fn adjacent_for_side(first: usize, second: usize, side: usize) -> bool {
    let row_delta = (first / side).abs_diff(second / side);
    let col_delta = (first % side).abs_diff(second % side);
    row_delta + col_delta == 1
}

pub fn find_matches_for_side(cells: &[u8], side: usize) -> Vec<bool> {
    let cells_count = side * side;
    let mut matches = vec![false; cells_count];
    for row in 0..side {
        let mut start = 0;
        while start < side {
            let color = cells[row * side + start];
            let mut end = start + 1;
            while end < side && cells[row * side + end] == color {
                end += 1;
            }
            if color != EMPTY && end - start >= 3 {
                for col in start..end {
                    matches[row * side + col] = true;
                }
            }
            start = end;
        }
    }
    for col in 0..side {
        let mut start = 0;
        while start < side {
            let color = cells[start * side + col];
            let mut end = start + 1;
            while end < side && cells[end * side + col] == color {
                end += 1;
            }
            if color != EMPTY && end - start >= 3 {
                for row in start..end {
                    matches[row * side + col] = true;
                }
            }
            start = end;
        }
    }
    matches
}

pub fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

pub fn default_side() -> usize {
    MatchThreeConfig::default().difficulties[0].side
}

pub fn default_colors() -> u8 {
    MatchThreeConfig::default().difficulties[0].colors
}

pub fn default_target_score() -> u16 {
    MatchThreeConfig::default().difficulties[0].target_score
}

pub fn contiguous_run(origin: usize, limit: usize, predicate: impl Fn(usize) -> bool) -> usize {
    let mut start = origin;
    while start > 0 && predicate(start - 1) {
        start -= 1;
    }
    let mut end = origin;
    while end + 1 < limit && predicate(end + 1) {
        end += 1;
    }
    end - start + 1
}
