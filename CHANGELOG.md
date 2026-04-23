# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-04-23

### Added

- Repo-local catalog configuration at `.codex/orpheum/config.json`, persisted by `orpheum init`.
- Repo-root `ORPHEUM.md` onboarding output that makes project state, catalog source, and next commands visible after initialization.
- Locked-decision and semantic-review sections across planning and handoff artifact templates.
- Required semantic artifact review guidance for `project-discovery`, `project-planning`, and `delivery-slice-planning`, including explicit Planning Mode or equivalent instructions.
- Release notes for this minor version at `docs/release/0.2.0.md`.

### Changed

- Bumped the workspace crates from `0.1.0` to `0.2.0`.
- Expanded catalog resolution precedence to use explicit `--catalog`, repo-local config, `ORPHEUM_CATALOG`, then runtime discovery.
- `orpheum init --json` now reports project state, catalog source/root, onboarding file, and local config file.
- `orpheum doctor --json` now reports project state, local-config status, catalog-source status, and recovery commands even when no active session exists.
- Scenario prompts and contracts now treat semantic artifact review as a required closeout step instead of optional polish.

### Fixed

- Artifact status is now persisted as `pending`, `present`, `verified`, or `failed` instead of loose string values.
- Unevaluable checks now surface as `not_evaluable_in_v1` instead of low-information `skipped`.
- Frontmatter parsing now tolerates mixed line endings in catalog markdown, which prevents false `INVALID_METADATA` failures.
