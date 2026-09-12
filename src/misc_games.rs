//! Five small deterministic puzzle games for the Misc cabinet shelf.

use serde::{Deserialize, Serialize};

mod words;

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
    pub round: u32,
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
        if self.selected_sum() == self.target {
            self.score = self.score.saturating_add(self.selected.len() as u16);
            self.round = self.round.saturating_add(1);
            self.prepare();
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
            self.prepare();
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
        use macroquad_toolkit::rng::SeededRng;
        let mut rng = SeededRng::new(pseudo(self.seed, self.round as usize));
        let start = 2 + rng.below(10) as i32;
        let gap = 2 + rng.below(5) as i32;
        let family = rng.below(6);
        let mut values = vec![start];
        for step in 1..6 {
            let previous = values[step - 1];
            values.push(match family {
                0 => previous + gap * step as i32,
                1 => previous * 2 + gap,
                2 => previous + if step % 2 == 1 { gap } else { gap * 3 },
                3 if step > 1 => previous + values[step - 2],
                3 => start + gap,
                4 => start + gap * (step * step) as i32,
                _ => previous + (step * step) as i32 + gap,
            });
        }
        let missing = values[5];
        self.prompt = format!(
            "{} · ?",
            values[..5]
                .iter()
                .map(i32::to_string)
                .collect::<Vec<_>>()
                .join(" · ")
        );
        self.detail = "Find the rule across all five values".into();
        let mut options = vec![missing, missing + gap, missing - gap, missing + gap * 2];
        rng.shuffle(&mut options);
        // The generated answer is always included in the option list.
        self.answer = options.iter().position(|value| *value == missing).unwrap();
        self.options = options.iter().map(i32::to_string).collect();
    }

    fn prepare_sum(&mut self) {
        self.prompt = "CONNECT THE TARGET".into();
        self.detail = "Tap any tiles to total the target; tap SUBMIT".into();
        let mut rng =
            macroquad_toolkit::rng::SeededRng::new(pseudo(self.seed, self.round as usize));
        self.board = (0..12).map(|_| 1 + rng.below(9) as u8).collect();
        let mut indices: Vec<usize> = (0..self.board.len()).collect();
        rng.shuffle(&mut indices);
        self.solution = indices[..3].to_vec();
        self.target = self
            .solution
            .iter()
            .map(|index| u16::from(self.board[*index]))
            .sum();
    }

    fn prepare_orbit(&mut self) {
        self.prompt = "RESTORE THE ORBIT".into();
        self.detail = "Put the numbered planets in ascending order".into();
        let mut rng = macroquad_toolkit::rng::SeededRng::new(self.seed);
        self.board = (1..=8).collect();
        loop {
            rng.shuffle(&mut self.board);
            // Count permutation cycles: n - cycles is the minimum swap count.
            let mut visited = [false; 8];
            let mut cycles = 0;
            for start in 0..8 {
                if !visited[start] {
                    cycles += 1;
                    let mut index = start;
                    while !visited[index] {
                        visited[index] = true;
                        index = usize::from(self.board[index] - 1);
                    }
                }
            }
            if 8 - cycles >= 5 {
                break;
            }
        }
    }

    fn prepare_word(&mut self) {
        let mut rng =
            macroquad_toolkit::rng::SeededRng::new(pseudo(self.seed, self.round as usize + 9));
        let mut index = rng.below(words::WORDS.len());
        if words::WORDS[index].0 == self.target_word {
            index = (index + 1) % words::WORDS.len();
        }
        let (word, clue) = words::WORDS[index];
        self.target_word = word.into();
        self.prompt = "FORGE THE WORD".into();
        self.detail = clue.into();
        self.board = self.target_word.bytes().collect();
        while self.board == self.target_word.as_bytes() {
            rng.shuffle(&mut self.board);
        }
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
