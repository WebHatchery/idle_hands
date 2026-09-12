# TODO — Idle Hands

- [ ] Complete the physical iPhone/iPad Safari and Windows acceptance passes.
  Automated browser coverage cannot prove behavior on owner-controlled devices.

- [ ] Finish migrating legacy private unit suites and capture harnesses from
  src/**/tests.rs and src/game_harness/** into tests/, or expose only the
  intentional public seams they require while preserving regression coverage.

- [ ] Complete the data-driven migration. Move the remaining authored content
  and tunable values—variants, word lists, achievements, tutorial copy, hints,
  labels, and balance—from Rust into typed JSON under assets/, with
  game-specific semantic validation and one production construction path.

- [ ] Continue decomposing the remaining large sources and functions to the
  planning threshold. Prioritize responsive_landscape_games.rs,
  responsive_library.rs, responsive_landscape_library.rs,
  game_progression.rs, game_variants.rs, begin_capture_scene, update_records,
  game_variants::cycle, and the larger UI renderers.

