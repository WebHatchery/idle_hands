# TODO — Idle Hands

- [ ] Migrate legacy source-mounted tests and test-only helpers to integration
  suites under the crate's `tests/` directory, using intentional public APIs.
  Remove test declarations from `src/` before expanding those suites (§11.4).
- [ ] Align `tests/feature_budget.rs` with the strong five-case target in
  `CODE_STANDARDS.md` §11.3. Preserve useful regressions, consolidate related
  inputs, and allow a brief justification when distinct coverage needs more
  than five cases; the existing unconditional cap is legacy migration work.

## UI_STYLE review — 2026-09-20

Audit/planning only; no game changes made. Preserve the two migration tasks above
as separate work. No completed checkboxes existed in this file at audit time.
Read AGENTS.md, UI_STYLE.md, CODE_STANDARDS.md, GAME_DEVELOPMENT_GUIDE.md,
README.md, docs/GAME_DESIGN.md and docs/TECHNICAL_DESIGN.md; no project-local
PROJECT_AGENTS.md was found. Apply UI_STYLE §§1–9 to the work below.

Evidence: inspected current source for the cabinet, game routing, shared header,
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

- [ ] **P1 — Establish board-first screen briefs and recompose 2048.**
  Scope: docs/GAME_DESIGN.md screen briefs; src/ui/game_2048.rs::draw_2048,
  responsive_ui.rs::draw_2048, responsive_landscape.rs::draw_2048 and their
  click maps. The desktop board is only 360×380 within a 1280×720 layout,
  pushed right of a permanent 300×160 instruction panel, size selectors and
  scattered controls. The inspected capture confirms that instructions and
  empty space weaken the board's dominance. Product principles exist, but
  explicit decision/focus/action/support/defer/layout/input briefs are missing.
  Record briefs for cabinet, puzzle/card play and live arcade play before
  implementation. Enlarge and center the playable board in the available area;
  group directions, score and Undo/Hint around it. Remove the permanent teaching
  card and “Every move is touch-complete” copy; teach through the existing
  dismissible tutorial and keep a visible Help route. Treat touch directions as
  equally valid actions, not four differently ranked choices. Coordinate rule
  selectors with the next task instead of creating another settings surface.
  Acceptance: board is the immediate focus; normal play has at most 2–3 strong
  attention regions; reclaimed space enlarges play rather than becoming blank.
  Verify initial, full 5×5, large-value, hint, win/loss and restart states at
  1280×720, 1024×768, 390×844, 320×568 and 568×320; exercise swipe and all four
  visible direction controls, Undo, Help and restart cancellation by touch.

- [ ] **P1 — Consolidate game setup and separate it from play and navigation.**
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

- [ ] **P1 — Reflow minimum-size layouts and provide precise dense-board selection.**
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

- [ ] **P1 — Show Tower Defence costs and consequences before spending gold.**
  Scope: src/tiny_tower_defence_ui.rs::draw/kind_button/clicks,
  tiny_tower_defence.rs::tower_cost/build_or_upgrade and
  game_board_dispatch/group_6.rs::apply_game_action_group_6. Kind buttons show
  only names; tapping a cell immediately attempts a paid build/upgrade. Costs
  vary by kind/level, and the dispatcher discards the failed operation's bool.
  The inspected tower capture confirms no visible cost beside these decisions.
  Label build choices with costs and short role explanations on selection;
  show the selected tower's next upgrade cost and effect before commitment,
  with explicit Build/Upgrade and cancellation when inspection is necessary.
  Explain insufficient gold, invalid lanes and maximum level locally. Make
  Start Wave/Resume the phase's clear primary control, with gold/lives nearby;
  reveal enemy-role explanations when those enemies appear rather than adding
  a permanent roster panel. Keep all valid tower choices accessible.
  Acceptance: players can predict gold spent and why an action is unavailable;
  inspection does not spend currency, and wave controls remain obvious.
  Verify build, wave 3/5, paused, insufficient-gold, level-3 and loss states at
  1280×720, 390×844, 320×568 and 568×320. Touch-build each kind, inspect/upgrade,
  cancel, fail an unaffordable purchase, Undo, start and pause/resume a wave.

- [ ] **P2 — Simplify the cabinet into a clear resume-or-browse decision.**
  Scope: src/cabinet_ui.rs::draw_sidebar/recent_games, cabinet_ui/home.rs::draw_home,
  cabinet_ui/library.rs, responsive_cabinet.rs and responsive_landscape_cabinet.rs.
  The desktop capture shows Continue, Favorites/Recent counts, Daily, six category
  cards, a recent strip, sidebar and profile/footer competing for attention.
  Favorites/Recent/Daily appear in multiple places; “Recently played” is populated
  with the first seven games when history is empty, despite a zero Recent count.
  Remove the redundant count cards and duplicate Daily/Recent entry points;
  keep Continue and the category/game collection dominant with one quiet utility
  route. Hide an empty recent strip or honestly label curated starters. Move
  stamps/achievement summaries to Records and remove the loaded “textures” count
  from player-facing home footers. Retain cabinet materials and identifiable game
  objects; remove redundant container borders and PLAY/EXPLORE labels where the
  whole card is already an obvious action. Preserve Finder, favorites, INFO,
  availability labels, filters, Help, Settings and save recovery discovery.
  Acceptance: at most 2–3 strong regions, obvious resume/browse choice, truthful
  history and no technical counters; utility destinations remain discoverable.
  Verify new/returning profiles, populated/empty favorites and history, filtered
  empty shelves and demo-locked drawers at 1280×720, 320×568 and 568×320. Touch
  Continue, browse/page, favorite, inspect, open a locked drawer and return home.

- [ ] **P2 — Replace persistent event prose with contextual feedback.**
  Scope: src/word_ladder.rs::submit/tap_letter/backspace, word_ladder_ui.rs::draw,
  nim_ui.rs::draw, and shared notification/hint presentation. Source confirms
  Word Ladder stores “Good step — keep climbing” and validation errors in one
  persistent message field; typing/backspace do not clear it. Nim permanently
  combines last player/CPU removals with moves and instructions. These findings
  are code-verified; their timed appearance was not tested. Keep current route,
  waypoint, heap counts, turn and rule constraints persistent near the board.
  Show accepted-step/stone-removal feedback briefly on the affected objects,
  clearing stale input errors when the input changes. Preserve a visible
  last-turn/history disclosure where the AI move must be reviewed; keep invalid
  submission reasons until corrected or dismissed and retain result state after
  effects expire. Keep requested hints contextual and readable. Remove ordinary
  fallback tutorial prose once taught, as in the 2048/Solitaire composition work.
  Acceptance: UI describes the current problem after feedback ends, old errors
  do not describe new input, and critical results never depend on a fading toast.
  Verify Word Ladder invalid/edit/valid/waypoint/win/undo and Nim player/AI/undo/
  end states at 1280×720, 320×568 and 568×320, including reduced motion. Touch
  open/dismiss history and Help and confirm the next decision stays understandable.

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
