# Idle Hands — Game Design Document

Status: Current post-launch product baseline
Date: 2026-08-15

## 1. High Concept

Idle Hands is a personal drawer of tactile tabletop and puzzle games. Opening
the app reveals a beautifully illustrated cabinet whose objects are the game
selector: a deck, booklet, tile tray, dice cup, and other small curios. Sessions
are short, rules are familiar, and the collection never pressures the player to
return.

The primary devices are iPhone and iPad in a browser. Desktop browsers and the
Windows build retain pointer and optional keyboard support, but the complete
experience is designed around taps, holds, and swipes.

The shipped cabinet currently contains 47 playable games. The eight titles in
Section 4 are the launch foundation; the post-launch additions are listed in
Section 11 and share the same persistence, tutorial, records, and responsive
touch contracts.

## 2. Design Pillars

1. **A satisfying object drawer.** Games should feel like physical things taken
   from a familiar cabinet, not unrelated entries in a software grid.
2. **One-thumb legibility.** Common actions are obvious, comfortably sized, and
   reachable; complex gestures always have a visible alternative.
3. **A complete game in minutes.** Every title offers a meaningful session
   without errands, energy systems, or meta-game maintenance.
4. **Quiet reasons to return.** Records, replayable seeds, optional daily
   puzzles, and cosmetic stamps add continuity without obligation.
5. **Rules before decoration.** Art and animation reinforce state; they never
   obscure legal moves, marks, values, or scoring.

## 3. Collection Loop

1. Open the cabinet.
2. Tap an object to open its game drawer.
3. Continue the saved session or start a new one.
4. Play through visible controls and direct board gestures.
5. Finish, abandon, or return to the cabinet; progress is saved automatically.
6. Optionally inspect records, rules, or cosmetic stamps.

The Records screen keeps the collection view legible with a compact summary of
stamps, achievements, and completed drawers out of the full 47-game cabinet.
Every drawer has its own completion achievement, in addition to first-finish
and full-cabinet collection awards. The Records screen opens a responsive
ACHIEVEMENTS shelf so earned and locked awards remain inspectable without
interrupting a game. Drawer awards use the drawer's own title so thematic
achievement names never obscure what was completed. ALL, EARNED, and LOCKED
filters let a player focus on the next collection goal.

Starting a new drawer session presents a clear CANCEL/START confirmation so an
accidental tap cannot replace an unfinished session.

The cabinet remembers the last-opened game and exposes a visible CONTINUE action
alongside Home, Help, Records, and Settings. Players can mark drawers with the
left-edge favorite markers and open optional FAVORITES and RECENT quick lists;
favorites and recent history are profile state and never hide or reorder the
full collection. A first-time player sees a short, game-specific
touch tutorial that can be replayed from Help.

## 4. Launch Games

### 4.1 Solitaire

Klondike with draw-one as the default launch ruleset. Tap a card to select it
and tap a destination to move it; dragging is a faster equivalent. Tapping the
stock deals, tapping a valid exposed card can auto-send it to a foundation, and
Undo/New Deal/Hint are visible controls. The initial release guarantees
rule-valid seeded deals, not necessarily deals proven solvable.

### 4.2 FreeCell

Standard 52-card FreeCell with four free cells and four foundations. It shares
card visuals, drag behavior, move animation, and seeded deck infrastructure
with Solitaire. Legal multi-card movement follows the available-free-cell and
empty-column capacity. Invalid moves return cleanly with a short explanation.

### 4.3 Sudoku

9×9 puzzles with selectable difficulty, pencil marks, undo, erase, and optional
mistake highlighting. A large number pad remains visible or one tap away.
Puzzles must have a unique solution and are generated or selected from seeded,
validated data. The player can disable all correctness assistance.

### 4.4 Minesweeper

Beginner, intermediate, expert, and a responsive custom board. The first reveal
is always safe. Normal tap reveals; marking uses either a long press or an
explicit Reveal/Flag mode, so precise timing is never required. Chording is
available through a visible action after selecting a revealed number.

### 4.5 2048

A standard 4×4 board controlled by horizontal and vertical swipes, with visible
direction buttons as an accessible alternative. One undo is available by
default. A session records score, best tile, and seed. Swipe thresholds scale
with screen density and do not trigger while navigating from the screen edge.

### 4.6 Nonogram

Seeded picture-logic puzzles beginning with 5×5, 10×10, and 15×15 boards. Fill
and Cross are explicit modes. Players may tap cells or drag a straight stroke;
the stroke locks to one row or column and previews before committing. Clues and
cells remain readable through zoom/pan or a focused-board layout on small
phones.

### 4.7 Yahtzee

Solo score chasing over thirteen rounds using the familiar five-dice category
structure. Tap dice to hold them, tap the large Roll control for up to three
rolls, then tap a legal score row. Potential scores are previews rather than
automatic choices. The game tracks personal best totals and category records.
The product uses the descriptive label “five-dice scorecard” where trademark
or storefront naming makes that preferable; exact public naming is a release
check.

