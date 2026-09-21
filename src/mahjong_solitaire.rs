//! Deterministic touch-first Mahjong Solitaire with a compact layered layout.

use serde::{Deserialize, Serialize};

pub const TILE_COUNT: usize = 36;
pub const PAIR_COUNT: usize = TILE_COUNT / 2;

pub type Position = (u8, u8, u8);
pub type PairPositions = (Position, Position);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tile {
    pub kind: u8,
    pub x: u8,
    pub y: u8,
    pub layer: u8,
    pub removed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MahjongStatus {
    Playing,
    Won,
    Stuck,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MahjongLayout {
    #[default]
    Classic,
    Temple,
}

impl MahjongLayout {
    pub const ALL: [Self; 2] = [Self::Classic, Self::Temple];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Classic => "CLASSIC · BRIDGE",
            Self::Temple => "TEMPLE · OFFSET BRIDGE",
        }
    }
}

pub type Snapshot = (Vec<Tile>, MahjongStatus, u16);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MahjongSolitaire {
    pub tiles: Vec<Tile>,
    pub status: MahjongStatus,
    pub moves: u16,
    pub seed: u64,
    pub selected: Option<usize>,
    #[serde(default)]
    pub layout: MahjongLayout,
    #[serde(skip)]
    pub undo: Option<Snapshot>,
}

impl Default for MahjongSolitaire {
    fn default() -> Self {
        Self::new(0x0A40_0001)
    }
}

impl MahjongSolitaire {
    pub fn new(seed: u64) -> Self {
        Self::new_with_layout(seed, MahjongLayout::default())
    }

    pub fn new_with_layout(seed: u64, layout: MahjongLayout) -> Self {
        let positions = layout_positions(layout);
        let solution_pairs = solution_pairs(layout);
        let pair_kinds = shuffled_pair_kinds(seed);
        debug_assert_eq!(positions.len(), TILE_COUNT);
        debug_assert_eq!(solution_pairs.len(), PAIR_COUNT);
        let tiles = positions
            .into_iter()
            .map(|position| {
                let pair_index = solution_pairs
                    .iter()
                    .position(|(left, right)| *left == position || *right == position)
                    // Layout construction and solution generation share the same tile set.
                    .expect("every Mahjong tile belongs to one solution pair");
                Tile {
                    kind: pair_kinds[pair_index],
                    x: position.0,
                    y: position.1,
                    layer: position.2,
                    removed: false,
                }
            })
            .collect();
        let mut game = Self {
            tiles,
            status: MahjongStatus::Playing,
            moves: 0,
            seed,
            selected: None,
            layout,
            undo: None,
        };
        game.resolve();
        game
    }

    pub fn tap(&mut self, index: usize) -> bool {
        if index >= self.tiles.len()
            || self.status != MahjongStatus::Playing
            || !self.available(index)
        {
            return false;
        }
        if let Some(selected) = self.selected {
            if selected == index {
                self.selected = None;
                return true;
            }
            if self.tiles[selected].kind != self.tiles[index].kind {
                self.selected = Some(index);
                return true;
            }
            self.undo = Some((self.tiles.clone(), self.status, self.moves));
            self.tiles[selected].removed = true;
            self.tiles[index].removed = true;
            self.moves = self.moves.saturating_add(1);
            self.selected = None;
            self.resolve();
            true
        } else {
            self.selected = Some(index);
            true
        }
    }

