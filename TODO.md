# TODO — Idle Hands

- [ ] Migrate legacy source-mounted tests and test-only helpers to integration
  suites under the crate's `tests/` directory, using intentional public APIs.
  Remove test declarations from `src/` before expanding those suites (§11.4).
- [ ] Align `tests/feature_budget.rs` with the strong five-case target in
  `CODE_STANDARDS.md` §11.3. Preserve useful regressions, consolidate related
  inputs, and allow a brief justification when distinct coverage needs more
  than five cases; the existing unconditional cap is legacy migration work.

## UI_STYLE review — 2026-09-20

The first board-composition task is complete and its verification captures are
kept in `docs/verification/`. Preserve the two migration tasks above as
separate work. The remaining UI tasks stay in dependency order below.
Read AGENTS.md, UI_STYLE.md, CODE_STANDARDS.md, GAME_DEVELOPMENT_GUIDE.md,
README.md, docs/GAME_DESIGN.md and docs/TECHNICAL_DESIGN.md; no project-local
PROJECT_AGENTS.md was found. Apply UI_STYLE §§1–9 to the work below.

Evidence: inspected source for the cabinet, game routing, shared header,
viewport/pointer conversion, 2048, Solitaire, Sudoku, Nonogram, Tower Defence,
and word-game feedback. Visually inspected these existing files directly under
docs/verification/ (historical evidence, not freshly rendered current builds):
ui_cabinet.png (1280×720), ui_2048.png (1024×768), ui_solitaire.png (1200×670),
ui_portrait_tiny_tower_roles.png and ui_portrait_nonogram_large.png (390×844),
ui_matrix_320x568_cabinet_scrolled.png, ui_matrix_320x568_nonogram.png,
ui_matrix_320x568_sudoku.png, and ui_matrix_568x320_2048.png.
Some older captures lack the current rule card or Finder control; source is the
authority for current behavior. This is a representative audit, not visual
certification of all 60 games. No live game, browser, or physical touch session
was run. No publish run is needed for this TODO-only planning change.

### Verified findings — implementation order

The completed 2048 composition is documented in the screen briefs above. The
desktop board now occupies 560×560 logical units with equal direction controls
and nearby score/recovery actions; compact portrait and landscape layouts keep
the board as the largest play region and preserve matching click maps. The
permanent teaching card and repeated touch-complete copy were removed, while
the replayable TUTORIAL control remains visible. Captures cover initial, hint,
restart-confirmation, high-contrast/large-text, desktop, portrait, compact
landscape, 320×568, 568×320, 844×390, and 1024×768 states. Physical iPhone/iPad
Safari and Windows touch acceptance remain owner-controlled limitations.

- [x] **P1 — Consolidate game setup and separate it from play and navigation.**
  Depends on the screen briefs. Scope: src/game_render.rs::draw_time_badge,
  game_variant_ui.rs, variant_card_data.rs, ui.rs::draw/actions_at,
  tutorial_ui.rs, mobile_tutorial_ui.rs, and the 2048/Solitaire/Sudoku/Nonogram
  renderers and responsive counterparts. Rule cards repeat information already
  shown in local board-size/difficulty selectors or Solitaire's rules subtitle;
  the global TIME/BEST badge adds another bordered region even in untimed games.
  Captures show multiple equally outlined header boxes. Make one readable active
  rules summary open an explicit round-setup disclosure; remove duplicate
  selectors/labels after preserving direct access to every supported choice.
  Relocate New Game/New Deal into that setup area, apart from Undo/Hint and
  board controls. Keep Cabinet/Help in a quiet, separate navigation group;
  keep pause/resume adjacent to live play. Put personal-best time in results/
  Records and retain live time only where it informs the current decision.
  Acceptance: one home for each rules/time fact, clearly distinct play/setup/
  navigation groups, no lost choices or silent replacement of unfinished games.
  Verify default and long variant labels, large text/high contrast, and selected
  states at 1280×720, 320×568 and 568×320. Touch-test changing a variant, Cancel,
  confirmed restart, Help dismissal, Cabinet and Continue with session restored.
  Implemented with the shared ROUND SETUP disclosure, option-specific restart
  confirmation, relocated 2048/Solitaire/Sudoku/Nonogram setup controls, and
  record/result-only personal-best time presentation. Fresh evidence is in
  target/verification_setup_desktop/, target/verification_setup_portrait/,
  target/verification_setup_landscape/ and target/verification_setup_accessible/;
  the browser smoke path covers opening setup, changing board size, canceling,
  canceling a new-board confirmation, and confirming a new board. Physical
  device touch, Help dismissal, and restored-session Continue remain owner-
  controlled limitations.

