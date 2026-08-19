use crate::ui::UiAction;

pub(super) fn requires_new_confirmation(action: UiAction) -> bool {
    matches!(
        action,
        UiAction::Game2048Size(_)
            | UiAction::SolitaireNew
            | UiAction::MineRestart
            | UiAction::MinePreset(_)
            | UiAction::SudokuDifficulty(_)
            | UiAction::NonogramPreset(_)
            | UiAction::FreeCellNew
            | UiAction::FivefoldNew
            | UiAction::ReversiNew
            | UiAction::LightsOutNew
            | UiAction::TicTacToeNew
            | UiAction::MemoryPairsNew
            | UiAction::SlidingPuzzleNew
            | UiAction::MastermindNew
            | UiAction::SpiderNew
            | UiAction::WordSearchNew
            | UiAction::HangmanNew
            | UiAction::ConnectFourNew
            | UiAction::CheckersNew
            | UiAction::PegSolitaireNew
            | UiAction::MahjongSolitaireNew
            | UiAction::SnakeNew
            | UiAction::SnakeMode(_)
            | UiAction::BreakoutNew
            | UiAction::HigherLowerNew
            | UiAction::KlondikeGolfNew
            | UiAction::BlackjackNew
            | UiAction::SpiderSolitaireNew
            | UiAction::PyramidNew
            | UiAction::TriPeaksNew
            | UiAction::NimNew
            | UiAction::DungeonNew
            | UiAction::DungeonDifficulty(_)
            | UiAction::PotionNew
            | UiAction::PotionDifficulty(_)
            | UiAction::TowerNew
            | UiAction::RogueNew
            | UiAction::RogueClass(_)
            | UiAction::DailyNew
            | UiAction::DotsNew
            | UiAction::DotsDifficulty(_)
            | UiAction::SokobanRestart
            | UiAction::SokobanNew
            | UiAction::MancalaNew
            | UiAction::MancalaVariant(_)
            | UiAction::HanoiNew
            | UiAction::HanoiDisks(_)
            | UiAction::NumberMatchNew
            | UiAction::NumberMatchRule(_)
            | UiAction::FloodNew
            | UiAction::FloodDifficulty(_)
            | UiAction::ColorSortNew
            | UiAction::ColorSortDifficulty(_)
            | UiAction::BattleshipNew
            | UiAction::WordGridNew
            | UiAction::PipeNew
            | UiAction::MazeNew
            | UiAction::MatchThreeNew
            | UiAction::MatchThreeDifficulty(_)
            | UiAction::WordLadderNew
    )
}