### 4.8 Reversi

An 8×8 game against local AI, with same-device two-player as an option. Legal
moves are visibly marked and tapping one previews/flips the captured lines.
Passes happen only when no legal move exists and are explained on screen. AI
ships with at least two difficulty levels and performs within a bounded frame
budget so touch feedback remains immediate.

## 5. Cabinet and Navigation

The home screen is an illustrated cabinet rather than a conventional icon grid.
Each game has a distinct object, silhouette, label, and focus state. Objects
remain recognizable without color. On narrow portrait screens, cabinet shelves
stack vertically and scroll; tablets and landscape screens show the complete
cabinet or a wider arrangement.

Required global destinations are Home, Continue/New Game, Rules, Settings, and
Records. Destructive restarts require a confirmation sheet that names the game
and states that its current session will be replaced. Back navigation never
silently discards play.

## 6. Progression and Retention

The optional meta layer uses one plainly named collectible: stamps. Finishing
games and selected achievements awards stamps once. Stamps unlock only cosmetic
items such as card backs, board materials, sound sets, and small shelf
decorations. Cosmetics never alter rules, puzzle information, odds, hints, or
scores.

Daily seeded games are a post-launch extension. If added, missing a day has no
penalty and there is no escalating streak reward. Public shared seeds require a
stable seed/version scheme so future rule changes do not alter old puzzles.

## 7. Presentation and Audio

The cabinet uses warm wood, paper, brass, felt, ceramic, and ink. Individual
games can emphasize one material while sharing typography, spacing, navigation,
and motion language. Movement should feel tactile: cards settle, dice tumble,
tiles slide, and discs turn, with reduced-motion substitutions available.

Audio is optional feedback, never required information. Master, effects, and
music/ambience levels persist separately and each can be muted. Haptics may be
used only when browser/platform support is safe and every cue also has a visual
equivalent.

## 8. Accessibility and Responsive Requirements

- Support portrait and landscape layouts from 320×568 CSS pixels upward, plus
  common tablet and desktop sizes.
- Respect browser safe areas and keep primary actions away from gesture zones.
- Use 44×44 logical-point minimum targets; prefer 48 points for primary play.
- Never require hover, right-click, multi-touch, keyboard, or a timed long press.
- Provide a visible alternative for swipe, drag, hold, and chord gestures.
- Preserve readable text at user-selectable UI scale without clipping controls.
- Convey selection, legality, errors, and game-over state through more than hue.
- Provide reduced motion, high contrast, sound controls, and tutorial replay.
- Pause simulation when the page loses focus; autosave before/after lifecycle
  transitions where the platform permits.

## 9. Content and Data

Authored settings, themes, achievements, puzzle catalogs, and per-game rule
presets live in JSON. Each game owns typed schemas and semantic validation while
`macroquad_toolkit::data_loader` owns parsing and embedded/runtime loading.

Random deals, boards, dice, and AI tie breaks come from state-owned seeded RNG.
A replay identity includes the game ID, ruleset version, generator version, and
seed. Saves record enough information to resume exactly rather than regenerate
from incomplete context.

## 10. Historical Non-Goals for the First Collection

- Online multiplayer, accounts, leaderboards, cloud saves, or social feeds
- Purchases, ads, stamina, daily chests, streak pressure, or multiple currencies
- Spider Solitaire, Mahjong Solitaire, Checkers, arcade games, word games, and
  original mini-games before the first eight were complete and polished
- A single rigid landscape canvas scaled down into unreadable phone controls
- Perfect-solvability proofs for every generated card deal in the first release

## 11. Post-Launch Cabinet Additions

The current post-launch cabinet includes Lights Out, Tic-Tac-Toe, Memory,
Sliding Puzzle, Mastermind, Spider, Word Search, Hangman, Connect Four,
Checkers, Peg Solitaire, Mahjong Solitaire, Snake, Breakout, Higher or Lower,
Klondike Golf, Blackjack, Spider Solitaire, Dungeon Sweeper, Potion 2048, Tiny
Tower Defence, One Room Roguelike, Daily Dungeon, Dots & Boxes, Sokoban,
Mancala, Hanoi, Number Match, Flood It, Color Sort, Battleship, Word Grid,
Pipe Loop, Maze Walk, Match Three, Pyramid, TriPeaks, Nim, and Word Ladder.

The former candidate list is now represented in the shipped cabinet. Ongoing
post-launch work focuses on polish, accessibility, balance, and additional
cabinet ideas only when they improve the quiet, touch-first collection.

## 12. Historical Phase 0 Decisions

The original eight-game launch scope established the current product principles:

- Web on iPhone/iPad is the primary interaction target.
- Portrait and landscape are both supported; no orientation is mandatory.
- Progression is cosmetic-only.
- Local persistence is sufficient for the current release.
- Rules, rendering, input, and shared profile state remain separate.

The eight-game scope is retained here as historical context; the current
playable scope is the 47-game cabinet listed in Sections 1 and 11.
