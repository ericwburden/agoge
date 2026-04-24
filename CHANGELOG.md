# Changelog

All notable changes to this project will be documented in this file.

## [1.1.0] - 2026-04-24

### Added

- New `definition-of-done` artifact and check so project planning can produce an explicit standing completion standard for implementation-phase work.
- Technical Planner workflow and role guidance for creating, reviewing, handing off, and quality-checking the Definition of Done as part of `project-planning`.
- Downstream implementation-package fields that preserve Definition-of-Done alignment and status through implementation handoff and downstream review or verification preparation.
- Release notes for this minor version at `docs/release/1.1.0.md`.

### Changed

- Bumped the workspace crates from `1.0.0` to `1.1.0`.
- `project-planning` now treats Definition of Done as a first-class planning output instead of leaving implementation completion rules implicit.
- Technical Planner traceability and boundary checks now verify the distinction between project-level Definition of Done rules and slice-local exit criteria.
- Implementation planning and implementation-package artifacts now carry Definition-of-Done context forward so “implemented” is less likely to be mistaken for “done.”

## [1.0.0] - 2026-04-23

### Added

- New `crates/orpheum-catalog` crate that bundles the canonical Orpheum catalog directly into the shipped CLI.
- CLI integration tests that simulate an installed binary running outside the Orpheum repo to verify embedded-catalog behavior.
- Release notes for this major version at `docs/release/1.0.0.md`.

### Changed

- Bumped the workspace crates from `0.3.0` to `1.0.0`.
- Catalog resolution now falls back to the embedded shipped catalog when no explicit path, local config, environment override, or runtime-discovered repo checkout is available.
- `orpheum init`, `orpheum update`, `orpheum scenario ...`, and `orpheum doctor` now work on a clean machine without requiring a separate catalog checkout.
- `orpheum init` and `orpheum update` now persist `.codex/orpheum/config.json` only when an external catalog override is selected; embedded-catalog projects do not need local config state.
- Consumer-facing docs and onboarding guidance now describe external catalog paths as override/development mechanisms rather than mandatory prerequisites.

### Fixed

- The no-catalog recovery path for installed binaries no longer dead-ends with `CATALOG_NOT_FOUND` on fresh machines.
- `orpheum update` now has a meaningful default recovery path because the CLI always has a built-in catalog available.

## [0.3.0] - 2026-04-23

### Added

- Explicit artifact-scope layering guidance across Product Owner, Solution Architect, and Technical Planner templates, workflows, scenario contracts, and checks so enduring direction stays distinct from current-slice execution detail.
- `orpheum update` as the explicit refresh path for initialized projects when local guidance or active-session metadata should be refreshed.
- Session-level CLI version recording in `.orpheum/session.json` via `last_orpheum_cli_version`.
- Release notes for this minor version at `docs/release/0.3.0.md`.
- Repo `.gitignore` coverage for generated local state and build output.

### Changed

- Bumped the workspace crates from `0.2.0` to `0.3.0`.
- `delivery-slice-planning` now states much more explicitly which artifacts should remain broad and which artifacts should carry the currently selected bounded slice.
- Product, architecture, and planning checks now verify enduring-context versus current-slice separation instead of leaving that distinction implicit.
- Active-session commands now warn when the running Orpheum CLI is newer than the session's recorded CLI version and direct users to `orpheum update`.
- `orpheum init` and `orpheum update` now refresh the recorded CLI version for active sessions.

### Fixed

- Scope bleed during slice planning is now harder to introduce because slice-local statements are guided downward into prioritization and planning artifacts instead of overwriting broader product or architecture artifacts.
- Older sessions created before CLI-version recording now trigger an explicit refresh warning instead of silently proceeding with stale local guidance expectations.

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
- Delivery-slice-planning and Product Owner guidance now explicitly protect artifact-scope layering so enduring direction stays broad while slice-specific detail is routed into backlog, review, handoff, and planning artifacts.
- Architecture and planning guidance now explicitly separate enduring system-architecture artifacts from slice-execution artifacts so slice planning does not bleed upward into solution architecture.

### Fixed

- Artifact status is now persisted as `pending`, `present`, `verified`, or `failed` instead of loose string values.
- Unevaluable checks now surface as `not_evaluable_in_v1` instead of low-information `skipped`.
- Frontmatter parsing now tolerates mixed line endings in catalog markdown, which prevents false `INVALID_METADATA` failures.
