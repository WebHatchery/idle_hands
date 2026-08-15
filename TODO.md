# TODO — Idle Hands

This is the delivery checklist for the first collection. A phase is complete
only when its player flow is touch-complete, responsive at the documented test
sizes, covered by rule tests, and passes `publish.ps1`.

## Phase 0 — Define the collection

- [x] Replace the template README with the Idle Hands product overview.
- [x] Fix the launch scope at Solitaire, FreeCell, Sudoku, Minesweeper, 2048,
  Nonogram, Yahtzee, and Reversi.
- [x] Define the illustrated cabinet identity and cosmetic-only progression.
- [x] Record touch, responsive, accessibility, persistence, deterministic seed,
  testing, and architecture requirements.
- [x] Update package, capture, runtime config, and web-page identity from the
  template to Idle Hands.
- [x] Mark the web listing as in development with touch-first controls.
- [x] Replace the inherited template verification image once the cabinet exists
  in Phase 1.
- [x] Create `catalog_thumbnail.png` from the finished title/cabinet screen in
  Phase 1; do not use the template screen as catalog art.

## Phase 1 — Cabinet, app shell, and 2048 vertical slice

- [x] Replace the template session with app-level screen/navigation state.
- [x] Add `GameId`, collection descriptors, game host lifecycle, and navigation.
- [ ] Implement safe viewport handling and compact/medium/expanded layouts.
  - [x] Add an aspect-preserving logical viewport with letterbox edge rejection.
  - [x] Add a full-size compact portrait canvas for the cabinet, 2048, Sudoku,
    Nonogram, Minesweeper, Solitaire, FreeCell, Fivefold, Reversi, and Settings flows.
- [ ] Normalize touch/pointer input with tap, drag, swipe, cancel, and edge rules.
  - [x] Normalize pointer taps, drags, cancellations, and logical edge rules.
- [x] Build 44-point-minimum shared controls, modal sheets, help, and settings.
- [x] Build the responsive cabinet with objects/labels for all eight games;
  unavailable objects clearly say “Coming soon” and remain accessible.
- [ ] Implement profile settings, per-game save keys, and autosave lifecycle.
  - [x] Add independent versioned profile and active-game slots alongside the
    combined migration fallback.
- [x] Implement seeded 2048 rules, swipe controls, visible direction controls,
  score/best tile, one undo, new-game confirmation, tutorial, and records.
- [ ] Add portrait/landscape capture scenes and replace the template screenshot.
  - [x] Route named cabinet, game, library, and settings scenes through the
    capture harness and verify desktop plus portrait captures.
- [x] Create the first real title-screen `catalog_thumbnail.png`.

## Phase 2 — Minesweeper and grid foundations

- [x] Implement first-tap-safe seeded mine placement and adjacency counts.
- [x] Add beginner/intermediate/expert/custom board presets.
- [x] Add Reveal/Flag modes, optional long-press shortcut, visible chord action,
  restart, timer, mine count, win/loss, tutorial, and records.
- [ ] Extract only proven shared grid geometry/stroke helpers from the two games.
  - [x] Share responsive grid sizing and half-open hit-testing across the grid drawers.
- [ ] Test touch cancellation, safe-first-reveal, flood reveal, marking, chording,
  save/resume, and responsive cell sizing.
  - [x] Cover shared grid boundaries and independent row/column sizing in rule tests.
  - [x] Verify the compact Minesweeper board, presets, mode toggle, restart,
    and long-press flagging at 390×844.

## Phase 3 — Sudoku and Nonogram

- [ ] Implement Sudoku rule state, conflicts, pencil marks, undo/erase, number
  pad, difficulty selection, assistance settings, tutorial, and records.
- [ ] Choose Sudoku generation or validated puzzle data; guarantee one solution.
- [ ] Implement Nonogram clues, fill/cross modes, tap and axis-locked drag strokes,
  undo, board focus/zoom for small phones, tutorial, and records.
- [x] Choose Nonogram generation or a validated catalog for 5×5, 10×10, and
  15×15 puzzles.
- [ ] Add high-contrast and large-text verification for dense grid screens.

## Phase 4 — Solitaire and shared cards

- [ ] Build shared card identity, deck, seeded shuffle, visuals, hit regions,
  selection, drag/drop, stack layout, and animation.
- [ ] Implement Klondike tableau, stock/waste, foundations, legal moves, scoring,
  win detection, undo, hint, new deal, tutorial, and records.
