# Idle Hands — Technical Design

Status: Current post-launch architecture baseline
Date: 2026-08-15

## 1. Runtime Boundaries

Idle Hands is one Rust/Macroquad executable with a collection shell and 60
independent game modules. The shell owns app lifecycle and shared services. A
game module owns rules and presents state through commands; it does not directly
change the profile, global settings, or another game's save.

The shell treats a finite frame gap of at least 0.5 seconds as a possible
visibility return. While a live game is active, it cancels pointer capture,
flushes the active autosave, and blocks simulation behind a safe-pause sheet.
The player must tap RESUME PLAY before the game accepts input or advances
again. Tutorials, restart/reset confirmations, non-game screens, and an
already-paused round do not open a second sheet.

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
- capture scope tied to the originating screen and topmost visible interaction
  layer, with release discarded after a scope change;
- tap slop and drag/swipe thresholds scaled to the logical viewport;
- cancel behavior when the pointer leaves, the app loses focus, or a modal opens;
- cancellation when a second touch appears so multi-touch cannot fall through as
  a single-pointer action;
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
- favorites, recent, and rules shelves with shared Drawer Info routing;
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
defines typed schemas and semantic validators. The production construction path
loads the complete authored catalog from `content_config.json` alongside the
existing game and puzzle configuration:

```text
assets/data/
  content_config.json
  game_config.json
  puzzle_config.json
  texture_manifest.json
```

`content_config.json` owns the game catalog, tutorial/help/credit/profile copy,
variant labels, word and riddle pools, achievement copy, hint fallback copy,
and tunable game balance. `src/content.rs` validates catalog coverage, ordering,
word shapes, puzzle references, and balance invariants before `AppState::new`
constructs a playable state.

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
the main cabinet. The shared quick-browse panel exposes direct FAVORITES and
RECENT tabs, in-place favorite removal, a clear-recent action, and mixed
open/done/locked summaries; completed cards reuse the best-clear-time ledger.

The Records summary derives completed-drawer progress from `CollectionRecords`
through the shared `progression::completed_games` function, keeping the count
consistent with cabinet completion status without adding another save field.
The same record envelope keeps normalized elapsed and fastest-completion time
vectors keyed by `GameId::ALL`; `time_summary` derives total playtime, active
drawers, timed completions, and the fastest clear without another persisted
counter. Results and live timer badges use the shared duration formatter.
The runtime-only Statistics screen derives its collection summary from those same
vectors plus favorites, recent history, and daily results. `stats_data` owns the
sorting and tie-breaking for top-playtime drawers, while `statistics_ui` provides
desktop, compact-landscape, and portrait layouts with one touchable BACK route.
No statistics counter is persisted separately, so save migration cannot create
disagreements between the Records shelf, the cabinet, and the Statistics shelf.
The `statistics` and `statistics_accessible` capture scenes seed a small mixed
ledger for visual inspection: several active times, clear records, favorites,
recent drawers, and daily attempts. Capture setup resets this fixture before
rendering and never writes it to the player's save slots.
Achievement flags are stored as a normalized vector keyed by the 62-entry
`AchievementId::ALL` list. Legacy ten-entry arrays deserialize as vectors and
are padded with unearned late-game achievements.
The runtime-only `achievements_view` flag routes Records to the responsive
achievement shelf; its cards are read-only and BACK returns to Records.
Achievement cards render the associated `GameId` title rather than only
thematic award text, and the shelf participates in the shared large-text
capture path. Each card derives its goal description and progress label from
the same completion predicates used by `progression::sync`; the Records shelf
also exposes the next locked achievement in collection order.
The runtime-only `achievement_filter` value selects ALL, EARNED, or LOCKED
cards; it is reset when opening the shelf and is never persisted.
Achievement filter controls use touch-complete responsive targets and a
non-color active outline; button counts and card spacing are reflowed per
orientation so the full 62-entry shelf remains inside its panel.
Cosmetics use typed catalog entries with stamp costs for card backs, board
themes, sound sets, and cabinet decorations. Settings derive their equipped
name, open count, and next cost from that catalog across all responsive layouts;
profile and collection-save restore paths normalize locked indices back to the
first affordable option.
The runtime-only `daily_archive_view` and `daily_archive_scroll` values route
the Records screen to a newest-first view over the bounded `daily_results`
ledger. Archive pages are sized per viewport, page controls stop at both ends,
and capture scenes populate mixed attempted and cleared routes for verification.
The runtime-only `cabinet_filter` value selects ALL, OPEN, or DONE drawers;
it is never persisted and maps filtered slots back to canonical `GameId`
indices before dispatching favorite or open actions.
Filtered cards also render their canonical one-based drawer number rather than
the temporary slot number produced by the active filter.
The persisted `cabinet_sort` preference cycles through A-Z, OPEN FIRST, and
LAST PLAYED ordering; profile and legacy collection restores normalize unknown
values back into that three-mode cycle. The `cabinet_sorted` capture scene
seeds mixed completion records so the progress order has a deterministic visual
fixture.
The runtime-only `records_filter` value cycles the Records shelf through ALL
and the six cabinet categories. `records_cards` and `records_arcade` capture
scenes seed mixed completion rows and open the matching category shelf so the
responsive filter and paging presentation can be inspected deterministically.
The companion `rules_logic` and `rules_word` scenes open filtered Rules shelves
directly, keeping capture review independent of pointer timing or filter-cycle
state.

