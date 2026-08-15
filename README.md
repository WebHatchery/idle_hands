# Idle Hands

Idle Hands is a touch-first collection of small, replayable games for iPhone,
iPad, desktop browsers, and Windows. Its home screen is a warmly illustrated
drawer cabinet: each physical object opens a different game, from a deck of
cards to a dice cup or a little wooden minefield.

The current build is a playable 43-game cabinet with persistent sessions,
records, tutorials, responsive touch layouts, and post-launch refinements.
TODO.md records the completed launch foundation and ongoing collection work.

## Current Collection

The original eight-game foundation remains at the center of the cabinet:

| Game | Cabinet object | Primary touch interaction |
| --- | --- | --- |
| Solitaire | Worn deck of cards | Tap or drag cards |
| FreeCell | Four card trays | Tap or drag cards |
| Sudoku | Pencil and puzzle booklet | Tap a cell, then a number |
| Minesweeper | Brass minefield board | Tap to reveal; use visible flag/chord controls |
| 2048 | Sliding numbered tiles | Swipe or tap visible directions |
| Nonogram | Graph-paper pad | Tap or drag to fill or cross |
| Fivefold | Dice cup and scorecard | Tap dice to hold, then roll and score |
| Reversi | Black-and-white disc box | Tap a legal square |

The post-launch cabinet adds Lights Out, Tic-Tac-Toe, Memory, Sliding Puzzle,
Mastermind, Spider, Word Search, Hangman, Connect Four, Checkers, Peg Solitaire,
Mahjong Solitaire, Snake, Breakout, Higher or Lower, Klondike Golf, Blackjack,
Spider Solitaire, Dungeon Sweeper, Potion 2048, Tiny Tower Defence, One Room
Roguelike, Daily Dungeon, Dots & Boxes, Sokoban, Mancala, Hanoi, Number Match,
Flood It, Color Sort, Battleship, Word Grid, Pipe Loop, Maze Walk, and Match
Three.

## Product Principles

- **Touch is complete, not supplemental.** Every game, tutorial, menu, undo,
  restart, settings, and recovery action has a visible tap target or a direct
  touch gesture. A keyboard is never required.
- **Comfortable on phones and tablets.** Layouts adapt to portrait and
  landscape screens, safe areas, and coarse pointers. Important targets are at
  least 44 logical points and do not depend on hover.
- **Fast to enter and leave.** The cabinet restores each game's unfinished
  session locally. A player can reach a game in one tap and return home without
  losing progress.
- **Calm progression.** Play may earn stamps that unlock cosmetic card backs,
  board themes, sounds, and cabinet decorations. There are no purchases,
  stamina, streak pressure, nested currencies, or claim screens.
- **Clear classic rules.** Familiar games remain recognisable. Any themed
  presentation is cosmetic and never hides state needed to play.
- **Deterministic where useful.** Seeded deals and puzzles can be replayed and
  tested. Randomness is owned by each game session rather than ambient runtime
  calls.

## Documentation

- [Game design](docs/GAME_DESIGN.md) defines the collection, cabinet identity,
  game rules, progression, accessibility, and non-goals.
- [Technical design](docs/TECHNICAL_DESIGN.md) defines responsive layout,
  touch input, shared game contracts, data, persistence, and testing.
- [TODO](TODO.md) is the phased implementation checklist and source of truth
  for outstanding work.
- `CODE_STANDARDS.md`, `GAME_DEVELOPMENT_GUIDE.md`, and
  `MACROQUAD_TOOLKIT.md` are shared WebHatchery references and remain generic.

## Technology

- Rust 2021
- Macroquad for the runtime, rendering, audio, and input
- `macroquad-toolkit` for virtual UI, assets, data loading, persistence,
  notifications, capture support, and reusable runtime behavior
- JSON under `assets/data/` for authored game data and presentation settings
- WebGL as the primary release, with a Windows build from the same codebase

Project code keeps game rules independent of rendering and input so seeded
sessions can be tested without opening a window. Shared collection code owns the
cabinet, navigation, settings, profile, persistence envelope, and common UI;
each of the 43 games owns its rules, state, commands, and board rendering.

## Validation

From this directory, use the project publisher as the required validation path:

```powershell
.\publish.ps1
```

During implementation, focused checks may also use:

```powershell
cargo test -p idle_hands
cargo clippy -p idle_hands --all-targets --all-features -- -D warnings
```

The publisher builds and validates the native and WebGL targets. Verification
screenshots belong directly in `docs/verification/` and should be replaced when
they show the same state as an earlier capture.