- [x] **P1 — Reflow minimum-size layouts and provide precise dense-board selection.**
  Depends on the space reclaimed above. Scope: src/ui.rs::layout_size/viewport/
  readable_text_size_for_scale/hit, responsive_sudoku.rs::portrait_board/
  portrait_cell/clicks, responsive_puzzles.rs Nonogram grid/focus controls,
  responsive_landscape_games.rs, nonogram_ui.rs::nonogram_clicks, game_update.rs
  and input.rs. Portrait uniformly fits 360×780 into 320×568 (about 0.728 scale),
  shrinking Sudoku's 32.2-unit cells to about 23.4 pixels. Its board hit test
  directly divides coordinates; general button target expansion does not fix
  adjacent cells. Nonogram's existing 9×9 focus still packs nine cells across
  300 units. Desktop Nonogram HINT directly tests a 28-unit-high rectangle.
  These are code-confirmed target deficits, not observed mistaps. Reflow for
  the actual safe viewport before shrinking; add a visible magnified selection
  or focus mode with unambiguous 44×44 CSS-point targets for dense grids, retain
  whole-board/clue context and a visible return/pan route, and enlarge actual
  button layouts instead of relying only on invisible overlapping hit regions.
  Acceptance: exact cell selection and every required action have a touch-safe
  path, readable clues and labels, and matching render/input coordinates.
  Verify 320×568, 568×320, 390×844, 844×390, 768×1024, 1024×768 and 1280×720 with
  large text and multiple device pixel ratios. Touch-select neighboring Sudoku
  cells, enter/erase notes, fill/cross and pan all corners of a 15×15 Nonogram,
  undo, resize/rotate and repeat. Do not claim success from geometry tests alone.
  Implemented with the visible FOCUS CELLS / FOCUS CELL routes in
  src/dense_focus_ui.rs. Sudoku gets a magnified 3×3 neighborhood, number pad,
  pencil, erase, undo and DONE controls; Nonogram gets the same magnified
  neighborhood plus explicit LEFT/RIGHT/UP/DOWN focus navigation, fill/cross,
  hint, undo and DONE controls. The whole-board/clue views remain the default,
  and focus state is cleared on Escape, game changes and new rounds.
  Fresh evidence is in docs/verification/ui_*_focus*.png and the seven-size
  matrix files for both dense games. The deployed browser touch regression in
  tests/dense-focus-smoke.spec.mjs selects a neighboring Sudoku cell, enters a
  number, pans Nonogram without marking intermediate cells, marks a cell and
  returns to each full-board view. Physical iPhone/iPad Safari and Windows
  touch acceptance remain owner-controlled limitations; Chromium touch
  emulation and the native capture matrix do not certify every device pixel
  ratio.

### Further inspection — not established defects

- [ ] **P2 — Verify live shell layering, camera/input agreement and the remaining games.**
  Depends on the focused changes above; use the existing capture harness in
  src/main.rs, capture_registry.rs and game_capture*.rs plus the browser smoke
  tooling. Inspect src/game_render.rs (rule card/time are drawn after ui::draw
  overlays), ui_game_routes.rs, game_update.rs, input.rs, restart_modal.rs,
  lifecycle_pause_ui.rs and save_recovery_ui.rs. Historical captures cannot prove
  current modal occlusion, safe-area behavior, actual browser canvas scaling or
  touch accuracy. Do not report those as confirmed failures yet. Current play
  uses VirtualUi framing and fixed board rectangles, with Nonogram focus/pan;
  there is no evidence that adding a generic world camera would improve it.
  Capture and interactively inspect normal/first-use, dense, selected/hint,
  paused, result, restart and supported recovery states across the collection,
  prioritizing long card stacks, arcade pressure and word keyboards. Check the
  2–3-region budget, contextual advanced information, control separation,
  object framing and text readability before cosmetic work. For each reproduced
  issue, add a concrete task or merge it into the matching task above; do not
  assume template ancestry merely because a screen uses panels.
  Acceptance: evidence identifies scene/build/viewport, actual CSS canvas size,
  touch path and result, and separately lists untested games/devices. Verify the
  seven-size matrix in the dense-board task, embedded and fullscreen browser,
  rotation/DPR changes, tutorial dismissal, modal cancellation, pause/resume,
  save recovery and return/continue without keyboard or hover. Reuse screenshots
  directly in docs/verification/, replacing equivalent states. Run .\publish.ps1
  without parameters after implementation; report its result and any remaining
  physical iPhone/iPad Safari or Windows verification limitations.