    pub fn undo(&mut self) -> bool {
        if let Some((tiles, status, moves)) = self.undo.take() {
            self.tiles = tiles;
            self.status = status;
            self.moves = moves;
            self.selected = None;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new_with_layout(seed, self.layout);
    }

    pub fn hint_pair(&self) -> Option<(usize, usize)> {
        if self.status != MahjongStatus::Playing {
            return None;
        }
        (0..self.tiles.len()).find_map(|first| {
            if !self.available(first) {
                return None;
            }
            (first + 1..self.tiles.len()).find_map(|second| {
                (self.available(second) && self.tiles[second].kind == self.tiles[first].kind)
                    .then_some((first, second))
            })
        })
    }

    pub fn available(&self, index: usize) -> bool {
        let Some(tile) = self.tiles.get(index) else {
            return false;
        };
        !tile.removed
            && !self
                .tiles
                .iter()
                .any(|other| !other.removed && other.layer > tile.layer && overlaps(tile, other))
            && (!self.side_blocked(tile, -1) || !self.side_blocked(tile, 1))
    }

    pub fn side_blocked(&self, tile: &Tile, direction: i8) -> bool {
        self.tiles.iter().any(|other| {
            !other.removed
                && other.layer == tile.layer
                && other.y == tile.y
                && i16::from(other.x) == i16::from(tile.x) + i16::from(direction)
        })
    }

    pub fn resolve(&mut self) {
        if self.tiles.iter().all(|tile| tile.removed) {
            self.status = MahjongStatus::Won;
        } else if !self.has_pair() {
            self.status = MahjongStatus::Stuck;
        }
    }

    pub fn has_pair(&self) -> bool {
        self.tiles.iter().enumerate().any(|(index, tile)| {
            self.available(index)
                && self.tiles.iter().enumerate().any(|(other_index, other)| {
                    other_index > index && other.kind == tile.kind && self.available(other_index)
                })
        })
    }
}

pub fn layout_positions(layout: MahjongLayout) -> Vec<(u8, u8, u8)> {
    let mut positions = Vec::with_capacity(TILE_COUNT);
    for y in 0..4u8 {
        for x in 0..8u8 {
            let base = if matches!(layout, MahjongLayout::Classic) {
                (y == 0 || y == 3) && (1..7).contains(&x) || (1..3).contains(&y)
            } else {
                true
            };
            if base {
                positions.push((x, y, 0));
            }
        }
    }
    if matches!(layout, MahjongLayout::Classic) {
        for x in 1..7u8 {
            positions.push((x, 1, 1));
        }
        positions.push((3, 1, 2));
        positions.push((4, 1, 2));
    } else {
        for x in 2..6u8 {
            positions.push((x, 1, 1));
        }
    }
    positions
}

pub fn solution_pairs(layout: MahjongLayout) -> Vec<PairPositions> {
    let mut pairs = Vec::with_capacity(PAIR_COUNT);
    match layout {
        MahjongLayout::Classic => {
            pairs.push(((3, 1, 2), (4, 1, 2)));
            append_row_pairs(&mut pairs, 1, 1, 6, 1);
            append_base_row_pairs(&mut pairs, &[(0, 1, 6), (1, 0, 7), (2, 0, 7), (3, 1, 6)]);
        }
        MahjongLayout::Temple => {
            append_row_pairs(&mut pairs, 1, 2, 5, 1);
            append_base_row_pairs(&mut pairs, &[(0, 0, 7), (1, 0, 7), (2, 0, 7), (3, 0, 7)]);
        }
    }
    pairs
}

pub fn append_base_row_pairs(pairs: &mut Vec<PairPositions>, rows: &[(u8, u8, u8)]) {
    for &(y, first_x, last_x) in rows {
        append_row_pairs(pairs, y, first_x, last_x, 0);
    }
}

pub fn append_row_pairs(pairs: &mut Vec<PairPositions>, y: u8, first_x: u8, last_x: u8, layer: u8) {
    let mut left = first_x;
    let mut right = last_x;
    while left < right {
        pairs.push(((left, y, layer), (right, y, layer)));
        left += 1;
        right -= 1;
    }
}

pub fn shuffled_pair_kinds(seed: u64) -> [u8; PAIR_COUNT] {
    let mut kinds = [0; PAIR_COUNT];
    for (index, kind) in kinds.iter_mut().enumerate() {
        *kind = index as u8;
    }
    let mut state = seed ^ 0x9E37_79B9_7F4A_7C15;
    for index in (1..PAIR_COUNT).rev() {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let swap = (state % (index as u64 + 1)) as usize;
        kinds.swap(index, swap);
    }
    kinds
}

pub fn overlaps(left: &Tile, right: &Tile) -> bool {
    left.x == right.x && left.y == right.y
}
