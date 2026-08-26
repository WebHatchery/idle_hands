# Idle Hands 1.0.0

Idle Hands 1.0.0 is the first release-candidate build of the complete cabinet.
It contains 60 replayable card, logic, board, word, arcade, and miscellaneous
games in one touch-first collection.

## Highlights

- Complete visible touch controls and replayable tutorials across all 60 games.
- Responsive portrait, compact-landscape, desktop, and fullscreen layouts.
- Per-game autosaves, collection records, achievements, favourites, and recent
  games stored locally on the player's device.
- Procedural sound feedback with browser user-activation handling.
- Project-specific procedural and AI-generated artwork with no third-party art
  licence or attribution requirements.
- Deterministic deals and puzzles where practical, plus visible undo, restart,
  hint, pause, or other recovery controls appropriate to each game.
- WebGL and Windows packages built from the same Rust and macroquad codebase.

## Release identity

The Cargo package, runtime save envelope, and public release notes use version
`1.0.0`. Existing `1.0.0` saves remain compatible because this release does not
change the save schema version.

## Remaining distribution approvals

The build may be presented as playable, but publishing to itch.io still needs
the owner-approved `owner/game` target and authenticated storefront access.
Physical iPhone/iPad Safari and Windows acceptance passes also remain owner QA
gates and are not represented as completed by automated testing.
