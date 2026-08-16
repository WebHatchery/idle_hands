# Idle Hands — Technical Design

Status: Current post-launch architecture baseline
Date: 2026-08-15

## 1. Runtime Boundaries

Idle Hands is one Rust/Macroquad executable with a collection shell and 47
independent game modules. The shell owns app lifecycle and shared services. A
game module owns rules and presents state through commands; it does not directly
change the profile, global settings, or another game's save.

The original proposed module tree below documents the architectural intent.
The current implementation uses named Rust source files under src/ with
game-specific UI siblings and child test modules; the enum-owned host remains
the active integration boundary.

Proposed module boundaries:

```text
src/
  main.rs                 runtime entry and capture harness
  app.rs                  top-level screen state and transitions
  collection.rs           game IDs, descriptors, availability
  input.rs                pointer/touch gesture normalization
  layout.rs               safe viewport and responsive breakpoints
  profile.rs              settings, stamps, records, save index
  screens/                cabinet, game host, settings, help, records
  games/
    solitaire.rs          parent for Solitaire implementation modules
    freecell.rs
    sudoku.rs
    minesweeper.rs
    game_2048.rs
    nonogram.rs
    yahtzee.rs
    reversi.rs
  cards/                  shared deck, card UI, move animation
  ui/                     shared controls, sheets, typography, themes
```

Named parent files with child directories are used instead of `mod.rs`. Every
Rust file, including tests, remains at or below 800 physical lines.

## 2. Game Contract

Each game should expose a small host-facing contract with these concepts:

- stable `GameId` and rules/generator versions;
- lifecycle commands: new, resume, pause, restart, serialize, restore;
- viewport update and presentation rendering;
- normalized pointer/gesture input converted into game-specific commands;
- tutorial steps expressed as visible-control or direct-gesture instructions;
- summary data for the cabinet and records screen;
- deterministic test construction from a seed.

Rust traits should be introduced only after two concrete games reveal the
shared shape. An enum-owned host is preferable initially because the collection
is closed and it avoids object-safety and serialization complexity.

Rule state must not depend on drawing APIs. Tests drive commands such as
`RevealCell`, `MoveCards`, `SetDigit`, `Swipe`, `HoldDie`, or `PlaceDisc` and
assert the resulting state.

## 3. Touch and Pointer Input

Macroquad reports touch and pointer events; the collection normalizes them into
one primary-pointer stream before game code sees them. The input layer owns:

- pointer identity and capture from press through release;
- tap slop and drag/swipe thresholds scaled to the logical viewport;
- cancel behavior when the pointer leaves, the app loses focus, or a modal opens;
- long-press recognition only as an optional shortcut;
- edge-gesture exclusion so browser navigation does not become a game move;
- gesture arbitration between a board, a scroll container, and global navigation.

Every gesture has a visible alternative. Card moves support select-then-select
in addition to drag. Minesweeper and Nonogram expose explicit modes. 2048 has
direction buttons. Tutorials name the actual target: “Tap ROLL,” “Tap FLAG,
then a square,” or “Swipe the board left.”

No rule action fires merely from hover. Activation normally occurs on release
within the same target, allowing cancellation. Immediate press actions must be
idempotent and visibly depressed.

## 4. Responsive Layout

Layout uses the actual safe viewport on every frame rather than one fixed
desktop resolution. A `SafeViewport` subtracts platform/browser insets and
provides density-independent coordinates.

Initial layout classes:

| Class | Shortest safe side | Typical arrangement |
| --- | ---: | --- |
| Compact | below 600 | One primary board region; controls dock below or in a sheet |
| Medium | 600–899 | Board plus persistent secondary controls |
| Expanded | 900+ | Wider cabinet or board/score side-by-side |

Orientation is a layout input, not an error state. A compact portrait phone may
stack the board, status, and actions; landscape may dock actions beside the
board. Boards preserve square cells where rules require them. Scroll is limited
to menus, rules, records, and the cabinet—not active boards unless Nonogram
explicitly enters pan/zoom mode.

The shared UI test matrix begins with 320×568, 390×844, 568×320, 844×390,
768×1024, 1024×768, and 1280×720 logical/CSS-pixel viewports. Device pixel ratio
is varied separately. Screens must remain playable with 44-point targets and no
clipped critical action.

## 5. Shared Systems

The collection shell owns:

- cabinet navigation and transition sheets;
- settings, accessibility, tutorials, help, and confirmation dialogs;
- per-game autosave slots and save-version migration dispatch;
- records, achievements/stamps, and cosmetic unlock inventory;
- theme tokens, typography, buttons, number pads, and mode selectors;
- seeded RNG construction and replay identity formatting;
- audio buses and reduced-motion behavior;
- screenshot scene registration and deterministic capture state.

