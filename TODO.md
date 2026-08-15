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
- [x] Implement safe viewport handling and compact/medium/expanded layouts.
  - [x] Add an aspect-preserving logical viewport with letterbox edge rejection.
  - [x] Add a full-size compact portrait canvas for the cabinet, 2048, Sudoku,
    Nonogram, Minesweeper, Solitaire, FreeCell, Fivefold, Reversi, and Settings flows.
- [x] Normalize touch/pointer input with tap, drag, swipe, cancel, and edge rules.
  - [x] Normalize pointer taps, drags, cancellations, and logical edge rules.
- [x] Build 44-point-minimum shared controls, modal sheets, help, and settings.
- [x] Build the responsive cabinet with objects/labels for all eight games;
  unavailable objects clearly say “Coming soon” and remain accessible.
- [x] Implement profile settings, per-game save keys, and autosave lifecycle.
  - [x] Add independent versioned profile and active-game slots alongside the
    combined migration fallback.
- [x] Implement seeded 2048 rules, swipe controls, visible direction controls,
  score/best tile, one undo, new-game confirmation, tutorial, and records.
- [x] Add portrait/landscape capture scenes and replace the template screenshot.
  - [x] Route named cabinet, game, library, and settings scenes through the
    capture harness and verify desktop plus portrait captures.
- [x] Create the first real title-screen `catalog_thumbnail.png`.

## Phase 2 — Minesweeper and grid foundations

- [x] Implement first-tap-safe seeded mine placement and adjacency counts.
- [x] Add beginner/intermediate/expert/custom board presets.
- [x] Add Reveal/Flag modes, optional long-press shortcut, visible chord action,
  restart, timer, mine count, win/loss, tutorial, and records.
- [x] Extract only proven shared grid geometry/stroke helpers from the two games.
  - [x] Share responsive grid sizing and half-open hit-testing across the grid drawers.
- [x] Test touch cancellation, safe-first-reveal, flood reveal, marking, chording,
  save/resume, and responsive cell sizing.
  - [x] Cover shared grid boundaries and independent row/column sizing in rule tests.
  - [x] Verify the compact Minesweeper board, presets, mode toggle, restart,
    and long-press flagging at 390×844.

## Phase 3 — Sudoku and Nonogram

- [x] Implement Sudoku rule state, conflicts, pencil marks, undo/erase, number
  pad, difficulty selection, assistance settings, tutorial, and records.
  - [x] Keep Easy, Medium, and Hard as separate catalog puzzles with distinct
    validated boards.
- [x] Choose Sudoku generation or validated puzzle data; guarantee one solution.
  - [x] Validate every authored Sudoku difficulty with a bounded unique-solution check.
- [x] Implement Nonogram clues, fill/cross modes, tap and axis-locked drag
  strokes, undo, board focus/zoom for small phones, tutorial, and records.
  - [x] Add a touch-sized 9 × 9 focus window with explicit portrait pan controls
    for the 15 × 15 catalog board.
- [x] Choose Nonogram generation or a validated catalog for 5×5, 10×10, and
  15×15 puzzles.
- [x] Add high-contrast and large-text verification for dense grid screens.
  - [x] Verify accessible expanded desktop Sudoku, Minesweeper, and Nonogram
    boards at 1280×720 without control overlap.

## Phase 4 — Solitaire and shared cards

- [x] Build shared card identity, deck, seeded shuffle, visuals, hit regions,
  selection, drag/drop, stack layout, and animation.
  - [x] Share card identity and seeded deck construction between Solitaire and
    FreeCell.
  - [x] Centralize card faces, backs, labels, selection glow, and reduced-motion
    behavior across desktop, portrait, and landscape card layouts.
- [x] Implement Klondike tableau, stock/waste, foundations, legal moves, scoring,
  win detection, undo, hint, new deal, tutorial, and records.
  - [x] Add deterministic visible Hint controls to Solitaire and FreeCell in
    desktop, portrait, and medium-landscape layouts.
  - [x] Route logical card drags through the same select-then-destination
    actions as taps.
  - [x] Explain rejected tableau and foundation moves with short feedback.
