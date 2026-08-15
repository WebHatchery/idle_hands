//! Deterministic touch-first Mahjong Solitaire with a compact layered layout.

use serde::{Deserialize, Serialize};

const TILE_COUNT: usize = 36;

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

type Snapshot = (Vec<Tile>, MahjongStatus, u16);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MahjongSolitaire {
    pub tiles: Vec<Tile>,
    pub status: MahjongStatus,
    pub moves: u16,
    pub seed: u64,
    pub selected: Option<usize>,
    #[serde(skip)]
    undo: Option<Snapshot>,
}

impl Default for MahjongSolitaire {
    fn default() -> Self {
        Self::new(0x0A40_0001)
    }
}

impl MahjongSolitaire {
    pub fn new(seed: u64) -> Self {
        let mut positions = Vec::new();
        for y in 0..4 {
            for x in 0..8 {
                if (y == 0 || y == 3) && (1..7).contains(&x) || (1..3).contains(&y) {
                    positions.push((x, y, 0));
                }
            }
        }
        for x in 1..7 {
            positions.push((x, 1, 1));
        }
        positions.push((3, 1, 2));
        positions.push((4, 1, 2));
        let mut tiles = Vec::with_capacity(TILE_COUNT);
        for (index, (x, y, layer)) in positions.into_iter().enumerate() {
            tiles.push(Tile {
                kind: ((index / 2 + seed as usize) % 18) as u8,
                x,
                y,
                layer,
                removed: false,
            });
        }
        Self {
            tiles,
            status: MahjongStatus::Playing,
            moves: 0,
            seed,
            selected: None,
            undo: None,
        }
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
        *self = Self::new(seed);
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

    fn side_blocked(&self, tile: &Tile, direction: i8) -> bool {
        self.tiles.iter().any(|other| {
            !other.removed
                && other.layer == tile.layer
                && other.y == tile.y
                && i16::from(other.x) == i16::from(tile.x) + i16::from(direction)
        })
    }

    fn resolve(&mut self) {
        if self.tiles.iter().all(|tile| tile.removed) {
            self.status = MahjongStatus::Won;
        } else if !self.has_pair() {
            self.status = MahjongStatus::Stuck;
        }
    }

    fn has_pair(&self) -> bool {
        self.tiles.iter().enumerate().any(|(index, tile)| {
            self.available(index)
                && self.tiles.iter().enumerate().any(|(other_index, other)| {
                    other_index > index && other.kind == tile.kind && self.available(other_index)
                })
        })
    }
}

fn overlaps(left: &Tile, right: &Tile) -> bool {
    i16::from(left.x).abs_diff(i16::from(right.x)) <= 1
        && i16::from(left.y).abs_diff(i16::from(right.y)) <= 1
}

#[cfg(test)]
mod tests;
