# TODO — Idle Hands

- [ ] Migrate legacy source-mounted tests and test-only helpers to integration
  suites under the crate's `tests/` directory, using intentional public APIs.
  Remove test declarations from `src/` before expanding those suites (§11.4).
- [ ] Align `tests/feature_budget.rs` with the strong five-case target in
  `CODE_STANDARDS.md` §11.3. Preserve useful regressions, consolidate related
  inputs, and allow a brief justification when distinct coverage needs more
  than five cases; the existing unconditional cap is legacy migration work.