- [x] Confirm launch draw/redeal options and encode them as versioned rulesets.
  - [x] Launch Solitaire with the explicit Draw 1, unlimited-redeal ruleset and
    preserve the ruleset through saves.
- [x] Ensure select-then-select can complete every action without dragging.
  - [x] Cover legal tableau, cascade, foundation, and rejected-destination
    selection behavior in rule tests.
- [x] Verify readable card ranks and usable stacked-card targets on compact phones.
  - [x] Verify compact Solitaire and FreeCell selection, foundations, tableau,
    undo, and new-deal controls at 390×844.

## Phase 5 — FreeCell

- [x] Reuse shared cards while keeping a separate FreeCell rule engine.
  - [x] Use the shared card identity and seeded deck for FreeCell deals.
- [x] Implement cells, foundations, cascades, supermove capacity, legal move
  feedback, undo, hint, seeded deals, win detection, tutorial, and records.
  - [x] Add deterministic visible Hint controls to Solitaire and FreeCell in
    desktop, portrait, and medium-landscape layouts.
  - [x] Route logical card drags through the same select-then-destination
    actions as taps.
  - [x] Explain rejected cascade and foundation moves with short feedback.
- [x] Add canonical and edge-case move tests, including empty-column capacity.
- [x] Verify all actions through both tap selection and drag interaction.
  - [x] Cover compact portrait Solitaire and FreeCell tap targets and shared
    source-to-destination drag dispatch in deterministic routing tests.

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
- [x] Finish coherent cabinet art, object states, transitions, audio, and reduced
  motion substitutions.
  - [x] Add cabinet completion/in-progress states and a reduced-motion-safe fade
    transition between screens.
  - [x] Add shared shelf rails and an accent motif to desktop, portrait, and
    medium-landscape cabinet scenes.
- [x] Add complete Rules, Records, Help, Settings, credits, and reset-data flows.
  - [x] Add a persisted Records screen covering current best scores and solves.
  - [x] Add first-run and replayable tutorials with exact visible touch instructions.
  - [x] Add dedicated Rules and Credits screens plus a confirmed reset-data flow.
  - [x] Verify the compact portrait reset confirmation renders visible Cancel
    and Reset actions at 390×844.
  - [x] Expose the shared Load action alongside Save on compact portrait
    Settings.
- [x] Add persisted High Contrast and Large Text settings and apply them to
  dense portrait and medium-landscape puzzle boards.
- [x] Audit autosave/resume and save migration independently for all eight games.
  - [x] Cover all eight independent game snapshots and migration defaults for
    progression and cosmetic fields.
  - [x] Verify all eight snapshots can be restored sequentially without
    overwriting another game's state.
- [x] Audit touch-only completion on iPhone/iPad portrait and landscape sizes.
  - [x] Verify touch-sized portrait cabinet, 2048, Sudoku, Nonogram, and
    Settings controls at 390×844.
  - [x] Verify touch-sized portrait Minesweeper, Solitaire, FreeCell, and
    Fivefold controls at 390×844.
  - [x] Verify touch-sized portrait Reversi board, legal-move markers, pass,
    new-board, and AI mode controls at 390×844.
  - [x] Verify the touch-sized portrait 15 × 15 Nonogram focus window and pan
    controls at 390×844.
  - [x] Verify compact portrait Help, Records, Rules, and Credits screens at
    390×844.
  - [x] Capture the complete landscape baseline at 844×390 for all named
    game, library, settings, and cabinet scenes; complete touch-size review.
  - [x] Verify touch-sized medium landscape cabinet and 2048 layouts at
    844×390, including cabinet navigation, arrows, undo, and new game.
  - [x] Verify touch-sized medium landscape Minesweeper, Sudoku, Nonogram,
    and Reversi boards with their primary controls at 844×390.
  - [x] Verify the touch-sized medium landscape 15 × 15 Nonogram focus window
    and pan controls at 844×390.
  - [x] Verify touch-sized medium landscape Solitaire, FreeCell, and Fivefold
    card/scorecard controls at 844×390.
  - [x] Verify touch-sized medium landscape Help, Records, Rules, Credits,
    and Settings controls at 844×390, including reset confirmation.
  - [x] Route first-run and replayable tutorials through a visible medium
    landscape overlay with an explicit CONTINUE touch target.
  - [x] Route medium-landscape Nonogram strokes and Minesweeper long-press
    flags through their responsive board coordinates.
