//! Touch-first falling-block stack, a compact Tetris-like cabinet game.

use serde::{Deserialize, Serialize};

pub const WIDTH: u8 = 10;
pub const HEIGHT: u8 = 20;
const TARGET_LINES: u16 = 20;
const STEP_INTERVAL: f32 = 0.55;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockMove {
    Left,
    Right,
    Rotate,
    Drop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockStatus {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone)]
struct Snapshot {
    board: Vec<u8>,
    piece: u8,
    next_piece: u8,
    rotation: u8,
    piece_x: i8,
    piece_y: i8,
    score: u32,
    lines: u16,
    level: u8,
    moves: u16,
    status: BlockStatus,
    seed: u64,
    paused: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStack {
    pub board: Vec<u8>,
    pub piece: u8,
    pub next_piece: u8,
    pub rotation: u8,
    pub piece_x: i8,
    pub piece_y: i8,
    pub score: u32,
    pub lines: u16,
    pub level: u8,
    pub moves: u16,
    pub status: BlockStatus,
    pub seed: u64,
    pub paused: bool,
    #[serde(default)]
    pub mode: u8,
    #[serde(skip)]
    undo: Option<Box<Snapshot>>,
    #[serde(skip)]
    elapsed: f32,
}

impl Default for BlockStack {
    fn default() -> Self {
        Self::new(0xB10C_0001)
    }
}

impl BlockStack {
    pub fn new(seed: u64) -> Self {
        let mut game = Self {
            board: vec![0; usize::from(WIDTH) * usize::from(HEIGHT)],
            piece: 0,
            next_piece: 0,
            rotation: 0,
            piece_x: 3,
            piece_y: 0,
            score: 0,
            lines: 0,
            level: 1,
            moves: 0,
            status: BlockStatus::Playing,
            seed,
            paused: false,
            mode: 0,
            undo: None,
            elapsed: 0.,
        };
        game.piece = game.draw_piece();
        game.next_piece = game.draw_piece();
        game
    }

    pub fn apply_move(&mut self, movement: BlockMove) -> bool {
        if self.status != BlockStatus::Playing || self.paused {
            return false;
        }
        match movement {
            BlockMove::Left if self.can_place(self.piece_x - 1, self.piece_y, self.rotation) => {
                self.snapshot();
                self.piece_x -= 1;
            }
            BlockMove::Right if self.can_place(self.piece_x + 1, self.piece_y, self.rotation) => {
                self.snapshot();
                self.piece_x += 1;
            }
            BlockMove::Rotate => {
                let rotation = (self.rotation + 1) % 4;
                let x = if self.can_place(self.piece_x, self.piece_y, rotation) {
                    Some(self.piece_x)
                } else if self.can_place(self.piece_x - 1, self.piece_y, rotation) {
                    Some(self.piece_x - 1)
                } else if self.can_place(self.piece_x + 1, self.piece_y, rotation) {
                    Some(self.piece_x + 1)
                } else {
                    None
                };
                if let Some(x) = x {
                    self.snapshot();
                    self.rotation = rotation;
                    self.piece_x = x;
                } else {
                    return false;
                }
            }
            BlockMove::Drop => {
                self.snapshot();
                while self.can_place(self.piece_x, self.piece_y + 1, self.rotation) {
                    self.piece_y += 1;
                }
                self.lock_piece();
            }
            _ => return false,
        }
        self.moves = self.moves.saturating_add(1);
        true
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        if self.status != BlockStatus::Playing || self.paused {
            return false;
        }
        self.elapsed += dt.max(0.);
        let interval = (STEP_INTERVAL - f32::from(self.level.saturating_sub(1)) * 0.04).max(0.18);
        if self.elapsed < interval {
            return false;
        }
        self.elapsed = 0.;
        self.advance_one();
        true
    }

    pub fn toggle_pause(&mut self) -> bool {
        if self.status != BlockStatus::Playing {
            return false;
        }
        self.paused = !self.paused;
        if self.paused {
            self.elapsed = 0.;
        }
        true
    }

    pub fn undo(&mut self) -> bool {
        let Some(snapshot) = self.undo.take() else {
            return false;
        };
        self.board = snapshot.board;
        self.piece = snapshot.piece;
        self.next_piece = snapshot.next_piece;
        self.rotation = snapshot.rotation;
        self.piece_x = snapshot.piece_x;
        self.piece_y = snapshot.piece_y;
        self.score = snapshot.score;
        self.lines = snapshot.lines;
        self.level = snapshot.level;
        self.moves = snapshot.moves;
        self.status = snapshot.status;
        self.seed = snapshot.seed;
        self.paused = snapshot.paused;
        self.elapsed = 0.;
        true
    }

    pub fn reset(&mut self, seed: u64) {
        let mode = self.mode;
        *self = Self::new(seed);
        self.mode = mode;
    }