- [ ] Confirm launch draw/redeal options and encode them as versioned rulesets.
- [ ] Ensure select-then-select can complete every action without dragging.
- [ ] Verify readable card ranks and usable stacked-card targets on compact phones.
  - [x] Verify compact Solitaire and FreeCell selection, foundations, tableau,
    undo, and new-deal controls at 390×844.

## Phase 5 — FreeCell

- [ ] Reuse shared cards while keeping a separate FreeCell rule engine.
- [ ] Implement cells, foundations, cascades, supermove capacity, legal move
  feedback, undo, hint, seeded deals, win detection, tutorial, and records.
- [ ] Add canonical and edge-case move tests, including empty-column capacity.
- [ ] Verify all actions through both tap selection and drag interaction.

## Phase 6 — Yahtzee-style scorecard and Reversi

- [x] Resolve the public/storefront name for the five-dice scorecard game as
  Fivefold.
- [x] Implement seeded dice, holds, three-roll turn flow, thirteen categories,
  score previews, bonus/total calculation, tutorial, and records.
- [x] Implement Reversi legal moves, flips, pass/end rules, score, same-device
  play, tutorial, and records.
- [x] Add at least two deterministic local-AI levels with bounded frame work.
- [x] Test every score category and Reversi direction/pass/end edge case.

## Phase 7 — Progression and collection polish

- [x] Implement one-time achievements and a single persisted stamp total.
- [x] Add cosmetic-only unlocks for card backs, board themes, sounds, and cabinet
  decorations; no gameplay effect or attention-pressure mechanics.
  - [x] Add stamp-gated card backs, board themes, and cabinet decorations with
    persisted touch-selectable choices.
  - [x] Connect the sound-set choices to generated audio cues.
- [ ] Finish coherent cabinet art, object states, transitions, audio, and reduced
  motion substitutions.
  - [x] Add cabinet completion/in-progress states and a reduced-motion-safe fade
    transition between screens.
- [ ] Add complete Rules, Records, Help, Settings, credits, and reset-data flows.
  - [x] Add a persisted Records screen covering current best scores and solves.
  - [x] Add first-run and replayable tutorials with exact visible touch instructions.
  - [x] Add dedicated Rules and Credits screens plus a confirmed reset-data flow.
- [ ] Audit autosave/resume and save migration independently for all eight games.
  - [x] Cover all eight independent game snapshots and migration defaults for
    progression and cosmetic fields.
- [ ] Audit touch-only completion on iPhone/iPad portrait and landscape sizes.
  - [x] Verify touch-sized portrait cabinet, 2048, Sudoku, Nonogram, and
    Settings controls at 390×844.
  - [x] Verify touch-sized portrait Minesweeper, Solitaire, FreeCell, and
    Fivefold controls at 390×844.
  - [x] Verify touch-sized portrait Reversi board, legal-move markers, pass,
    new-board, and AI mode controls at 390×844.
  - [x] Verify compact portrait Help, Records, Rules, and Credits screens at
    390×844.
  - [x] Capture the complete landscape baseline at 844×390 for all named
    game, library, settings, and cabinet scenes; touch-size review remains.
  - [x] Verify touch-sized medium landscape cabinet and 2048 layouts at
    844×390, including cabinet navigation, arrows, undo, and new game.
  - [x] Verify touch-sized medium landscape Minesweeper, Sudoku, Nonogram,
    and Reversi boards with their primary controls at 844×390.
  - [x] Verify touch-sized medium landscape Solitaire, FreeCell, and Fivefold
    card/scorecard controls at 844×390.
  - [x] Verify touch-sized medium landscape Help, Records, Rules, Credits,
    and Settings controls at 844×390, including reset confirmation.
- [x] Run the complete capture matrix, replace duplicate-state images, and update
  the catalog thumbnail with the final cabinet.
- [x] Complete final `cargo fmt`, tests, warning-free Clippy, and `publish.ps1`.

## Post-launch candidates (not scheduled)

- Card family: Spider Solitaire, Klondike Golf, Higher or Lower, Blackjack.
- Puzzles/tabletop: Lights Out, Sliding Puzzle, Mastermind, Memory/Pairs, Word
  Search, Hangman, Connect Four, Tic-Tac-Toe, Checkers, Peg Solitaire, Mahjong
  Solitaire.
- Arcade: Snake and Breakout.
- Original cabinet games: Dungeon Sweeper, Potion 2048, Tiny Tower Defence, One
  Room Roguelike, and Daily Dungeon.

Do not begin a post-launch candidate until the eight-game first collection is
touch-complete and the shared system it would reuse is stable.