- [x] Run the complete capture matrix, replace duplicate-state images, and update
  the catalog thumbnail with the final cabinet.
- [x] Complete final `cargo fmt`, tests, warning-free Clippy, and `publish.ps1`.

## Phase 8 — First post-launch cabinet game

- [x] Add Lights Out as a ninth deterministic, touch-complete cabinet game.
  - [x] Add seeded solvable boards, cross toggles, win detection, undo, and
    new-board reset with focused rule tests.
  - [x] Add persisted active state and best-move records without breaking
    existing saves or per-game snapshot isolation.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, and verification captures.
- [x] Publish the expanded cabinet and game through the standard validation
    path.

## Phase 10 — Third post-launch cabinet game

- [x] Add Memory/Pairs as an eleventh deterministic, touch-complete cabinet
  game.
  - [x] Add seeded pair deals, face-up matching, mismatch recovery, undo, new
    board reset, and focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, and verification captures.

## Phase 11 — Fourth post-launch cabinet game

- [x] Add Sliding Puzzle as a twelfth deterministic, touch-complete cabinet
  game.
  - [x] Add a guaranteed-solvable seeded shuffle, adjacent tile moves, win
    detection, undo, new board reset, and focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, and verification captures.

## Phase 12 — Fifth post-launch cabinet game

- [x] Add Mastermind as a thirteenth deterministic, touch-complete cabinet
  game with a seeded four-color code, ten guess rows, and duplicate-aware
  feedback.
  - [x] Cover code scoring, win/loss boundaries, clear, undo, and seeded
    boards with focused rule tests.
  - [x] Add persisted active state and best-row records with snapshot,
    achievement, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, and verification captures.

## Phase 13 — Sixth post-launch cabinet game

- [x] Add Spider as a fourteenth deterministic, touch-complete cabinet game
  using a one-suit ruleset with eight tableau columns and eight quiet webs.
  - [x] Cover seeded 104-card deals, descending run moves, stock dealing,
    completed-run removal, undo, and win detection with focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 14 — Seventh post-launch cabinet game

- [x] Add Word Search as a fifteenth deterministic, touch-complete cabinet
  game with a seeded letter field and six quiet words.
  - [x] Cover seeded grids, forward/reverse endpoint selection, invalid-path
    recovery, completion, and move records with focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 15 — Eighth post-launch cabinet game

- [x] Add Hangman as a sixteenth deterministic, touch-complete cabinet game
  with a seeded quiet word, visible letter buttons, and six wrong guesses.
  - [x] Cover seeded words, correct guesses, duplicate protection, win/loss,
    and reset behavior with focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 16 — Ninth post-launch cabinet game

- [x] Add Connect Four as a seventeenth deterministic, touch-complete cabinet
  game with visible column controls and a bounded local opponent.
  - [x] Cover seeded boards, gravity, four-in-a-row detection, AI replies,
    undo, and reset behavior with focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 9 — Second post-launch cabinet game

- [x] Add Tic-Tac-Toe as a tenth deterministic, touch-complete cabinet game.
  - [x] Add player moves, bounded local AI responses, win/draw detection, undo,
    new-board reset, and focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, and verification captures.
  - [x] Publish the expanded cabinet and game through the standard validation
    path.

## Phase 17 — Tenth post-launch cabinet game

