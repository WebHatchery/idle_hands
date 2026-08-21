//! Five small deterministic puzzle games for the Misc cabinet shelf.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MiscKind {
    RiddleRoom,
    PatternVault,
    SumCircuit,
    OrbitOrder,
    WordForge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MiscPhase {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscGame {
    pub kind: MiscKind,
    pub seed: u64,
    pub round: u8,
    pub score: u16,
    pub moves: u16,
    pub mistakes: u8,
    pub phase: MiscPhase,
    pub prompt: String,
    pub detail: String,
    pub options: Vec<String>,
    pub answer: usize,
    pub board: Vec<u8>,
    pub selected: Vec<usize>,
    pub solution: Vec<usize>,
    pub target: u16,
    pub target_word: String,
    #[serde(default)]
    pub hint_used: bool,
    #[serde(skip)]
    history: Vec<Self>,
}

impl Default for MiscGame {
    fn default() -> Self {
        Self::new(0x4D49_5343_0001, MiscKind::RiddleRoom)
    }
}

impl MiscGame {
    pub fn new(seed: u64, kind: MiscKind) -> Self {
        let mut game = Self {
            kind,
            seed,
            round: 0,
            score: 0,
            moves: 0,
            mistakes: 0,
            phase: MiscPhase::Playing,
            prompt: String::new(),
            detail: String::new(),
            options: Vec::new(),
            answer: 0,
            board: Vec::new(),
            selected: Vec::new(),
            solution: Vec::new(),
            target: 0,
            target_word: String::new(),
            hint_used: false,
            history: Vec::new(),
        };
        game.prepare();
        game
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed, self.kind);
    }

    pub fn tap(&mut self, index: usize) -> bool {
        if self.phase == MiscPhase::Won {
            return false;
        }
        match self.kind {
            MiscKind::RiddleRoom | MiscKind::PatternVault => self.answer_choice(index),
            MiscKind::SumCircuit => self.toggle_sum_tile(index),
            MiscKind::OrbitOrder => self.swap_orbit(index),
            MiscKind::WordForge => self.pick_letter(index),
        }
    }

    pub fn submit(&mut self) -> bool {
        if self.phase == MiscPhase::Won {
            return false;
        }
        match self.kind {
            MiscKind::SumCircuit => self.submit_sum(),
            MiscKind::WordForge => self.submit_word(),
            MiscKind::RiddleRoom | MiscKind::PatternVault | MiscKind::OrbitOrder => false,
        }
    }

    pub fn clear_selection(&mut self) -> bool {
        if self.selected.is_empty() {
            return false;
        }
        self.push_history();
        self.selected.clear();
        true
    }

    pub fn undo(&mut self) -> bool {
        let Some(previous) = self.history.pop() else {
            return false;
        };
        let mut history = std::mem::take(&mut self.history);
        *self = previous;
        self.history = std::mem::take(&mut history);
        true
    }

    pub fn hint(&mut self) -> String {
        self.hint_used = true;
        match self.kind {
            MiscKind::RiddleRoom => format!("The answer is {}.", self.options[self.answer]),
            MiscKind::PatternVault => format!("The next value is {}.", self.options[self.answer]),
            MiscKind::SumCircuit => format!(
                "Try tiles {}.",
                self.solution
                    .iter()
                    .map(|i| (i + 1).to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            MiscKind::OrbitOrder => {
                let position = self
                    .board
                    .iter()
                    .enumerate()
                    .position(|(i, value)| *value != (i + 1) as u8);
                position.map_or_else(
                    || "The order is already clear.".into(),
                    |i| {
                        format!(
                            "Planet {} belongs at position {}.",
                            self.board[i], self.board[i]
                        )
                    },
                )
            }
            MiscKind::WordForge => format!("The word is {}.", self.target_word),
        }
    }

    pub fn won(&self) -> bool {
        self.phase == MiscPhase::Won
    }

    pub fn title(&self) -> &'static str {
        match self.kind {
            MiscKind::RiddleRoom => "RIDDLE ROOM",
            MiscKind::PatternVault => "PATTERN VAULT",
            MiscKind::SumCircuit => "SUM CIRCUIT",
            MiscKind::OrbitOrder => "ORBIT ORDER",
            MiscKind::WordForge => "WORD FORGE",
        }
    }

    pub fn subtitle(&self) -> &'static str {
        match self.kind {
            MiscKind::RiddleRoom => "Solve the cabinet clues",
            MiscKind::PatternVault => "Read the next step",
            MiscKind::SumCircuit => "Complete each target",
            MiscKind::OrbitOrder => "Sort the drifting planets",
            MiscKind::WordForge => "Build the hidden word",
        }
    }

    pub fn status_text(&self) -> String {
        if self.won() {
            return "PUZZLE COMPLETE — tap NEW ROUND to play again".into();
        }
        match self.kind {
            MiscKind::RiddleRoom => "Choose the answer that fits the clue".into(),
            MiscKind::PatternVault => "Find the next value in the sequence".into(),
            MiscKind::SumCircuit => {
                format!("Target {} • selected {}", self.target, self.selected_sum())
            }
            MiscKind::OrbitOrder => "Tap two planets to swap their positions".into(),
            MiscKind::WordForge => format!("Build {} letters, then tap SUBMIT", self.board.len()),
        }
    }

    pub fn selected_sum(&self) -> u16 {
        self.selected
            .iter()
            .filter_map(|index| self.board.get(*index))
            .map(|value| u16::from(*value))
            .sum()
    }

    fn answer_choice(&mut self, index: usize) -> bool {
        if index >= self.options.len() {
            return false;
        }
        self.push_history();
        self.moves = self.moves.saturating_add(1);
        if index == self.answer {
            self.score = self.score.saturating_add(1);
            self.round = self.round.saturating_add(1);
            if self.round >= 5 {
                self.phase = MiscPhase::Won;
            } else {
                self.prepare();
            }
        } else {
            self.mistakes = self.mistakes.saturating_add(1);
            self.selected = vec![index];
        }
        true
    }

    fn toggle_sum_tile(&mut self, index: usize) -> bool {
        if index >= self.board.len() || self.board[index] == 0 {
            return false;
        }
        self.push_history();
        if let Some(position) = self.selected.iter().position(|selected| *selected == index) {
            self.selected.remove(position);
        } else {
            self.selected.push(index);
        }
        true
    }

    fn submit_sum(&mut self) -> bool {
        if self.selected.is_empty() {
            return false;
        }
        self.push_history();
        self.moves = self.moves.saturating_add(1);
        let mut selected = self.selected.clone();
        selected.sort_unstable();
        let mut solution = self.solution.clone();
        solution.sort_unstable();
        if self.selected_sum() == self.target && selected == solution {
            for index in &self.selected {
                self.board[*index] = 0;
            }
            self.score = self.score.saturating_add(self.selected.len() as u16);
            self.round = self.round.saturating_add(1);
            if self.round >= 4 {
                self.phase = MiscPhase::Won;
                self.selected.clear();
            } else {
                self.prepare();
            }
        } else {
            self.mistakes = self.mistakes.saturating_add(1);
            self.selected.clear();
        }
        true
    }

    fn swap_orbit(&mut self, index: usize) -> bool {
        if index >= self.board.len() {
            return false;
        }
        if self.selected.is_empty() {
            self.push_history();
            self.selected.push(index);
            return true;
        }
        let first = self.selected[0];
        if first == index {
            self.selected.clear();
            return true;
        }
        self.board.swap(first, index);
        self.selected.clear();
        self.moves = self.moves.saturating_add(1);
        if self
            .board
            .iter()
            .enumerate()
            .all(|(i, value)| *value == (i + 1) as u8)
        {
            self.phase = MiscPhase::Won;
            self.score = 25;
        }
        true
    }

    fn pick_letter(&mut self, index: usize) -> bool {
        if index >= self.board.len() || self.selected.contains(&index) {
            return false;
        }
        self.push_history();
        self.selected.push(index);
        true
    }

    fn submit_word(&mut self) -> bool {
        if self.selected.len() != self.board.len() {
            return false;
        }
        self.push_history();
        self.moves = self.moves.saturating_add(1);
        let guess: String = self
            .selected
            .iter()
            .map(|index| self.board[*index] as char)
            .collect();
        if guess == self.target_word {
            self.score = self.score.saturating_add(1);
            self.round = self.round.saturating_add(1);
            if self.round >= 5 {
                self.phase = MiscPhase::Won;
            } else {
                self.prepare();
            }
        } else {
            self.mistakes = self.mistakes.saturating_add(1);
            self.selected.clear();
        }
        true
    }

    fn prepare(&mut self) {
        self.selected.clear();
        self.solution.clear();
        self.options.clear();
        self.board.clear();
        self.hint_used = false;
        match self.kind {
            MiscKind::RiddleRoom => self.prepare_riddle(),
            MiscKind::PatternVault => self.prepare_pattern(),
            MiscKind::SumCircuit => self.prepare_sum(),
            MiscKind::OrbitOrder => self.prepare_orbit(),
            MiscKind::WordForge => self.prepare_word(),
        }
    }

    fn prepare_riddle(&mut self) {
        let riddle = RIDDLES[(self.seed as usize + self.round as usize) % RIDDLES.len()];
        self.prompt = riddle.question.into();
        self.detail = riddle.clue.into();
        self.options = riddle
            .answers
            .iter()
            .map(|answer| (*answer).into())
            .collect();
        self.answer = riddle.answer;
    }

    fn prepare_pattern(&mut self) {
        let family = ((self.seed as usize + self.round as usize) % 3) as u8;
        let start = 2 + ((self.seed.wrapping_add(u64::from(self.round)) % 5) as u8);
        let values: Vec<u8> = (0..4)
            .map(|step| match family {
                0 => start.saturating_add(step * 2),
                1 => start.saturating_mul(1 + step),
                _ => start.saturating_add(step * step),
            })
            .collect();
        let missing = *values.last().unwrap_or(&start);
        self.prompt = format!("{}  ·  {}  ·  {}  ·  ?", values[0], values[1], values[2]);
        self.detail = match family {
            0 => "The gap stays even".into(),
            1 => "The steps multiply".into(),
            _ => "The gaps grow by one".into(),
        };
        self.options = vec![
            missing,
            missing.saturating_add(1),
            missing.saturating_sub(2),
            missing.saturating_add(3),
        ]
        .into_iter()
        .map(|value| value.to_string())
        .collect();
        let rotation = (pseudo(self.seed, usize::from(self.round)) % 4) as usize;
        self.options.rotate_left(rotation);
        self.answer = (4 - rotation) % 4;
    }

    fn prepare_sum(&mut self) {
        self.prompt = "CONNECT THE TARGET".into();
        self.detail = "Select the three lit tiles that make the target".into();
        if self.board.is_empty() {
            self.board = (0..12)
                .map(|index| 1 + (pseudo(self.seed, index) % 8) as u8)
                .collect();
        }
        let start = usize::from(self.round) * 3;
        self.solution = vec![start, start + 1, start + 2];
        self.target = self
            .solution
            .iter()
            .map(|index| u16::from(self.board[*index]))
            .sum();
    }

    fn prepare_orbit(&mut self) {
        self.prompt = "RESTORE THE ORBIT".into();
        self.detail = "Put the numbered planets in ascending order".into();
        self.board = (1..=5).collect();
        let rotation = (pseudo(self.seed, 2) % 4 + 1) as usize;
        self.board.rotate_left(rotation);
        if self
            .board
            .iter()
            .enumerate()
            .all(|(i, value)| *value == (i + 1) as u8)
        {
            self.board.swap(0, 1);
        }
    }

    fn prepare_word(&mut self) {
        const WORDS: [&str; 5] = ["BRICK", "CABIN", "QUIET", "RUNE", "TAPES"];
        self.target_word = WORDS[(self.seed as usize + self.round as usize) % WORDS.len()].into();
        self.prompt = "FORGE THE WORD".into();
        self.detail = "Tap each letter once, then submit".into();
        self.board = self.target_word.bytes().collect();
        let rotation =
            (pseudo(self.seed, usize::from(self.round) + 9) % self.board.len() as u64) as usize;
        self.board.rotate_left(rotation);
    }

    fn push_history(&mut self) {
        let mut snapshot = self.clone();
        snapshot.history.clear();
        self.history.push(snapshot);
        if self.history.len() > 64 {
            self.history.remove(0);
        }
    }
}

