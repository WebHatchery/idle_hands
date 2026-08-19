//! Deterministic touch-first Towers of Hanoi.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

const DEFAULT_DISKS: u8 = 5;
const DISK_VARIANTS: [u8; 3] = [3, 4, 5];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HanoiPhase {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hanoi {
    pub stacks: [Vec<u8>; 3],
    pub selected: Option<usize>,
    pub moves: u16,
    pub seed: u64,
    #[serde(default = "default_disks")]
    pub disks: u8,
    pub phase: HanoiPhase,
    #[serde(skip)]
    undo: Option<Box<Self>>,
}

impl Default for Hanoi {
    fn default() -> Self {
        Self::new(0x0048_414E_4F49)
    }
}

impl Hanoi {
    pub fn new(seed: u64) -> Self {
        Self::new_with_disks(seed, DEFAULT_DISKS)
    }

    pub fn new_with_seed(seed: u64) -> Self {
        let index = ((seed ^ seed.rotate_left(19)) % DISK_VARIANTS.len() as u64) as usize;
        Self::new_with_disks(seed, DISK_VARIANTS[index])
    }

    pub fn new_with_disks(seed: u64, disks: u8) -> Self {
        let disks = disks.clamp(3, DEFAULT_DISKS);
        let stack = (1..=disks).rev().collect();
        Self {
            stacks: [stack, Vec::new(), Vec::new()],
            selected: None,
            moves: 0,
            seed,
            disks,
            phase: HanoiPhase::Playing,
            undo: None,
        }
    }

    pub fn tap_peg(&mut self, peg: usize) -> bool {
        if self.phase != HanoiPhase::Playing || peg >= 3 {
            return false;
        }
        let Some(source) = self.selected else {
            if self.stacks[peg].is_empty() {
                return false;
            }
            self.selected = Some(peg);
            return true;
        };
        if source == peg {
            self.selected = None;
            return true;
        }
        let Some(&disk) = self.stacks[source].last() else {
            self.selected = None;
            return false;
        };
        if self.stacks[peg].last().is_some_and(|&top| top < disk) {
            self.selected = None;
            return false;
        }
        let previous = self.clone_without_undo();
        self.undo = Some(Box::new(previous));
        self.stacks[source].pop();
        self.stacks[peg].push(disk);
        self.moves = self.moves.saturating_add(1);
        self.selected = None;
        if self.stacks[2].len() == self.disks as usize {
            self.phase = HanoiPhase::Won;
        }
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
        *self = Self::new_with_disks(seed, self.disks);
    }

    pub fn reset_next(&mut self, seed: u64) {
        let index = DISK_VARIANTS
            .iter()
            .position(|candidate| *candidate == self.disks)
            .unwrap_or(DISK_VARIANTS.len() - 1);
        *self = Self::new_with_disks(seed, DISK_VARIANTS[(index + 1) % DISK_VARIANTS.len()]);
    }

    pub fn won(&self) -> bool {
        self.phase == HanoiPhase::Won
    }

    pub fn hint_move(&self) -> Option<(usize, usize)> {
        if self.won() {
            return None;
        }
        let mut queue = VecDeque::from([(self.stacks.clone(), None)]);
        let mut seen: Vec<[Vec<u8>; 3]> = Vec::new();
        while let Some((stacks, first)) = queue.pop_front() {
            if seen.contains(&stacks) {
                continue;
            }
            seen.push(stacks.clone());
            for source in 0..3 {
                let Some(&disk) = stacks[source].last() else {
                    continue;
                };
                for destination in 0..3 {
                    if source == destination
                        || stacks[destination].last().is_some_and(|&top| top < disk)
                    {
                        continue;
                    }
                    let mut next = stacks.clone();
                    next[source].pop();
                    next[destination].push(disk);
                    let first = first.or(Some((source, destination)));
                    if next[2].len() == self.disks as usize {
                        return first;
                    }
                    queue.push_back((next, first));
                }
            }
        }
        None
    }

    fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.undo = None;
        copy
    }
}

fn default_disks() -> u8 {
    DEFAULT_DISKS
}

#[cfg(test)]
mod tests;