- [x] Add Checkers as an eighteenth deterministic, touch-complete cabinet game
  with mandatory captures, king promotion, chained jumps, and a bounded local
  opponent.
  - [x] Cover seeded setup, legal movement, mandatory captures, promotion,
    bounded replies, chained jumps, undo, and reset behavior with focused rule
    tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 18 — Eleventh post-launch cabinet game

- [x] Add Peg Solitaire as a nineteenth deterministic, touch-complete cabinet
  game on the classic 33-hole cross board.
  - [x] Cover seeded setup, legal jumps, middle-peg removal, win/stuck states,
    undo, and reset behavior with focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 19 — Twelfth post-launch cabinet game

- [x] Add Mahjong Solitaire as a twentieth deterministic, touch-complete
  cabinet game with layered tiles, free-pair matching, undo, and reset.
  - [x] Cover seeded layouts, tile availability, pair removal, undo, and stuck
    state behavior with focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 20 — Thirteenth post-launch cabinet game

- [x] Add Snake as a twenty-first deterministic, touch-complete cabinet game
  with visible turn controls, food growth, collision states, undo, and reset.
  - [x] Cover seeded starts, direction rejection, movement, wall collision,
    food growth, undo, and reset behavior with focused rule tests.
  - [x] Add persisted active state and best-score records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 21 — Fourteenth post-launch cabinet game

- [x] Add Breakout as a twenty-second deterministic, touch-complete cabinet
  game with visible paddle controls, brick collisions, undo, and reset.
  - [x] Cover seeded layouts, paddle steps, brick field setup, wall and paddle
    collision states, undo, and reset behavior with focused rule tests.
  - [x] Add persisted active state and best-score records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 22 — Fifteenth post-launch cabinet game

- [x] Add Higher or Lower as a twenty-third deterministic, touch-complete card
  cabinet game with visible guesses, deterministic draws, undo, and reset.
  - [x] Cover seeded rounds, correct and incorrect guesses, score progression,
    undo, and reset behavior with focused rule tests.
  - [x] Add persisted active state and best-score records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 23 — Sixteenth post-launch cabinet game

- [x] Add Klondike Golf as a twenty-fourth deterministic, touch-complete card
  cabinet game with seven tableau columns, rank-adjacent clears, stock draws,
  undo, and reset.
  - [x] Cover seeded deals, adjacent-rank moves, stock draws, undo, and reset
    behavior with focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 24 — Seventeenth post-launch cabinet game

- [x] Add Blackjack as a twenty-fifth deterministic, touch-complete card
  cabinet game with seeded deals, ace-aware totals, HIT/STAND resolution,
  undo, and new-round reset.
  - [x] Cover seeded deals, soft aces, hit/undo, stand resolution, and finished
    round boundaries with focused rule tests.
  - [x] Add persisted active state and best-win records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 25 — Eighteenth post-launch cabinet game

- [x] Add Spider Solitaire as a twenty-sixth deterministic, touch-complete
  card cabinet game with the standard four-suit, ten-column ruleset.
  - [x] Cover 104-card seeded deals, same-suit descending runs, stock deals,
    completed-run removal, undo, and reset behavior with focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 26 — Nineteenth post-launch cabinet game

- [x] Add Dungeon Sweeper as a twenty-seventh deterministic, touch-complete
  cabinet game with an eight-by-eight trap field, safe first reveal, flag mode,
  exit discovery, undo, and reset.
  - [x] Cover seeded fields, safe starts, flags, trap loss, exit victory, and
    undo behavior with focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 27 — Twentieth post-launch cabinet game

- [x] Add Potion 2048 as a twenty-eighth deterministic, touch-complete cabinet
  game with potion-themed tiles, visible direction controls, a 4096 target,
  undo, and new-brew reset.
  - [x] Cover seeded starts, merges, undo, and the higher target boundary with
    focused rule tests.
  - [x] Add persisted active state and best-score records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 28 — Twenty-first post-launch cabinet game

