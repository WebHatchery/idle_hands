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

## Verified baseline — 2026-08-26

- `cargo fmt --manifest-path Cargo.toml -- --check` passes.
- `cargo clippy --all-targets --all-features -- -D warnings` passes.
- `cargo test --all-targets --all-features` passes: 891 tests.
- The full 60-game capture set is present, and the mobile matrix contains 390
  captures across the six supported viewport sizes.
- The required `publish.ps1`, itch dry run, packaged-browser smoke test, Butler
  status check, and public-page playcheck are the release validation path.
- No unchecked TODO entries or in-source `TODO`, `FIXME`, `TBD`, or `HACK`
  markers remain outside generated build and distribution output.
