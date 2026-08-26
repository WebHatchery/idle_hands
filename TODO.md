# TODO — Idle Hands

## Release blockers

- [x] Restore the required Clippy gate. `cargo clippy --all-targets
  --all-features -- -D warnings` passes after documenting the intentionally
  flat terminal-result contract and updating the affected tests.
- [x] Replace `catalog_thumbnail.png` with a current title image. The new
  cover frames the cabinet mascots and a player's hand around the game title.

## Release candidate gates

- [x] Add a shipping-WebGL browser smoke test. CI packages the release through
  the shared publisher, serves the deployed game shell, and exercises loading,
  errors, touch, audio activation, resize/fullscreen, persistence, and recovery
  in Chromium.
- [x] Align and document the release identity as `1.0.0`. Cargo and the save
  envelope now agree, the catalog marks the build Playable, and
  `RELEASE_NOTES.md` records the release candidate. Create the release tag only
  after the owner-only distribution and physical-device gates below pass.
- [ ] Finish the approved storefront release. `itch.json` targets
  `webhatchery/idle-hands`, and the Butler dry run validates the free HTML5
  demo plus unrestricted Windows package with analytics disabled. The first
  live upload, itch.io file flags, pay-what-you-want page setting, embed
  options, and Public visibility still require owner authorization and QA.
- [x] Record the available artwork provenance and ship it with the release.
  `assets/THIRD_PARTY_NOTICES.md` ties all three generated assets to their
  introducing commits, records the owner's confirmation that all artwork is
  procedural or AI-generated without third-party licensing requirements, and
  Credits identifies the original generated art.
- [ ] Complete physical iPhone/iPad Safari and Windows acceptance passes. The
  automated browser smoke and screenshot matrix cannot prove shipping behavior
  on those owner-controlled devices.

## Documentation cleanup

- [x] Refresh the public project documents for the release candidate. The
  historical Phase 0 review is explicitly marked as superseded, and the README
  points to the active release, browser, provenance, and owner-only gates.

## Verified baseline — 2026-08-26

- `cargo fmt --manifest-path Cargo.toml -- --check` passes.
- `cargo clippy --all-targets --all-features -- -D warnings` passes.
- `cargo test --all-targets --all-features` passes: 891 tests.
- The full 60-game capture set is present, and the mobile matrix contains 390
  captures across the six supported viewport sizes.
- The required `publish.ps1` and packaged-browser smoke test are the current
  release-candidate validation path.
- No unchecked TODO entries or in-source `TODO`, `FIXME`, `TBD`, or `HACK`
  markers remain outside generated build and distribution output.