#[derive(Clone, Copy)]
struct Riddle {
    question: &'static str,
    clue: &'static str,
    answers: [&'static str; 4],
    answer: usize,
}

const RIDDLES: [Riddle; 5] = [
    Riddle {
        question: "I have keys but no locks. What am I?",
        clue: "A cabinet clue about a familiar object",
        answers: ["A piano", "A river", "A candle", "A shadow"],
        answer: 0,
    },
    Riddle {
        question: "What gets wetter as it dries?",
        clue: "Think about a quiet room's linen",
        answers: ["A towel", "A cloud", "A sponge", "A brush"],
        answer: 0,
    },
    Riddle {
        question: "I speak without a mouth. What am I?",
        clue: "The answer returns what you send",
        answers: ["An echo", "A book", "A bell", "A map"],
        answer: 0,
    },
    Riddle {
        question: "What has a face and two hands?",
        clue: "It keeps the cabinet's calm pace",
        answers: ["A clock", "A chair", "A coin", "A door"],
        answer: 0,
    },
    Riddle {
        question: "What can travel around the world while staying put?",
        clue: "Look for it on an old envelope",
        answers: ["A stamp", "A suitcase", "A compass", "A postcard"],
        answer: 0,
    },
];

fn pseudo(seed: u64, index: usize) -> u64 {
    seed.wrapping_add(index as u64)
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
        ^ seed.rotate_left((index % 63) as u32 + 1)
}

#[cfg(test)]
mod tests;
