//! Shared touch actions emitted by every cabinet screen.

use crate::state::Direction;

#[derive(Debug, Clone, Copy)]
pub enum UiAction {
    Open(usize),
    ContinueGame,
    ToggleFavorite(usize),
    Cabinet,
    Help,
    Records,
    Favorites,
    Recent,
    CabinetFilter(u8),
    CabinetScroll(i8),
    LibraryScroll(i8),
    Achievements,
    AchievementFilter(u8),
    Rules,
    Credits,
    ResetData,
    ConfirmResetData,
    CancelResetData,
    TutorialContinue,
    ReplayTutorial,
    Settings,
    Save,
    Load,
    Move(Direction),
    Game2048Hint,
    Game2048Size(crate::game_2048::Game2048Size),
    Undo,
    Restart,
    ConfirmRestart,
    Cancel,
    ToggleSound,
    ToggleMotion,
    ToggleHighContrast,
    ToggleLargeText,
    CycleCardBack,
    CycleBoardTheme,
    CycleSoundSet,
    CycleCabinetDecoration,
    MineReveal(usize),
    MineFlag(usize),
    MineChord(usize),
    MineRestart,
    MineFlagMode,
    MineHint,
    MinePreset(crate::minesweeper::MinePreset),
    SudokuCell(usize),
    SudokuNumber(u8),
    SudokuErase,
    SudokuNoteMode,
    SudokuDifficulty(crate::sudoku::SudokuDifficulty),
    SudokuUndo,
    SudokuHint,
    NonogramCell(usize),
    NonogramMode,
    NonogramUndo,
    NonogramPreset(crate::nonogram::NonogramPreset),
    NonogramZoom,
    NonogramPan(i8, i8),
    NonogramHint,
    SolitaireStock,
    SolitaireTableau(usize, usize),
    SolitaireWaste,
    SolitaireFoundation(usize),
    SolitaireUndo,
    SolitaireHint,
    SolitaireNew,
    FreeCellCell(usize),
    FreeCellCascade(usize, usize),
    FreeCellFoundation(usize),
    FreeCellUndo,
    FreeCellHint,
    FreeCellNew,
    FivefoldRoll,
    FivefoldHold(usize),
    FivefoldCategory(crate::fivefold::Category),
    FivefoldScorePage(i8),
    FivefoldHint,
    FivefoldNew,
    ReversiPlace(usize),
    ReversiHint,
    ReversiPass,
    ReversiNew,
    ReversiLevel(crate::reversi::AiLevel),
    LightsOutPress(usize),
    LightsOutHint,
    LightsOutUndo,
    LightsOutNew,
    TicTacToePress(usize),
    TicTacToeHint,
    TicTacToeUndo,
    TicTacToeNew,
    TicTacToeLevel(crate::tic_tac_toe::AiLevel),
    MemoryPairsSelect(usize),
    MemoryPairsHint,
    MemoryPairsUndo,
    MemoryPairsNew,
    SlidingPuzzleMove(usize),
    SlidingPuzzleHint,
    SlidingPuzzleUndo,
    SlidingPuzzleNew,
    MastermindPick(u8),
    MastermindHint,
    MastermindSubmit,
    MastermindClear,
    MastermindUndo,
    MastermindNew,
    SpiderSelect(usize, usize),
    SpiderMove(usize),
    SpiderDeal,
    SpiderHint,
    SpiderUndo,
    SpiderNew,
    WordSearchCell(usize),
    WordSearchClear,
    WordSearchNew,
    WordSearchHint,
    HangmanGuess(u8),
    HangmanHint,
    HangmanNew,
    ConnectFourDrop(usize),
    ConnectFourHint,
    ConnectFourUndo,
    ConnectFourNew,
    ConnectFourLevel(crate::connect_four::AiLevel),
    CheckersTap(usize),
    CheckersHint,
    CheckersUndo,
    CheckersNew,
    CheckersLevel(crate::checkers::AiLevel),
    PegSolitaireTap(usize),
    PegSolitaireHint,
    PegSolitaireUndo,
    PegSolitaireNew,
    MahjongSolitaireTap(usize),
    MahjongSolitaireHint,
    MahjongSolitaireUndo,
    MahjongSolitaireNew,
    SnakeStep(crate::snake::SnakeDirection),
    SnakeMode(crate::snake::SnakeMode),
    SnakePause,
    SnakeHint,
    SnakeUndo,
    SnakeNew,
    BreakoutStep(crate::breakout::PaddleMove),
    BreakoutPause,
    BreakoutHint,
    BreakoutUndo,
    BreakoutNew,
    HigherLowerGuess(crate::higher_lower::Guess),
    HigherLowerHint,
    HigherLowerUndo,
    HigherLowerNew,
    KlondikeGolfColumn(usize),
    KlondikeGolfStock,
    KlondikeGolfHint,
    KlondikeGolfUndo,
    KlondikeGolfNew,
    BlackjackHit,
    BlackjackStand,
    BlackjackHint,
    BlackjackUndo,
    BlackjackNew,
    SpiderSolitaireSelect(usize, usize),
    SpiderSolitaireMove(usize),
    SpiderSolitaireDeal,
    SpiderSolitaireHint,
    SpiderSolitaireUndo,
    SpiderSolitaireNew,
    PyramidTap(usize),
    PyramidStock,
    PyramidHint,
    PyramidUndo,
    PyramidNew,
    TriPeaksTap(usize),
    TriPeaksStock,
    TriPeaksHint,
    TriPeaksUndo,
    TriPeaksNew,
    NimSelect(usize),
    NimTake(u8),
    NimHint,
    NimUndo,
    NimNew,
    DungeonCell(usize),
    DungeonToggleFlag,
    DungeonHint,
    DungeonUndo,
    DungeonNew,
    DungeonDifficulty(crate::dungeon_sweeper::DungeonDifficulty),
    PotionMove(Direction),
    PotionHint,
    PotionUndo,
    PotionNew,
    PotionDifficulty(crate::potion_2048::PotionDifficulty),
    TowerCell(usize),
    TowerSelectKind(crate::tiny_tower_defence::TowerKind),
    TowerWave,
    TowerHint,
    TowerUndo,
    TowerNew,
    RogueMove(Direction),
    RogueStrike,
    RoguePotion,
    RogueHint,
    RogueUndo,
    RogueNew,
    RogueClass(crate::one_room_roguelike::HeroClass),
    DailyMove(Direction),
    DailyScout,
    DailyHint,
    DailyUndo,
    DailyNew,
    DotsEdge(crate::dots_boxes::Edge),
    DotsHint,
    DotsUndo,
    DotsNew,
    DotsDifficulty(crate::dots_boxes::DotsDifficulty),
    SokobanMove(Direction),
    SokobanHint,
    SokobanUndo,
    SokobanRestart,
    SokobanNew,
    MancalaPit(usize),
    MancalaHint,
    MancalaUndo,
    MancalaNew,
    MancalaLevel(crate::mancala::AiLevel),
    MancalaVariant(crate::mancala::MancalaVariant),
    HanoiPeg(usize),
    HanoiHint,
    HanoiUndo,
    HanoiNew,
    HanoiDisks(u8),
    NumberMatchTap(usize),
    NumberMatchHint,
    NumberMatchUndo,
    NumberMatchNew,
    FloodColor(u8),
    FloodHint,
    FloodUndo,
    FloodNew,
    FloodDifficulty(crate::flood_it::FloodDifficulty),
    ColorSortTap(usize),
    ColorSortHint,
    ColorSortUndo,
    ColorSortNew,
    ColorSortDifficulty(crate::color_sort::ColorSortDifficulty),
    BattleshipFire(usize),
    BattleshipHint,
    BattleshipUndo,
    BattleshipNew,
    WordGridLetter(u8),
    WordGridBackspace,
    WordGridSubmit,
    WordGridHint,
    WordGridUndo,
    WordGridNew,
    WordLadderLetter(u8),
    WordLadderBackspace,
    WordLadderSubmit,
    WordLadderHint,
    WordLadderUndo,
    WordLadderNew,
    PipeRotate(usize),
    PipeHint,
    PipeUndo,
    PipeNew,
    MazeStep(crate::state::Direction),
    MazeHint,
    MazeUndo,
    MazeNew,
    MatchThreeTap(usize),
    MatchThreeHint,
    MatchThreeUndo,
    MatchThreeNew,
    MatchThreeDifficulty(crate::match_three::MatchThreeDifficulty),
}

