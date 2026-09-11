//! Shared touch actions emitted by every cabinet screen.

use crate::domain::Direction;

#[derive(Debug, Clone, Copy)]
pub enum UiAction {
    Open(usize),
    ContinueGame,
    ToggleFavorite(usize),
    ClearRecent,
    Cabinet,
    Help,
    Records,
    Statistics,
    Tutorials,
    OpenTutorial(usize),
    ToggleTutorialFilter,
    Favorites,
    Recent,
    DailyArchive,
    DailyArchiveScroll(i8),
    DailyArchiveOpen(u64),
    RecordsFilter(u8),
    RulesFilter(u8),
    CabinetFilter(u8),
    CabinetSort,
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
    CycleGameVariant,
    ConfirmRestart,
    Cancel,
    ToggleSound,
    CycleSoundVolume,
    ResumeLifecycle,
    DismissSaveRecovery,
    ToggleNoticeLog,
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
    LightsOutGuide,
    LightsOutDifficulty(crate::lights_out::LightsDifficulty),
    TicTacToePress(usize),
    TicTacToeHint,
    TicTacToeUndo,
    TicTacToeNew,
    TicTacToeLevel(crate::tic_tac_toe::AiLevel),
    MemoryPairsSelect(usize),
    MemoryPairsHint,
    MemoryPairsUndo,
    MemoryPairsNew,
    MemoryPairsPeek,
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
    HangmanReveal,
    HangmanUndo,
    HangmanCategory(crate::hangman::HangmanCategory),
    HangmanRule(crate::hangman::HangmanRule),
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
    SpaceInvadersStep(crate::space_invaders::ShipDirection),
    SpaceInvadersFire,
    SpaceInvadersPause,
    SpaceInvadersUndo,
    SpaceInvadersNew,
    AsteroidsStep(crate::asteroids::ShipDirection),
    AsteroidsFire,
    AsteroidsPause,
    AsteroidsUndo,
    AsteroidsNew,
    FroggerMove(crate::domain::Direction),
    FroggerPause,
    FroggerUndo,
    FroggerNew,
    MunchMove(crate::domain::Direction),
    MunchPause,
    MunchUndo,
    MunchNew,
    BlockMove(crate::block_stack::BlockMove),
    BlockPause,
    BlockUndo,
    BlockNew,
    CannonAngle(i16),
    CannonPower(i16),
    CannonFire,
    CannonPause,
    CannonUndo,
    CannonNew,
    FlingAngle(i16),
    FlingPower(i16),
    FlingFire,
    FlingPause,
    FlingUndo,
    FlingNew,
    FlingNext,
    FlingRestart,
    PaddleMove(crate::paddle_duel::PaddleMove),
    PaddlePause,
    PaddleUndo,
    PaddleNew,
    BreakoutStep(crate::breakout::PaddleMove),
    BreakoutPause,
    BreakoutHint,
    BreakoutUndo,
    BreakoutNew,
    HigherLowerGuess(crate::higher_lower::Guess),
    HigherLowerHint,
    HigherLowerUndo,
    HigherLowerNew,
    HigherLowerCashOut,
    HigherLowerRule(crate::higher_lower::HigherLowerRule),
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
    SpiderSolitaireDeal,
    SpiderSolitaireHint,
    SpiderSolitaireUndo,
    SpiderSolitaireNew,
    PyramidTap(usize),
    PyramidStock,
    PyramidHint,
    PyramidUndo,
    PyramidNew,
    PyramidDrawRule(crate::pyramid::PyramidDraw),
    TriPeaksTap(usize),
    TriPeaksStock,
    TriPeaksHint,
    TriPeaksUndo,
    TriPeaksNew,
    TriPeaksRule(crate::tri_peaks::TriPeaksRule),
    TriPeaksBridge,
    NimSelect(usize),
    NimTake(u8),
    NimHint,
    NimUndo,
    NimNew,
    NimRule(crate::nim::NimRule),
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
    NumberMatchRemix,
    NumberMatchRule(crate::number_match::LinkRule),
    FloodColor(u8),
    FloodHint,
    FloodUndo,
    FloodNew,
    FloodSurge,
    FloodDifficulty(crate::flood_it::FloodDifficulty),
    ColorSortTap(usize),
    ColorSortHint,
    ColorSortUndo,
    ColorSortNew,
    ColorSortDifficulty(crate::color_sort::ColorSortDifficulty),
    BattleshipFire(usize),
    BattleshipSonar,
    BattleshipHint,
    BattleshipUndo,
    BattleshipNew,
    WordGridLetter(u8),
    WordGridBackspace,
    WordGridSubmit,
    WordGridHint,
    WordGridUndo,
    WordGridNew,
    WordGridMode(crate::word_grid::WordGridMode),
    WordLadderLetter(u8),
    WordLadderBackspace,
    WordLadderSubmit,
    WordLadderHint,
    WordLadderUndo,
    WordLadderNew,
    WordLadderMode(crate::word_ladder::LadderMode),
    PipeRotate(usize),
    PipeHint,
    PipeUndo,
    PipeNew,
    PipePattern(crate::pipe_loop::PipePattern),
    MazeStep(crate::domain::Direction),
    MazeHint,
    MazeUndo,
    MazeNew,
    MazeMode(crate::maze_walk::MazeMode),
    MatchThreeTap(usize),
    MatchThreeHint,
    MatchThreeUndo,
    MatchThreeNew,
    MatchThreeDifficulty(crate::match_three::MatchThreeDifficulty),
    MiscTap(usize),
    MiscSubmit,
    MiscClear,
    MiscHint,
    MiscUndo,
    MiscNew,
}

impl UiAction {
    pub fn starts_new_round(self) -> bool {
        matches!(
            self,
            Self::Restart
                | Self::CycleGameVariant
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
                | Self::LightsOutDifficulty(_)
                | Self::TicTacToeNew
                | Self::MemoryPairsNew
                | Self::SlidingPuzzleNew
                | Self::MastermindNew
                | Self::SpiderNew
                | Self::WordSearchNew
                | Self::HangmanNew
                | Self::HangmanCategory(_)
                | Self::HangmanRule(_)
                | Self::ConnectFourNew
                | Self::CheckersNew
                | Self::PegSolitaireNew
                | Self::MahjongSolitaireNew
                | Self::SnakeNew
                | Self::SnakeMode(_)
                | Self::SpaceInvadersNew
                | Self::AsteroidsNew
                | Self::FroggerNew
                | Self::MunchNew
                | Self::BlockNew
                | Self::CannonNew
                | Self::FlingNew
                | Self::PaddleNew
                | Self::BreakoutNew
                | Self::HigherLowerNew
                | Self::HigherLowerRule(_)
                | Self::KlondikeGolfNew
                | Self::BlackjackNew
                | Self::SpiderSolitaireNew
                | Self::PyramidNew
                | Self::PyramidDrawRule(_)
                | Self::TriPeaksNew
                | Self::TriPeaksRule(_)
                | Self::NimNew
                | Self::NimRule(_)
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
                | Self::NumberMatchRule(_)
                | Self::FloodNew
                | Self::FloodDifficulty(_)
                | Self::ColorSortNew
                | Self::ColorSortDifficulty(_)
                | Self::BattleshipNew
                | Self::WordGridNew
                | Self::WordGridMode(_)
                | Self::WordLadderNew
                | Self::WordLadderMode(_)
                | Self::PipeNew
                | Self::PipePattern(_)
                | Self::MazeNew
                | Self::MazeMode(_)
                | Self::MatchThreeNew
                | Self::MatchThreeDifficulty(_)
                | Self::MiscNew
        )
    }
}
