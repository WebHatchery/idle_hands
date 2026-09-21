# TODO — Idle Hands

- [x] Migrate legacy test modules and test-only helpers out of `src/**` into
  integration suites under `tests/`. Expose intentional public seams through
  `src/lib.rs`, remove production-module test declarations, and preserve useful
  regression coverage.
- [x] Align `tests/feature_budget.rs` and `tests/feature_ownership.json` with
  `CODE_STANDARDS.md` §11.3's strong five-case target. Consolidate related
  inputs, preserve distinct regressions, and document any feature that still
  needs more than five cases.
- [ ] Complete the live UI review of the remaining games and shell states with
  the capture and browser tools. Check setup, play, result, pause, restart, and
  recovery across the supported viewport matrix, including the dense-board
  sizes; verify CSS canvas scaling and touch-only paths; replace equivalent
  evidence in `docs/verification/`; and run `publish.ps1` after any fixes.
