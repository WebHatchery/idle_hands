# Idle Hands test budget

No more than five `#[test]` cases per cohesive feature. This is a hard cap,
with no exceptions. Keep the strongest five; adding a regression means
replacing a weaker case when the feature is full.

- Count all files and test targets together. Each game's long-session case
  consumes a slot in that game's rule-engine budget.
- Use tables only for equivalent inputs to the same behavior. Do not rename
  excess tests into helpers or bundle unrelated behaviors to conceal the count.
- Do not invent smaller features or split files to obtain additional slots.
- Review `tests/feature_ownership.json` when changing coverage. It names the
  responsibilities and joins suites that share one budget. New files must be
  assigned to an existing feature unless they implement a genuinely separate
  responsibility.
- `cargo test --test feature_budget` enforces ownership and the hard cap.
  `docs/TEST_COVERAGE.md` explains the retained coverage and tradeoffs.

This project-specific rule takes precedence over the flexible target and
exception language in the shared standards. Shared workspace standards are
maintained in `rust_management/docs/`; do not edit their local synced copies.
