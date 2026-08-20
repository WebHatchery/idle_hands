# Architecture audit status

The ten items from the 2026-08-20 senior architecture audit are complete for
the current closed 47-game collection. The project remains an enum-owned host,
but its boundaries are now explicit and tested.

1. `AppState` owns shell/profile state; `GameStore` owns all game rules and
   game-specific presentation state.
2. `CollectionIndex`, `ProfileSave`, and `GameSnapshot` are authoritative in
   separate toolkit slots. The old monolithic `CollectionSave` remains only as
   a read-side migration format.
3. Shell routing is identified by `game_actions::is_shell`; game rule actions
   enter the dedicated `apply_game_action` boundary.
4. `GameDescriptor::ALL` owns titles, subtitles, save keys, categories,
   completion strategy, activity, and variant metadata.
5. `ui_game_routes` is the shared game-surface route for both clicks and draw;
   `Screen` exposes the common game identity helper, avoiding parallel route
   ownership in the host.
6. Autosave requests set a dirty flag and flush after a short coalescing delay;
   explicit SAVE still flushes immediately.
7. Rule modules use dependency-neutral `domain::Direction`; navigation state
   no longer supplies game-rule primitives.
8. Index validation rejects malformed shell state. Load failures report the
   affected slot and quarantine its raw bytes before another save can replace
   them.
9. Template action/config data was removed, and CI now checks `publish.ps1`
   instead of assuming an `index.html` owned by this repository.
10. Game undo histories use the bounded `UndoStack`; capture scene routing is
    derived from the descriptor registry and has coverage tests.

## Deliberate remaining shape

The collection is closed and stable, so the host still uses an enum rather
than object-safe game traits. That keeps serialization, deterministic tests,
and touch routing straightforward. New game work should add a descriptor,
snapshot variant, record slot, rule module, UI route, tutorial, and capture
fixture together; the existing coverage tests are the lockstep guard.

## Verification contract

Run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features
-- -D warnings`, `cargo test --all-targets`, and `./publish.ps1` from this
project directory before handing off a release.