- [x] Add Tiny Tower Defence as a twenty-ninth deterministic, touch-complete
  cabinet game with five quiet lanes, build and upgrade towers, wave ticks,
  enemy leaks, undo, reset, and an eight-wave target.
  - [x] Cover seeded waves, tower costs, shots, leaks, undo, loss, and the
    target-wave victory boundary with focused rule tests.
  - [x] Add persisted active state and best-wave records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 29 — Twenty-second post-launch cabinet game

- [x] Add One Room Roguelike as a thirtieth deterministic, touch-complete
  cabinet game with a seven-by-seven room, adjacent strikes, enemy turns,
  treasure, potions, undo, reset, and an exit-clear victory.
  - [x] Cover seeded rooms, combat damage, potion recovery, loss, victory,
    undo, and reset behavior with focused rule tests.
  - [x] Add persisted active state and best-score records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 30 — Twenty-third post-launch cabinet game

- [x] Add Daily Dungeon as a thirty-first deterministic, touch-complete
  cabinet game with a seeded daily challenge, hidden six-by-six rooms, three
  runes, one-use traps, hearts, undo, reset, and an exit-clear victory.
  - [x] Cover seeded challenges, rune scoring, trap exhaustion, loss, victory,
    undo, and reset behavior with focused rule tests.
  - [x] Add persisted active state and best-score records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 31 — Twenty-fourth post-launch cabinet game

- [x] Add Dots & Boxes as a thirty-second deterministic, touch-complete
  cabinet game with a four-by-four square board, box claiming, a bounded
  local opponent, undo, and reset.
  - [x] Cover edge validation, box scoring, deterministic opponent turns,
    win/loss boundaries, undo, and reset behavior with focused rule tests.
  - [x] Add persisted active state and best-score records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 32 — Twenty-fifth post-launch cabinet game

- [x] Add Sokoban as a thirty-third deterministic, touch-complete cabinet game
  with a compact crate-pushing room, undo, reset, and a clear-room victory.
  - [x] Cover walking, pushing, blocked crates, win detection, undo, and reset
    behavior with focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 33 — Twenty-sixth post-launch cabinet game

- [x] Add Mancala as a thirty-fourth deterministic, touch-complete cabinet
  game with six sowing pits, store capture, a bounded local opponent, undo,
  reset, and a majority victory.
  - [x] Cover sowing, captures, extra turns, opponent turns, win/loss,
    undo, and reset behavior with focused rule tests.
  - [x] Add persisted active state and best-store records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 34 — Twenty-seventh post-launch cabinet game

- [x] Add Hanoi as a thirty-fifth deterministic, touch-complete cabinet game
  with five disks, tap-to-select peg movement, undo, reset, and a clear-room
  victory.
  - [x] Cover selection, legal and illegal moves, classic-solution win,
    undo, and reset behavior with focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 35 — Twenty-eighth post-launch cabinet game

- [x] Add Number Match as a thirty-sixth deterministic, touch-complete cabinet
  game with a six-by-six adjacent-pair grid, equal/sum-to-ten clears, undo,
  reset, and a clear-grid victory.
  - [x] Cover pair validation, adjacency, clearing, win detection, undo, and
    reset behavior with focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 36 — Twenty-ninth post-launch cabinet game

- [x] Add Flood It as a thirty-seventh deterministic, touch-complete cabinet
  game with an eight-by-eight color field, visible color choices, flood-fill
  region growth, undo, reset, and a move-limited victory.
  - [x] Cover flood expansion, color validation, win/loss boundaries, undo,
    and reset behavior with focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 37 — Thirtieth post-launch cabinet game

- [x] Add Color Sort as a thirty-eighth deterministic, touch-complete cabinet
  game with six visible tubes, four colors, matching pours, undo, reset, and a
  single-color tube victory.
  - [x] Cover seeded layouts, capacity and color validation, grouped pours,
    victory, undo, and reset behavior with focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 38 — Thirty-first post-launch cabinet game

