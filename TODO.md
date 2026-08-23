# TODO — Idle Hands

## Release blockers

- [x] Restore the required Clippy gate. `cargo clippy --all-targets
  --all-features -- -D warnings` passes after documenting the intentionally
  flat terminal-result contract and updating the affected tests.
- [x] Replace `catalog_thumbnail.png` with a current title image. The new
  cover frames the cabinet mascots and a player's hand around the game title.

## Release candidate gates

- [ ] Add a shipping-WebGL browser smoke test. CI builds the WebGL binary and
  checks that the `.wasm` file exists, but it does not load the packaged page.
  Serve the `dist/webgl` bundle in a browser and assert that it reaches the
  cabinet with no console or runtime errors.
- [ ] Cut and document a single production version. `Cargo.toml` says `0.1.0`,
  while the runtime save schema says `1.0.0`; `game_page.json` is still marked
  “In Development.” Choose the public release version, align those values,
  write release notes, and create the release tag.
- [ ] Finish the approved storefront package. The itch.io publisher is present
  but cannot run because this project has no `itch.json`. Once the owner
  supplies the storefront target and authorizes release, add the non-secret
  configuration, validate both HTML5 and Windows packages, and publish the
  approved channel(s).
- [ ] Record asset rights and third-party notices before public distribution.
  The three shipped art assets are listed in the texture manifest, but the
  repository has no license/provenance file and the in-game credits name only
  the technology stack. Document each asset's source and licence, then add any
  required notices or attributions to the release materials and Credits screen.

## Documentation cleanup

- [ ] Refresh the public project documents for the release candidate. The
  Phase 0 code review still describes a starter harness and template
  screenshot, and the README still describes this file as completed launch
  work rather than the active release-gate list.

## Verified baseline — 2026-08-23

- `cargo fmt --manifest-path Cargo.toml -- --check` passes.
- `cargo clippy --all-targets --all-features -- -D warnings` passes.
- `cargo test --all-targets` passes: 891 tests.
- The full 60-game capture set is present, and the mobile matrix contains 390
  captures across the six supported viewport sizes.
- `publish.ps1` completed successfully before this review.
- No unchecked TODO entries or in-source `TODO`, `FIXME`, `TBD`, or `HACK`
  markers remain outside generated build and distribution output.