The card package owns rank/suit/card identity, deck construction, face/back
rendering, selection, hit testing, drag/drop, move animation, and stack layout.
Solitaire and FreeCell retain separate rule engines.

Grid helpers may share geometry and drag-stroke traversal, but cells and rules
remain game-specific. Avoid a universal board abstraction that obscures simple
rules.

## 6. Data and Assets

All game JSON is loaded through `macroquad_toolkit::data_loader`. The project
defines typed schemas and semantic validators. Candidate data files are:

```text
assets/data/
  collection.json
  settings.json
  themes.json
  achievements.json
  solitaire_rules.json
  sudoku_puzzles.json
  nonogram_puzzles.json
  reversi_ai.json
```

Only runtime-loaded assets appear in `asset_registry.json`; embedded JSON does
not. Missing art uses an obvious development placeholder and fails publish when
the launch manifest requires the asset. Cabinet objects need selected,
unselected, locked (if ever used), and high-contrast-readable states without
depending on color alone.

## 7. Persistence

Use toolkit save slots with a versioned collection envelope. Separate profile
settings from game sessions so a corrupt or migrated game save cannot erase
global accessibility choices. The profile also stores a normalized boolean
favorite vector keyed by `GameId::ALL`; short legacy vectors are padded with
false values and never change the selected game.

The runtime-only `favorites_view` and `recent_view` flags open optional
quick-browse lists from the cabinet. Favorites are persisted as a normalized
boolean vector; recent history is persisted as a deduplicated, newest-first
list capped at five `GameId` values. Neither list changes the default drawer
ordering, and selecting a listed game routes through the same `Open` action as
the main cabinet.

The Records summary derives completed-drawer progress from `CollectionRecords`
through the shared `progression::completed_games` function, keeping the count
consistent with cabinet completion status without adding another save field.
Achievement flags are stored as a normalized vector keyed by the 49-entry
`AchievementId::ALL` list. Legacy ten-entry arrays deserialize as vectors and
are padded with unearned late-game achievements.
The runtime-only `achievements_view` flag routes Records to the responsive
achievement shelf; its cards are read-only and BACK returns to Records.

Conceptual keys:

```text
idle_hands/profile
idle_hands/game/solitaire
idle_hands/game/freecell
...
```

Each game save includes schema version, rules/generator version, seed, current
rule state, elapsed/score data where relevant, and undo history only when that
game promises undo after resume. The shell keeps a small index for continue
badges but treats each game slot as authoritative.

Autosave occurs after committed player commands, on return Home, and on pause
or visibility loss where available. Writes are coalesced so drag previews and
animations do not create storage churn.

Explicit NEW actions use a shared confirmation modal before dispatching the
game-specific reset command. The pending command is held only in runtime state;
CANCEL clears it, while START dispatches the existing reset handler unchanged.

## 8. Determinism and AI

Each new session receives a seed and owns its RNG. Rendering, time, pointer
position, and global random functions never alter rules. Seed/generator version
pairs reproduce card deals, mine placement, puzzle selection/generation, dice
rolls, and AI tie breaks.

Reversi AI runs from a copied rule state with deterministic move ordering. Work
is bounded by depth, node count, or elapsed budget and must not block touch
feedback. Native threads are not assumed because WebGL is a primary target; an
incremental search across frames is preferred if deeper play is needed.

## 9. Testing and Verification

Tests live in separate child files (`foo/tests.rs`) and cover:

- pure rule legality, win/loss/end conditions, scoring, and undo invariants;
- seeded replay stability and save round trips/migrations;
- generated Sudoku uniqueness and Nonogram validity if generators ship;
- touch gesture thresholds, cancellation, and visible alternative commands;
- responsive layout bounds and minimum target sizes at every matrix viewport;
- Reversi AI legal output and bounded work;
- asset registry consistency and the 800-line source limit.

After meaningful work, `publish.ps1` is the required end-to-end path. Capture
scenes should include cabinet portrait/landscape, every game’s initial/play/end
states, settings at large text, and high contrast. Same-state screenshots replace
their predecessors in `docs/verification/`.

## 10. Delivery Strategy

Phase 1 builds the shell and proves touch/responsive behavior with 2048 because
its command surface is small. Minesweeper then exercises tap modes and seeded
boards. Sudoku and Nonogram establish dense adaptive grids. Solitaire creates
the card foundation before FreeCell consumes it. Yahtzee exercises scorecards
and animated randomness; Reversi closes the set with local AI. Final phases add
cosmetic progression and collection-wide polish only after all foundation games
are fully playable without a keyboard. The current post-launch implementation
extends that same contract across 47 games, with collection-wide persistence,
records, tutorials, rules, and responsive capacity treated as maintained
systems rather than launch-only scaffolding.
