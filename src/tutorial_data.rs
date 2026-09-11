//! Touch-first instruction copy shared by desktop and responsive tutorials.

use crate::state::GameId;

pub(crate) fn instructions(game: GameId) -> [&'static str; 3] {
    match game {
        GameId::Game2048 => [
            "Swipe the board to move tiles.",
            "You can also tap a visible direction arrow.",
            "Tap TUTORIAL above to see this again.",
        ],
        GameId::Minesweeper => [
            "Tap a hidden square to reveal it.",
            "Tap REVEAL MODE / FLAG MODE to mark safely.",
            "Tap a revealed number to use the visible chord action.",
        ],
        GameId::Sudoku => [
            "Tap a cell to select it.",
            "Tap a number on the visible number pad to place it.",
            "Tap ERASE or PENCIL when you need another mark.",
        ],
        GameId::Nonogram => [
            "Tap a cell to fill or cross it.",
            "Tap FILL / CROSS to change the visible mode.",
            "Drag across a row or column for a straight stroke.",
        ],
        GameId::Solitaire => [
            "Tap a face-up card to select it.",
            "Tap a legal tableau or foundation destination.",
            "Tap STOCK to deal; use the visible UNDO button.",
        ],
        GameId::FreeCell => [
            "Tap a card to select it.",
            "Tap a cascade, free cell, or foundation destination.",
            "Every card stays visible while you build sequences.",
        ],
        GameId::Yahtzee => [
            "Tap ROLL DICE for the first roll.",
            "Tap dice to hold them, then tap ROLL AGAIN.",
            "Tap a score row to record the visible preview.",
        ],
        GameId::Reversi => [
            "Tap a glowing square to place a disc.",
            "The captured line flips visibly after your move.",
            "Tap PASS only when no legal square remains.",
        ],
        GameId::LightsOut => [
            "Tap a light to toggle its cross. Turn every light off; MOVES and PAR track efficiency.",
            "Tap GUIDE to outline every switch in a current shortest route, or HINT for its exact first press.",
            "Tap CLASSIC 12 or DENSE 20 for a new scramble. Use repeated UNDO or NEW BOARD as needed.",
        ],
        GameId::TicTacToe => [
            "Tap an empty square to place your X.",
            "The cabinet answers with O after your move.",
            "Use UNDO or NEW BOARD when you need it.",
        ],
        GameId::MemoryPairs => [
            "Tap a card to turn it up. A corner dot marks a face-down card you have already seen.",
            "Consecutive pairs build score; a mismatch costs five and breaks the chain.",
            "Tap PEEK once to study two cards until your next tap. HINT uses only remembered cards; UNDO is repeatable.",
        ],
        GameId::SlidingPuzzle => [
            "Tap a tile beside the empty space.",
            "Put every numbered tile back in order.",
            "Use UNDO or NEW BOARD whenever you need it.",
        ],
        GameId::Mastermind => [
            "Tap four colors to build a guess.",
            "GUESS shows exact and partial matches.",
            "Use CLEAR, UNDO, or NEW BOARD visibly below.",
        ],
        GameId::Spider => [
            "Tap a face-up descending run to select it.",
            "Tap a destination column to move the run.",
            "Tap STOCK to deal one card to every column.",
        ],
        GameId::WordSearch => [
            "Tap the first letter of a hidden word.",
            "Tap its final letter in a straight line.",
            "Use CLEAR or NEW BOARD with the visible controls.",
        ],
        GameId::Hangman => [
            "Tap a visible letter button. Correct guesses build a score chain; a wrong guess breaks it.",
            "Tap CABINET, NATURE, or VOYAGE for a new word set. RAPID allows four errors and doubles points.",
            "Tap REVEAL for one free letter, or use HINT, UNDO, and NEW WORD with the visible controls.",
        ],
        GameId::ConnectFour => [
            "Tap a numbered column to drop your red disc.",
            "The cabinet answers with a yellow disc.",
            "Make four in a row; use UNDO or NEW BOARD visibly.",
        ],
        GameId::Checkers => [
            "Tap a red piece, then tap a diagonal destination.",
            "Captures are mandatory; continue tapping for a chained jump.",
            "Reach the far edge to crown a king; use UNDO or NEW BOARD.",
        ],
        GameId::PegSolitaire => [
            "Tap a peg, then tap a two-step destination over a neighbor.",
            "Each jump removes the middle peg; keep clearing the board.",
            "Leave one peg in the center; use UNDO or NEW BOARD.",
        ],
        GameId::MahjongSolitaire => [
            "Tap a free tile, then tap its matching free partner.",
            "A tile is free when one side is open and no tile covers it.",
            "Clear every pair; use UNDO or NEW BOARD with the visible controls.",
        ],
        GameId::Snake => [
            "Tap CLASSIC, WRAP, or GARDEN, then use the visible direction buttons to steer.",
            "WRAP crosses edges; GARDEN adds rocks. Gold food is worth three and the pace rises.",
            "Reach 20; tap PAUSE, UNDO, or NEW BOARD with the visible controls.",
        ],
        GameId::Breakout => [
            "Tap LEFT, STAY, or RIGHT to steer while the ball moves automatically.",
            "Clear three patterned walls. Armored bricks need two or three hits.",
            "You have three balls. Tap LAUNCH after a miss, or PAUSE, UNDO, and NEW BOARD as needed.",
        ],
        GameId::HigherLower => [
            "Tap HIGHER or LOWER; each button shows its exact chance and green marks the safer side.",
            "Correct cards grow the pot. After two, tap CASH OUT to bank it, or reach ten for a perfect run.",
            "FRIENDLY wins ties. HOUSE loses ties but pays double. HINT, UNDO, and NEW ROUND stay visible.",
        ],
        GameId::KlondikeGolf => [
            "Tap a top card one rank above or below the waste.",
            "Tap STOCK when no column can play; clear every column to win.",
            "Use UNDO or NEW BOARD with the visible controls.",
        ],
        GameId::Blackjack => [
            "Tap HIT to take another card or STAND to hold.",
            "The dealer draws to seventeen; stay at or below twenty-one.",
            "Use UNDO or NEW ROUND with the visible controls.",
        ],
        GameId::SpiderSolitaire => [
            "Tap a same-suit descending run to select it.",
            "Tap a destination column; deal STOCK when every column is filled.",
            "Clear eight suited runs; use UNDO or NEW DEAL visibly.",
        ],
        GameId::DungeonSweeper => [
            "Tap EXPLORER, DELVER, or PERIL to choose hearts, traps, and required relics.",
            "Reveal rooms for clues; K marks a relic. FLAG MODE and clue taps safely chord rooms.",
            "Collect every K to unlock EXIT; HINT, UNDO, and NEW DUNGEON stay visible.",
        ],
        GameId::Potion2048 => [
            "Tap a visible direction to slide the potions.",
            "Consecutive merging moves multiply score and fill the visible Chain counter.",
            "Complete the Chain to brew a C catalyst that merges with any tier; UNDO and NEW BREW stay visible.",
        ],
        GameId::TinyTowerDefence => [
            "Tap BOLT, FROST, or BURST, then tap an empty lane cell to build.",
            "Bolt hits hard, Frost slows, and Burst splashes nearby lanes; tap a tower to upgrade it.",
            "Tap START WAVE and survive wave 8. PAUSE, UNDO, and NEW TOWER remain visible.",
        ],
        GameId::OneRoomRoguelike => [
            "Tap BLADE, WARDEN, or ALCHEMIST to begin with a different hero talent.",
            "Tap directions to explore; defeat G guards, moving S stalkers, and tough B brutes.",
            "Clear five rooms and tap onto STAIRS; POTION, UNDO, and NEW RUN stay visible.",
        ],
        GameId::DailyDungeon => [
            "Each day is WAYFINDER, FORAGER, or DAREDEVIL with a different trap and SCOUT supply.",
            "Tap SCOUT to reveal adjacent rooms; entering one unseen earns extra bravery score.",
            "Find three runes, use + springs, and reach EXIT; HINT, UNDO, and REPLAY DAY stay visible.",
        ],
        GameId::DotsBoxes => [
            "Tap a gap between two dots to draw one edge.",
            "Complete a square to keep your turn; a circled ! warns that one edge remains.",
            "HARD avoids gifts and EXPERT minimizes forced chains; use HINT, UNDO, or NEW BOARD.",
        ],
        GameId::Sokoban => [
            "Tap UP, LEFT, DOWN, or RIGHT to walk and push; crates cannot be pulled.",
            "A red X marks a cornered crate; tap UNDO or RESTART to recover.",
            "Beat PAR across six rooms, then tap NEXT ROOM after each clear.",
        ],
        GameId::Mancala => [
            "Tap QUICK, CLASSIC, or GRAND for three, four, or five opening stones per pit.",
            "Tap one of your six pits: E earns another turn and C previews a capture.",
            "GENTLE, SHARP, and EXPERT change cabinet tactics; gather the majority.",
        ],
        GameId::Hanoi => [
            "Tap 3 DISKS, 5 DISKS, or 7 DISKS to choose the tower and its par.",
            "Tap a source peg; green rings are legal destinations and red rings are blocked.",
            "Move the numbered tower to the far peg; use HINT, multi-step UNDO, or RESTART.",
        ],
        GameId::NumberMatch => [
            "Tap NEAR, LINES, or DIAGONAL to choose which clear paths may connect a pair.",
            "Tap equal numbers or numbers totaling ten; green outlines show valid links.",
            "Chain pairs for points; if NO LINKS appears, tap multi-step UNDO or REMIX.",
        ],
        GameId::FloodIt => [
            "Tap a color; its +number forecasts how many cells join the outlined region.",
            "Chain growing moves for points; three strong gains earn a free SURGE.",
            "Fill the field before the limit; tap SURGE, HINT, multi-step UNDO, or NEW FIELD.",
        ],
        GameId::ColorSort => [
            "Tap an unsealed tube; green tubes preview where its full top run can pour.",
            "Letters identify colors; matching pours build a chain and full tubes seal.",
            "Seal every color tube; use HINT, multi-step UNDO, or NEW BOARD.",
        ],
        GameId::Battleship => [
            "Tap unknown water to fire, or tap SONAR then a cell to sweep its 3 × 3 area.",
            "SONAR marks contacts ! and clear water ~; sunk vessels change to gold S marks.",
            "Sink all three ships; chain hits for points and use multi-step UNDO or NEW FLEET.",
        ],
        GameId::WordGrid => [
            "Tap letters to build a five-letter guess, then tap SUBMIT.",
            "Tiles use = for exact, ? for present, and X for absent; the header counts candidates.",
            "HARD binds every clue; use HINT, BACKSPACE, multi-step UNDO, or NEW WORD.",
        ],
        GameId::PipeLoop => [
            "Tap a pipe tile to rotate it clockwise.",
            "Bright pipes are powered; red endpoint caps mark leaks that must be closed.",
            "Power all 25 in SERPENT or TRUNK; use HINT, multi-step UNDO, or NEW LOOP.",
        ],
        GameId::MazeWalk => [
            "Tap a bright visible direction control; dim controls are blocked by walls.",
            "Collect both B beacons before entering the E exit; L means it is still locked.",
            "Try EXPLORE or FOG, and use HINT, multi-step UNDO, or NEW MAZE.",
        ],
        GameId::MatchThree => [
            "Tap two adjacent tiles to swap their positions.",
            "Match four for a line arrow; match five or a cross for a burst tile.",
            "Reach the target before moves run out. Tap HINT, UNDO, or NEW BOARD as needed.",
        ],
        GameId::Pyramid => [
            "Tap an outlined king, or select a card to outline exposed partners totaling 13.",
            "Chain clears for points; STOCK breaks the chain and DRAW 3 reveals three cards.",
            "Clear all 28 cards; empty stock can RECYCLE once. Use HINT, UNDO, or NEW PYRAMID.",
        ],
        GameId::TriPeaks => [
            "Tap a green outlined card one rank above or below the waste; A↔K WRAP also links ace and king.",
            "Chain tableau clears for rising points. Tap STOCK for a new waste card, but it breaks the run.",
            "Tap BRIDGE, then any exposed card, once per deal. Clear all peaks; HINT and UNDO remain visible.",
        ],
        GameId::Nim => [
            "Tap a non-empty heap to select it.",
            "Tap TAKE 1–3; SAFE marks a forced win and RISK means the cabinet can answer.",
            "Take last in NORMAL or avoid last in MISERE; use HINT, multi-step UNDO, or NEW BOARD.",
        ],
        GameId::WordLadder => [
            "Tap letters to build a five-letter step, then tap SUBMIT.",
            "Change exactly one letter; the outlined tile shows which letter changed.",
            "Climb DIRECT or visit the SCENIC waypoint; use HINT, multi-step UNDO, or NEW LADDER.",
        ],
        GameId::SpaceInvaders => [
            "Tap LEFT or RIGHT to line up your ship with the descending fleet.",
            "Tap FIRE to launch a shot; PAUSE stops the moving formation.",
            "Clear three waves, or use UNDO and NEW WAVE with the visible controls.",
        ],
        GameId::Asteroids => [
            "Tap LEFT or RIGHT to steer around the drifting rocks.",
            "Tap FIRE to split an asteroid before it reaches your ship.",
            "Reach the score target; PAUSE, UNDO, and NEW FIELD stay visible.",
        ],
        GameId::Frogger => [
            "Tap the visible arrow controls to move the frog through traffic.",
            "Cars keep moving while you plan each crossing.",
            "Reach the far bank three times; PAUSE, UNDO, and NEW CROSSING stay visible.",
        ],
        GameId::MunchMaze => [
            "Tap UP, DOWN, LEFT, or RIGHT to guide the runner through the maze.",
            "Collect every glowing pellet while the patrols move in real time.",
            "Use PAUSE, UNDO, or NEW MAZE with the visible controls.",
        ],
        GameId::BlockStack => [
            "Tap LEFT or RIGHT to position the falling block.",
            "Tap ROTATE or DROP; clear twenty lines to finish the stack.",
            "PAUSE, UNDO, and NEW STACK stay visible below the board.",
        ],
        GameId::TerrainCannon => [
            "Tap ANGLE − / + and POWER − / + to set the launch.",
            "Tap FIRE; every impact removes a chunk from the terrain.",
            "Use PAUSE, UNDO, or NEW HILLS with the visible controls.",
        ],
        GameId::FlingFury => [
            "Tap ANGLE − / + and POWER − / + to aim the sling.",
            "Tap FLING to send a physics shot through blocks and targets.",
            "Use PAUSE, UNDO, or NEW FORT with the visible controls.",
        ],
        GameId::PaddleDuel => [
            "Tap UP or DOWN to move your paddle.",
            "Return the ball and score seven points before the cabinet does.",
            "Use PAUSE, UNDO, or NEW MATCH with the visible controls.",
        ],
        GameId::RiddleRoom => [
            "Read the clue and tap one of the four visible answers.",
            "A correct answer opens the next cabinet riddle; HINT names the answer.",
            "Solve five clues. Use UNDO or NEW ROUND whenever you need a fresh start.",
        ],
        GameId::PatternVault => [
            "Read the visible number sequence and tap its next value.",
            "The clue line explains the pattern family; HINT reveals the next value.",
            "Solve five sequences. Use UNDO or NEW ROUND whenever you need recovery.",
        ],
        GameId::SumCircuit => [
            "Tap three tiles whose values add up to the visible target.",
            "The selected total is shown below the puzzle; tap CLEAR to change it.",
            "Complete four targets. Use HINT, UNDO, or NEW ROUND with visible controls.",
        ],
        GameId::OrbitOrder => [
            "Tap two numbered planets to swap their positions.",
            "Restore positions one through five from left to right.",
            "Use HINT, UNDO, or NEW ROUND when a swap needs correcting.",
        ],
        GameId::WordForge => [
            "Tap the scrambled letters in the order that forms a word.",
            "Your current word appears above the letters; tap CLEAR to start it again.",
            "Forge five words. Use HINT, UNDO, or NEW ROUND with visible controls.",
        ],
    }
}

#[cfg(test)]
mod tests;
