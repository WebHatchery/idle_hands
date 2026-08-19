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
- [x] Re-run the full 47-scene capture harness after adding a game or
      difficulty, keeping `GameId::ALL`, snapshots, records, and achievements
      in lockstep.