impl UiAction {
    pub fn starts_new_round(self) -> bool {
        matches!(
            self,
            Self::Open(_)
                | Self::ContinueGame
                | Self::Restart
                | Self::ConfirmRestart
                | Self::Game2048Size(_)
                | Self::MineRestart
                | Self::MinePreset(_)
                | Self::SudokuDifficulty(_)
                | Self::NonogramPreset(_)
                | Self::SolitaireNew
                | Self::FreeCellNew
                | Self::FivefoldNew
                | Self::ReversiNew
                | Self::LightsOutNew
                | Self::TicTacToeNew
                | Self::MemoryPairsNew
                | Self::SlidingPuzzleNew
                | Self::MastermindNew
                | Self::SpiderNew
                | Self::WordSearchNew
                | Self::HangmanNew
                | Self::ConnectFourNew
                | Self::CheckersNew
                | Self::PegSolitaireNew
                | Self::MahjongSolitaireNew
                | Self::SnakeNew
                | Self::SnakeMode(_)
                | Self::BreakoutNew
                | Self::HigherLowerNew
                | Self::KlondikeGolfNew
                | Self::BlackjackNew
                | Self::SpiderSolitaireNew
                | Self::PyramidNew
                | Self::TriPeaksNew
                | Self::NimNew
                | Self::DungeonNew
                | Self::DungeonDifficulty(_)
                | Self::PotionNew
                | Self::PotionDifficulty(_)
                | Self::TowerNew
                | Self::RogueNew
                | Self::RogueClass(_)
                | Self::DailyNew
                | Self::DotsNew
                | Self::DotsDifficulty(_)
                | Self::SokobanRestart
                | Self::SokobanNew
                | Self::MancalaNew
                | Self::MancalaVariant(_)
                | Self::HanoiNew
                | Self::HanoiDisks(_)
                | Self::NumberMatchNew
                | Self::FloodNew
                | Self::FloodDifficulty(_)
                | Self::ColorSortNew
                | Self::ColorSortDifficulty(_)
                | Self::BattleshipNew
                | Self::WordGridNew
                | Self::WordLadderNew
                | Self::PipeNew
                | Self::MazeNew
                | Self::MatchThreeNew
                | Self::MatchThreeDifficulty(_)
        )
    }
}
