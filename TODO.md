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
- [ ] Normalize touch/pointer input with tap, drag, swipe, cancel, and edge rules.
- [x] Build 44-point-minimum shared controls, modal sheets, help, and settings.
- [x] Build the responsive cabinet with objects/labels for all eight games;
  unavailable objects clearly say “Coming soon” and remain accessible.
- [ ] Implement profile settings, per-game save keys, and autosave lifecycle.
- [x] Implement seeded 2048 rules, swipe controls, visible direction controls,
  score/best tile, one undo, new-game confirmation, tutorial, and records.
- [ ] Add portrait/landscape capture scenes and replace the template screenshot.
- [ ] Create the first real title-screen `catalog_thumbnail.png`.

## Phase 2 — Minesweeper and grid foundations

- [ ] Implement first-tap-safe seeded mine placement and adjacency counts.
- [ ] Add beginner/intermediate/expert/custom board presets.
- [ ] Add Reveal/Flag modes, optional long-press shortcut, visible chord action,
  restart, timer, mine count, win/loss, tutorial, and records.
- [ ] Extract only proven shared grid geometry/stroke helpers from the two games.
- [ ] Test touch cancellation, safe-first-reveal, flood reveal, marking, chording,
  save/resume, and responsive cell sizing.

## Phase 3 — Sudoku and Nonogram

- [ ] Implement Sudoku rule state, conflicts, pencil marks, undo/erase, number
  pad, difficulty selection, assistance settings, tutorial, and records.
- [ ] Choose Sudoku generation or validated puzzle data; guarantee one solution.
- [ ] Implement Nonogram clues, fill/cross modes, tap and axis-locked drag strokes,
  undo, board focus/zoom for small phones, tutorial, and records.
- [ ] Choose Nonogram generation or a validated catalog for 5×5, 10×10, and
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

## Phase 5 — FreeCell

- [ ] Reuse shared cards while keeping a separate FreeCell rule engine.
- [ ] Implement cells, foundations, cascades, supermove capacity, legal move
  feedback, undo, hint, seeded deals, win detection, tutorial, and records.
- [ ] Add canonical and edge-case move tests, including empty-column capacity.
- [ ] Verify all actions through both tap selection and drag interaction.

## Phase 6 — Yahtzee-style scorecard and Reversi

- [ ] Resolve the public/storefront name for the five-dice scorecard game.
- [ ] Implement seeded dice, holds, three-roll turn flow, thirteen categories,
  score previews, bonus/total calculation, tutorial, and records.
- [ ] Implement Reversi legal moves, flips, pass/end rules, score, same-device
  play, tutorial, and records.
- [ ] Add at least two deterministic local-AI levels with bounded frame work.
- [ ] Test every score category and Reversi direction/pass/end edge case.

## Phase 7 — Progression and collection polish

- [ ] Implement one-time achievements and a single stamp total.
- [ ] Add cosmetic-only unlocks for card backs, board themes, sounds, and cabinet
  decorations; no gameplay effect or attention-pressure mechanics.
- [ ] Finish coherent cabinet art, object states, transitions, audio, and reduced
  motion substitutions.
- [ ] Add complete Rules, Records, Help, Settings, credits, and reset-data flows.
- [ ] Audit autosave/resume and save migration independently for all eight games.
- [ ] Audit touch-only completion on iPhone/iPad portrait and landscape sizes.
- [ ] Run the complete capture matrix, replace duplicate-state images, and update
  the catalog thumbnail with the final cabinet.
- [ ] Complete final `cargo fmt`, tests, warning-free Clippy, and `publish.ps1`.

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
