# TODO — Idle Hands

## Future agent tasks

- [x] Add optional elapsed-time and best-time records to classic games where
      speed improves replay, keeping pause, save, and touch behavior verified.
- [x] Add selectable AI strengths to Tic-Tac-Toe, Connect Four, Checkers, and
      Mancala while preserving deterministic seeded tests and touch controls.
- [x] Add authored level rotation or seeded variants for the intentionally fixed
      Nonogram, Sokoban, and Hanoi openings if more startup variety is desired.
- [x] Replace the vector-only presentation with an authored sprite/texture
      pass where art direction requires it; update `assets/`, the texture
      manifest, the asset registry, and responsive verification captures.
- [x] Add automated portrait and compact-landscape bounds assertions for every
      game, including hit-target overlap checks for shared tutorial/back UI.
- [x] Re-run the full 55-scene capture harness after adding a game or
      difficulty, keeping `GameId::ALL`, snapshots, records, and achievements
      in lockstep.

## Architecture audit follow-up — 2026-08-20

- [x] Split shell state from the 55-game rule store.
- [x] Separate collection index, profile, and active-game persistence, with
      legacy migration retained only on the read path.
- [x] Partition shell actions from game-action handling at the host boundary.
- [x] Centralize cabinet metadata and save keys in `GameDescriptor::ALL`.
- [x] Share game input/render routing through `ui_game_routes`.
- [x] Coalesce dirty autosaves and keep explicit SAVE immediate.
- [x] Move rule directions into the dependency-neutral domain module.
- [x] Validate and quarantine corrupt persistence slots with user-visible
      recovery notices.
- [x] Remove template data and repair the CI file-presence assumption.
- [x] Bound undo histories and derive capture fixture routing from descriptors.

See [the architecture status note](docs/ARCHITECTURE_STATUS.md) for the
implementation map and deliberate remaining enum-host decision.

## Misc shelf expansion — 2026-08-21

- [x] Add five touch-first Misc drawers: Riddle Room, Pattern Vault, Sum
      Circuit, Orbit Order, and Word Forge.
- [x] Give the new drawers deterministic rules, hints, undo, restart, seeded
      persistence, records, achievements, tutorials, and catalog captures.
