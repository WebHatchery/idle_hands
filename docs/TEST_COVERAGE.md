# Test coverage notes

The crate keeps one focused suite beside each game, data module, and UI
surface. The 2026-09-12 audit found 58 suites with more than five cases. These
are intentionally separate checks for rules, terminal states, serialization,
input routing, responsive bounds, persistence recovery, and migration
compatibility; they are not copies of one generic happy-path test.

The former 49-case card-hints suite was consolidated into a table-driven
regression test plus two edge-case tests. The remaining larger suites retain
named cases where the setup or failure meaning is materially different. New
features should still target at least five meaningful cases, using tables for
families of equivalent inputs and keeping distinct behavioral boundaries
named.

The crate-level public API migration is complete for the harness and legacy
suites. `tests/public_api.rs` and `tests/undo_stack.rs` exercise intentional
library seams. Cross-game contract, desktop, mobile, and long-session checks
live under `tests/game_harness/`, while the detailed rule and UI suites live
under `tests/legacy/`. Source modules retain only cfg(test) path declarations
so those suites keep private access without placing test source in `src/`.
