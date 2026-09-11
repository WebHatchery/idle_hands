# TODO — Idle Hands

## Release blockers

- [x] Restore the required Clippy gate. `cargo clippy --all-targets
  --all-features -- -D warnings` passes after documenting the intentionally
  flat terminal-result contract and updating the affected tests.
- [x] Replace `catalog_thumbnail.png` with a current title image. The new
  cover frames the cabinet mascots and a player's hand around the game title.

## Release gates

- [x] Add a shipping-WebGL browser smoke test. CI packages the release through
  the shared publisher, serves the deployed game shell, and exercises loading,
  errors, touch, audio activation, resize/fullscreen, persistence, and recovery
  in Chromium.
- [x] Align and document the release identity as `1.0.0`. Cargo, the save
  envelope, itch configuration, and public release notes agree.
- [x] Publish the approved storefront release at
  `webhatchery/idle-hands`. The public page now serves the free 30-game HTML5
  demo inline and offers the unrestricted 60-game Windows package as
  pay-what-you-want content, with analytics disabled. The exact procedure and
  first-release fixes are recorded in `docs/ITCH_PUBLISHING_GUIDE.md`.
- [x] Record the available artwork provenance and ship it with the release.
  `assets/THIRD_PARTY_NOTICES.md` ties all three generated assets to their
  introducing commits, records the owner's confirmation that all artwork is
  procedural or AI-generated without third-party licensing requirements, and
  Credits identifies the original generated art.
- [ ] Complete physical iPhone/iPad Safari and Windows acceptance passes. The
  automated browser smoke and screenshot matrix cannot prove shipping behavior
  on those owner-controlled devices.

## Documentation cleanup

- [x] Refresh the public project documents for the release. The historical
  Phase 0 review is explicitly marked as superseded, and the README points to
  the active release, browser, provenance, publishing guide, and remaining QA.

## Standards alignment backlog

The 2026-09-12 review against the updated `AGENTS.md` and
`CODE_STANDARDS.md` found these follow-up items:

- [ ] Migrate the test architecture to the crate-level public API pattern.
  This crate is currently binary-only (`src/lib.rs` is absent), while its
  unit tests and test helpers live in `src/**/tests.rs`, `src/game_2048_tests.rs`,
  and inline `#[cfg(test)]` modules. Add `src/lib.rs`, make `main.rs` consume
  the library, move tests and harnesses into the crate's `tests/` directory,
  and expose only intentional public seams while preserving regression
  coverage.
- [ ] Reconcile feature-suite size with §11.3. There are 59 existing source
  test files with more than five cases, including `src/card_hints/tests.rs`
  with 49 cases. Consolidate related cases with table-driven assertions where
  practical and document why any distinct coverage remains above the target.
- [ ] Split sources before they become difficult to scan. Sixteen Rust files
  are already at or above the 600-line planning threshold, led by
  `src/game.rs` (791), `src/game_capture.rs` (790),
  `src/responsive_landscape_games.rs` (774), `src/responsive_library.rs`
  (772), and `src/responsive_landscape_library.rs` (762). Extract cohesive
  responsibilities while keeping the empty source-gate exception list.
- [ ] Refactor functions over the §4.1 100-line maximum, especially
  `Game::begin_capture_scene`, `Game::apply`, `Game::update`,
  `Game::update_records`, `game_board_actions::apply_game_action`,
  `game_variants::cycle`, and the larger screen renderers. Keep drawing,
  routing, and state mutation in separately testable responsibilities.
- [ ] Complete the data-driven migration. `GameData` loads only the general
  and four puzzle configuration groups; most game balance values, variants,
  word lists, achievements, tutorial copy, hints, and player-facing labels
  remain in Rust, with duplicate puzzle defaults in `src/data.rs` and the
  four puzzle engines. Move authored content and tunable values to typed JSON
  under `assets/`, retain game-specific semantic validation, and make
  production construction consistently use the loaded configuration.
- [ ] Audit shared toolkit usage in the UI. Replace duplicate local button,
  hit-testing, text, and palette behavior with the toolkit equivalents where
  they provide the needed semantics; document intentional exceptions for the
  cabinet's established visual treatment.
- [ ] Add the required short `//!` purpose comment to every remaining module.
  The current scan finds 15 non-test runtime modules and 65 test-harness
  modules without module-level documentation; handle the harness files as
  part of the test migration where possible.
- [ ] Clean up unused-code conventions and high-arity APIs. Rename misleading
  `_state` parameters that are actually used, remove or redesign unused
  `_data`, `_game`, and `_mode` parameters, replace the stale
  `#[allow(dead_code)]` on `Game2048::can_move`, and audit all remaining
  `#[allow]` attributes for a nearby rationale. Replace the nine- and
  ten-argument result/analytics helpers with named input structs where that
  improves ownership and readability.
- [ ] Review production `unwrap`/`expect` calls at fallible boundaries. Keep
  only proven internal invariants with an explanatory comment; return
  `Option`/`Result` or degrade visibly for malformed saves, missing assets,
  invalid geometry, and other recoverable runtime conditions.
- [ ] Bring `GAME_DEVELOPMENT_GUIDE.md` examples and checklists into line with
  the updated standards, especially `src/lib.rs` plus crate-level `tests/`,
  toolkit-owned JSON loading, release validation, and release-on-button-up
  interaction semantics.

## Verified baseline — 2026-08-26

- `cargo fmt --manifest-path Cargo.toml -- --check` passes.
- `cargo clippy --all-targets --all-features -- -D warnings` passes.
- `cargo test --all-targets --all-features` passes: 891 tests.
- The full 60-game capture set is present, and the mobile matrix contains 390
  captures across the six supported viewport sizes.
- The required `publish.ps1`, itch dry run, packaged-browser smoke test, Butler
  status check, and public-page playcheck are the release validation path.
- No in-source `TODO`, `FIXME`, `TBD`, or `HACK` markers remain outside
  generated build and distribution output.