The runtime-only Tutorials screen derives its rows from `GameId::ALL` and the
normalized `tutorial_seen` vector. `tutorial_library_data` owns the visible
capacity and bounded scroll limit; `tutorial_library_ui` owns the responsive
card layout and dispatches `OpenTutorial` with the canonical game index. The
navigation handler opens the same game route as the cabinet, then explicitly
starts its tutorial so replaying a seen lesson remains touch-complete. The
runtime-only `tutorial_filter` flag narrows the derived rows to unseen lessons
and resets `library_scroll` whenever it changes. The `tutorials` capture fixture
seeds mixed seen state, while `tutorials_accessible` opens the filtered view.

The runtime-only Finder screen reuses `cabinet_filter` as its five-value
alphabet bucket while it is open; entering Finder resets the bucket and
`library_scroll`. `finder_data` sorts the canonical `GameId::ALL` list by
display title, partitions it into ALL, #–F, G–M, N–S, and T–Z buckets, and
owns the layout-aware page limits. `finder_ui` owns the responsive cards,
touchable bucket controls, paging, and BACK route. Cards dispatch the same
`Open` action used by the cabinet, so unavailable drawers keep their normal
storefront notice. The `finder`, `finder_filtered`, and `finder_scrolled`
capture fixtures seed the default, filtered, and paged states without writing
to player saves.

The runtime-only Profile screen is a touch-first nameplate editor over the
persisted `profile_name` field. `profile_data` owns the stable curated name
list and recognizes legacy custom names without rewriting them until the
player chooses a new plate. `profile_ui` lays out the same eight `SetProfileName`
cards in desktop, compact-landscape, and portrait modes; BACK returns to
Settings. The action handler requests the normal profile autosave after a
selection. `profile` and `profile_accessible` capture fixtures seed a non-default
plate, with the latter inheriting the shared high-contrast and large-text
capture setup.

The runtime-only `DrawerInfo(GameId)` screen is the shared pre-launch details
route for cabinet and Finder cards. `drawer_info_ui` derives category, status,
availability, variants, and favorite copy from the existing descriptor,
storefront, and cabinet-status boundaries, then dispatches the same `Open` and
`ToggleFavorite` actions used elsewhere. Cabinet, Finder, Favorites, Recent,
and Rules rows reserve a separate INFO touch lane from their launch or favorite
lane. The
`drawer_info`, `drawer_info_accessible`, `favorites_info`, and `recent_info`
capture fixtures seed representative inspection surfaces without changing
player saves.

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

Autosave occurs after committed player commands, on return Home, and before
the safe-pause sheet shown after a frame-gap visibility return. Writes are
coalesced so drag previews and animations do not create storage churn.

Load failures never overwrite the rejected bytes. The shell quarantines the
affected slot, records a runtime-only recovery notice, and continues with safe
defaults or the remaining valid slots. DISMISS schedules an autosave of that
clean state; the notice is not persisted into profile or game snapshots.

The toolkit notification history is bounded and session-only. Settings exposes
the newest entries through a responsive NOTICES overlay; the view shortens long
messages, renders newest-first, and uses a neutral high-contrast row color when
severity accents would be insufficient.

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
extends that same contract across 60 games, with collection-wide persistence,
records, tutorials, rules, and responsive capacity treated as maintained
systems rather than launch-only scaffolding.