- [x] Add Battleship as a thirty-ninth deterministic, touch-complete cabinet
  game with a six-by-six hidden fleet, visible hit/miss reveals, undo, reset,
  and a five-square victory.
  - [x] Cover seeded layouts, hit/miss validation, repeated shots, victory,
    undo, and reset behavior with focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 39 — Thirty-second post-launch cabinet game

- [x] Add Word Grid as a fortieth deterministic, touch-complete cabinet game
  with five-letter guesses, duplicate-aware feedback, six rows, on-screen
  keyboard controls, undo, reset, and a word-solving victory.
  - [x] Cover input bounds, duplicate scoring, win/loss boundaries, undo, and
    reset behavior with focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 40 — Thirty-third post-launch cabinet game

- [x] Add Pipe Loop as a forty-first deterministic, touch-complete cabinet game
  with a five-by-five rotated path, visible pipe connections, undo, reset, and
  a complete-path victory.
  - [x] Cover seeded scrambling, rotation bounds, victory, undo, and reset
    behavior with focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 41 — Thirty-fourth post-launch cabinet game

- [x] Add Maze Walk as a forty-second deterministic, touch-complete cabinet
  game with a seven-by-seven walled route, visible directional controls, undo,
  reset, and an exit victory.
  - [x] Cover seeded walls, blocked moves, route completion, undo, and reset
    behavior with focused rule tests.
  - [x] Add persisted active state and best-move records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 42 — Thirty-fifth post-launch cabinet game

- [x] Add Match Three as a forty-third deterministic, touch-complete cabinet
  game with a seven-by-seven colored tile field, adjacent swaps, cascades,
  scoring, undo, reset, and a target-score victory.
  - [x] Cover seeded boards, swap validation, match removal, cascades, score,
    undo, victory, and reset behavior with focused rule tests.
  - [x] Add persisted active state and best-score records with snapshot,
    migration, progression, and tutorial coverage.
  - [x] Add responsive desktop, portrait, and medium-landscape presentation,
    visible controls, records, and verification captures.

## Phase 43 — Dungeon Sweeper post-launch expedition

- [x] Refine Dungeon Sweeper with real trap-aware flags and a Minesweeper-style
  chord action that opens neighboring rooms when a revealed clue is satisfied.
  - [x] Cover matching and mismatched flags, revealed-clue taps, chord loss,
    undo history, and flagged-trap persistence with focused rule tests.
  - [x] Update the touch presentation, active flag-mode label, tutorial wording,
    and responsive title/status spacing for desktop, portrait, and landscape.
  - [x] Capture the refined Dungeon Sweeper layouts and verify the full publish
    path after the rules and presentation update.

## Phase 44 — Forty-three-game cabinet capacity polish

- [x] Reflow the desktop, portrait, and medium-landscape cabinet grids so all
  forty-three games remain visible, individually tappable, and separated from
  footer controls.
  - [x] Fit the final game row without clipping or shelf-rail collisions.
  - [x] Keep HELP, RECORDS, and SETTINGS touch targets clear at each documented
    viewport size.
  - [x] Recapture the cabinet matrix and refresh the catalog thumbnail after the
    layout correction.

## Phase 45 — Full-cabinet tutorial persistence audit

- [x] Expand tutorial-seen persistence from the original eight-game array to
  the complete forty-three-game cabinet so late-game tutorials cannot index
  beyond the saved state.
  - [x] Normalize legacy eight-entry collection and profile saves to the full
    game count while preserving existing tutorial history.
  - [x] Cover late-game tutorial indexing, round-trip persistence, and legacy
    save migration with focused state tests.

## Phase 46 — Full-cabinet records capacity polish

- [x] Reflow desktop and medium-landscape records so all forty-three game
  records stay inside their panels after the cabinet expansion.
  - [x] Keep every record label/value pair readable without right-edge clipping.
  - [x] Preserve the portrait two-column layout and clear BACK targets.
  - [x] Recapture records at desktop, portrait, and landscape sizes.