    pub fn cycle_mode(&mut self) {
        let seed = self.seed.wrapping_add(1);
        let mode = (self.mode + 1) % 2;
        *self = Self::new(seed);
        self.mode = mode;
    }

    pub const fn mode_label(&self) -> &'static str {
        if self.mode == 0 {
            "CLASSIC"
        } else {
            "RUSH"
        }
    }

    pub const fn target_lines() -> u16 {
        TARGET_LINES
    }

    pub fn active_cells(&self) -> [(i8, i8); 4] {
        piece_cells(self.piece, self.rotation)
    }

    fn advance_one(&mut self) {
        self.snapshot();
        if self.can_place(self.piece_x, self.piece_y + 1, self.rotation) {
            self.piece_y += 1;
        } else {
            self.lock_piece();
        }
    }

    fn lock_piece(&mut self) {
        for (x, y) in self.active_cells() {
            let board_x = self.piece_x + x;
            let board_y = self.piece_y + y;
            if board_y < 0 || board_x < 0 || board_x >= WIDTH as i8 || board_y >= HEIGHT as i8 {
                self.status = BlockStatus::Lost;
                return;
            }
            self.board
                [usize::from(board_y as u8) * usize::from(WIDTH) + usize::from(board_x as u8)] =
                self.piece + 1;
        }
        let cleared = self.clear_lines();
        self.lines = self.lines.saturating_add(cleared);
        self.score = self
            .score
            .saturating_add(u32::from(cleared) * 100 * u32::from(self.level));
        self.level = 1 + (self.lines / 5) as u8;
        if self.lines >= TARGET_LINES {
            self.status = BlockStatus::Won;
            self.paused = false;
            return;
        }
        self.piece = self.next_piece;
        self.next_piece = self.draw_piece();
        self.rotation = 0;
        self.piece_x = 3;
        self.piece_y = 0;
        if !self.can_place(self.piece_x, self.piece_y, self.rotation) {
            self.status = BlockStatus::Lost;
            self.paused = false;
        }
    }

    fn clear_lines(&mut self) -> u16 {
        let mut kept = Vec::with_capacity(self.board.len());
        let mut cleared = 0;
        for row in self.board.chunks(usize::from(WIDTH)) {
            if row.iter().all(|cell| *cell != 0) {
                cleared += 1;
            } else {
                kept.extend_from_slice(row);
            }
        }
        while kept.len() < self.board.len() {
            for _ in 0..WIDTH {
                kept.insert(0, 0);
            }
        }
        self.board = kept;
        cleared
    }

    fn can_place(&self, x: i8, y: i8, rotation: u8) -> bool {
        piece_cells(self.piece, rotation)
            .into_iter()
            .all(|(cell_x, cell_y)| {
                let board_x = x + cell_x;
                let board_y = y + cell_y;
                board_x >= 0
                    && board_x < WIDTH as i8
                    && board_y < HEIGHT as i8
                    && (board_y < 0
                        || self.board[usize::from(board_y as u8) * usize::from(WIDTH)
                            + usize::from(board_x as u8)]
                            == 0)
            })
    }

    fn draw_piece(&mut self) -> u8 {
        self.seed = self
            .seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.seed % 7) as u8
    }

    fn snapshot(&mut self) {
        self.undo = Some(Box::new(Snapshot {
            board: self.board.clone(),
            piece: self.piece,
            next_piece: self.next_piece,
            rotation: self.rotation,
            piece_x: self.piece_x,
            piece_y: self.piece_y,
            score: self.score,
            lines: self.lines,
            level: self.level,
            moves: self.moves,
            status: self.status,
            seed: self.seed,
            paused: self.paused,
        }));
    }
}

fn piece_cells(kind: u8, rotation: u8) -> [(i8, i8); 4] {
    let mut cells = match kind % 7 {
        0 => [(0, 1), (1, 1), (2, 1), (3, 1)],
        1 => [(0, 0), (1, 0), (0, 1), (1, 1)],
        2 => [(1, 0), (0, 1), (1, 1), (2, 1)],
        3 => [(0, 0), (0, 1), (1, 1), (2, 1)],
        4 => [(2, 0), (0, 1), (1, 1), (2, 1)],
        5 => [(1, 0), (2, 0), (0, 1), (1, 1)],
        _ => [(0, 0), (1, 0), (1, 1), (2, 1)],
    };
    if kind % 7 == 1 {
        return cells;
    }
    for _ in 0..rotation % 4 {
        let pivot = if kind.is_multiple_of(7) { 3 } else { 2 };
        for cell in &mut cells {
            let next = (pivot - cell.1, cell.0);
            cell.0 = next.0;
            cell.1 = next.1;
        }
    }
    cells
}

#[cfg(test)]
mod tests;
