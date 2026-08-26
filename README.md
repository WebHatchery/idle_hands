# Idle Hands

Idle Hands is a touch-first collection of small, replayable games for iPhone,
iPad, desktop browsers, and Windows. Its home screen is a warmly illustrated
drawer cabinet: each physical object opens a different game, from a deck of
cards to a dice cup or a little wooden minefield.

The current `1.0.0` release candidate is a playable 60-game cabinet with
persistent sessions, records, tutorials, responsive touch layouts, and
post-launch refinements. [TODO.md](TODO.md) records the remaining owner-only
storefront, rights confirmation, and physical-device acceptance gates.

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
Flood It, Color Sort, Battleship, Word Grid, Pipe Loop, Maze Walk, Match Three,
Pyramid, TriPeaks, Nim, Word Ladder, Space Invaders, Asteroids, Frogger, Munch
Maze, Block Stack, Terrain Cannon, Fling Fury, Paddle Duel, Riddle Room,
Pattern Vault, Sum Circuit, Orbit Order, and Word Forge.

The cabinet groups the collection into Cards, Logic, Board, Word, Arcade, and
Misc. Arcade is reserved for the continuously moving games: Snake, Breakout,
Tiny Tower Defence, Space Invaders, Asteroids, Frogger, Munch Maze, Block Stack,
Terrain Cannon, Fling Fury, and Paddle Duel. Dungeon Sweeper belongs with
Logic, Battleship with Board, and One Room Roguelike with Misc; Higher or Lower remains a
Cards game.

Misc also includes five original puzzle drawers: Riddle Room, Pattern Vault,
Sum Circuit, Orbit Order, and Word Forge.

## Product Principles

- **Touch is complete, not supplemental.** Every game, tutorial, menu, undo,
  restart, settings, and recovery action has a visible tap target or a direct
  touch gesture. A keyboard is never required.
- **Comfortable on phones and tablets.** Layouts adapt to portrait and
  landscape screens, safe areas, and coarse pointers. Important targets are at
  least 44 logical points and do not depend on hover.
- **Fast to enter and leave.** The cabinet restores each game's unfinished
  session locally and exposes a visible CONTINUE action for the last-opened
  drawer. A player can mark favorite drawers with the cabinet's touch-sized
  markers, open optional FAVORITES and RECENT quick lists, reach a game in one
  tap, and return home without losing progress.
- **Easy to scan.** The cabinet offers ALL, OPEN, and DONE filters so a large
  collection can be narrowed to unfinished or completed drawers without
  changing the canonical order; each filter shows its live count and explains
  an empty shelf, and the active filter remains outlined in high-contrast mode.
- **Calm progression.** Play may earn stamps that unlock cosmetic card backs,
  board themes, sounds, and cabinet decorations. There are no purchases,
  stamina, streak pressure, nested currencies, or claim screens.
- **Collection feedback.** Records shows stamps, achievements, and the number
  of completed drawers out of the full cabinet so progress stays legible. Each
  drawer has its own completion achievement alongside the collection awards,
  and a visible ACHIEVEMENTS shelf names the associated drawer and shows which
  awards are earned or locked. ALL, EARNED, and LOCKED filters keep the shelf
  easy to scan.
- **Clear classic rules.** Familiar games remain recognisable. Any themed
  presentation is cosmetic and never hides state needed to play.
- **Visible rule cards.** Every drawer names its active variant in a compact
  touch control; tapping the card starts the next deterministic ruleset with
  the same confirmation and save behavior as a fresh round.
- **Real-time means real-time.** Arcade rounds advance from the frame clock,
  include visible pause controls, and remain fully playable with touch alone.
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
- [Release notes](RELEASE_NOTES.md) describe the `1.0.0` release candidate.
- [Artwork provenance](assets/THIRD_PARTY_NOTICES.md) records the owner's
  confirmation and repository history for the procedural and AI-generated art.
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
each of the 60 games owns its rules, state, commands, and board rendering.

## Validation

### Analytics

Gameplay analytics are disabled for the storefront release by
`IDLE_HANDS_ANALYTICS_ENABLED = "false"` in `.cargo/config.toml`. The endpoint
and write key remain as dormant preview configuration so analytics can be
restored deliberately once production details are available; changing them
alone does not enable collection.

### Demo builds

The `demo` Cargo feature produces the storefront edition. It keeps five games
from each of the six cabinet categories playable (30 total), leaves the rest
visible as full-version drawers, and shows an itch.io purchase message when a
locked drawer is tapped. The normal build remains the unrestricted 60-game
edition.

```powershell
cargo build --release --features demo --target-dir target-demo
cargo build --release --features demo --target wasm32-unknown-unknown --target-dir target-demo
```

The resulting Windows executable is
`target-demo/release/idle_hands.exe`; the WebGL module is
`target-demo/wasm32-unknown-unknown/release/idle_hands.wasm`. Keeping the demo
in its own target directory prevents it from being mistaken for a full build.

From this directory, use the project publisher as the required validation path:

```powershell
.\publish.ps1
```

During implementation, focused checks may also use:

```powershell
cargo test -p idle_hands
cargo clippy -p idle_hands --all-targets --all-features -- -D warnings
.\scripts\test_game_suites.ps1
npm ci
npx playwright install chromium
npm run test:webgl
```

`test_game_suites.ps1` runs one independently filterable host-contract suite
for each of the 60 catalog games, checks the suite registry against `GameId`,
and then runs the complete Rust test set. Pass `-TargetDir target-codex` when a
separate local Cargo target directory is needed.

The publisher builds and validates the native and WebGL targets. The browser
smoke suite then serves the packaged preview deployment and checks loading,
console/runtime errors, real touch input, audio activation, resize/fullscreen,
local persistence across reload, and a visible recovery action. Set
`IDLE_HANDS_WEB_ROOT` to the directory containing the deployed `idle_hands/`
folder when it is not under `dist/browser-smoke/games`.

Verification screenshots belong directly in `docs/verification/` and should be
replaced when they show the same state as an earlier capture. Automated Chromium
coverage complements, but does not replace, physical iPhone/iPad Safari and
Windows acceptance testing.