## Phase 47 — Full-cabinet rules guide

- [x] Replace the launch-only Rules copy with a responsive, data-driven guide
  that lists every game in the forty-three-game cabinet.
  - [x] Keep desktop, portrait, and medium-landscape entries inside their
    panels with readable objectives and clear BACK controls.
  - [x] Derive titles and objectives from GameId::ALL so future cabinet games
    cannot be omitted from the guide.
  - [x] Capture the Rules screen at all documented viewport sizes.

## Phase 48 — Current-state documentation alignment

- [x] Update README, storefront metadata, game design, and technical design to
  describe the verified 43-game post-launch cabinet instead of the original
  eight-game/template baseline.
  - [x] Record the current touch, persistence, tutorial, records, and
    responsive-runtime state for future development.
  - [x] Validate storefront JSON and remove stale launch-era claims from the
    active product documentation.

## Phase 49 — Persisted continue flow

- [x] Persist the last-opened cabinet drawer and expose a visible CONTINUE
  action in desktop, portrait, and medium-landscape cabinet layouts.
  - [x] Normalize legacy saves and bound restored selection indices.
  - [x] Cover collection round-trip persistence and verify all three responsive
    cabinet captures with the resume control visible.

## Phase 50 — Post-launch dense-board accessibility

- [x] Extend the persisted High Contrast and Large Text settings to the
  Match Three, Maze Walk, and Dungeon Sweeper boards.
  - [x] Improve board fills, grid lines, symbols, labels, and touch controls
    without changing the underlying game rules.
  - [x] Capture and verify accessible desktop, portrait, and medium-landscape
    scenes for all three dense post-launch boards.

## Phase 51 — Original board accessibility

- [x] Extend the persisted High Contrast and Large Text settings to Lights
  Out, Sliding Puzzle, and Memory / Pairs.
  - [x] Improve board fills, grid lines, symbols, labels, and touch controls
    without changing the underlying game rules.
  - [x] Capture and verify accessible desktop, portrait, and medium-landscape
    scenes for all three board games.

## Phase 52 — Color and number puzzle accessibility

- [x] Extend the persisted High Contrast and Large Text settings to
  Mastermind, Number Match, and Flood It.
  - [x] Improve board fills, grid lines, symbols, labels, and touch controls
    without changing the underlying game rules.
  - [x] Capture and verify accessible desktop, portrait, and medium-landscape
    scenes for all three puzzle games.

## Phase 53 — Symbolic puzzle accessibility

- [x] Extend the persisted High Contrast and Large Text settings to Color
  Sort, Pipe Loop, and Word Grid.
  - [x] Improve board fills, grid lines, symbols, labels, and touch controls
    without changing the underlying game rules.
  - [x] Capture and verify accessible desktop, portrait, and medium-landscape
    scenes for all three symbolic puzzle games.

## Phase 54 — Tabletop and grid accessibility

- [x] Extend the persisted High Contrast and Large Text settings to
  Battleship, Dots & Boxes, and Sokoban.
  - [x] Improve board fills, grid lines, symbols, labels, and touch controls
    without changing the underlying game rules.
  - [x] Capture and verify accessible desktop, portrait, and medium-landscape
    scenes for all three tabletop games.

## Phase 55 — Adventure and room-board accessibility

- [x] Extend the persisted High Contrast and Large Text settings to Potion
  2048, One Room Roguelike, and Daily Dungeon.
  - [x] Improve board fills, grid lines, symbols, labels, and touch controls
    without changing the underlying game rules.
  - [x] Capture and verify accessible desktop, portrait, and medium-landscape
    scenes for all three adventure games.

## Post-launch candidates (not scheduled)

- Puzzles/tabletop: no remaining scheduled candidates.
- Arcade: no remaining scheduled candidates.
- Original cabinet games: no remaining scheduled candidates.

Do not begin a post-launch candidate until the eight-game first collection is
touch-complete and the shared system it would reuse is stable.
