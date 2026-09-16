# Test coverage policy

Idle Hands has a hard limit of **five `#[test]` cases per cohesive feature**,
with no exceptions. Five is a ceiling, not a quota. When a feature is full,
keep the stronger regression and remove a weaker case. This project-specific
rule supersedes the flexible target in the shared standards.

A feature is a responsibility, not a file or test executable:

- Each game rule engine has one budget across `tests/legacy/` and its
  `tests/game_harness/` long-session case. Variants, hints, undo, terminal
  states, and game-specific save compatibility spend that same budget.
- Game input/layout contracts have a separate responsibility from game rules.
  Equivalent viewport inputs use one case where the assertions are identical.
- Shared host metadata, deterministic construction, and variant routing use
  catalog-driven cases. They are not repeated as 60 separate named cases.
- Collection features such as settings, favorites, profiles, notices, and
  rules browsing share their budgets across data, UI, and responsive modules.
- Snapshot persistence, completion records, content validation, and the
  browser smoke contract cover distinct boundaries. The five 2048 completion
  cases protect persistent achievement recording, not tile movement rules.

`tests/feature_ownership.json` records these decisions, including individual
case ownership where an old module contains several responsibilities.
`cargo test --test feature_budget` scans both source and test directories,
rejects missing/stale ownership, and counts disabled/ignored source cases as
well. The gate also runs under ordinary `cargo test` and the existing CI jobs.
Reviewers must still reject artificial feature splits and unrelated checks
bundled into one case: a count check cannot establish semantic cohesion.

## Selection on 2026-09-16

The suite was reduced from 1,093 to 537 Rust cases across 150 responsibilities,
all capped at five. All 60 games retain their distinct long-session scenario.
Selection favored moves with observable consequences, terminal transitions,
undo restoration, bounded generated solutions, and specific regressions over
constructor smoke checks, copy/constants, duplicate seed checks, and repeated
layout wrappers. Examples include FreeCell supermove capacity, safe first
reveals, Sokoban deadlocks, multi-step undo, and indefinite word/number rounds.

This intentionally removes some distinct edge and legacy-save checks as well
as redundant coverage. It does not claim identical assertion or branch
coverage. The retained snapshot cases cover variant serialization, independent
game restoration, old collection defaults, and valid/malformed save indexes.
The five 2048 completion-record regressions remain intact.

The artificial `responsive_bounds` fixture was removed: its rectangles were
unrelated to the production layout and identical for every game. Shared tap
routing and actual game layout assertions remain. Shared host checks are
separated by behavior rather than retaining the old omnibus contract helper.

`scripts/test_game_suites.ps1` now runs the full test set once. Its previous
per-game exact filters included a nonexistent `game_harness::` prefix and
matched zero tests before the final full run.

## Remaining migration work

Legacy suites are still loaded into private source modules through existing
`#[cfg(test)]` path declarations, and three existing tests remain inline in
`src/`. The public-API test migration required by shared standards section
11.4 is **not complete**. This pruning change does not widen private APIs or
pretend that moving a file alone completes that migration.

The source-size gate continues to enforce the 800-line hard limit. The single
Playwright shipping-browser smoke test remains unchanged.

## Validation

- `scripts/test_game_suites.ps1`: all 537 default-feature cases passed.
- `cargo test --all-targets --all-features`: all 537 demo-feature cases passed.
- Strict Clippy across all targets/features and formatting checks passed.
- Temporary ownership mutations confirmed that a sixth case spanning test
  targets and missing ownership both fail the budget gate; mutations were
  restored before the final suite run.
- `publish.ps1` without parameters built and packaged Windows and WebGL,
  deployed to Preview, and updated the publish tracker successfully.
