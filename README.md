# Orpheum

[![crates.io](https://img.shields.io/crates/v/orpheum.svg)](https://crates.io/crates/orpheum)
[![docs.rs](https://docs.rs/orpheum/badge.svg)](https://docs.rs/orpheum)

`orpheum` is a scenario catalog and Rust CLI for applying reusable AI-assisted SDLC workflows to real projects.

Current release documentation:

- [CHANGELOG.md](C:/Users/ericw/Projects/orpheum/CHANGELOG.md)
- [Orpheum 1.1.0 Release Notes](C:/Users/ericw/Projects/orpheum/docs/release/1.1.0.md)

This repository now has two closely related responsibilities:

- maintain the canonical catalog of scenarios, roles, workflows, artifacts, and checks
- provide the `orpheum` CLI that resolves that catalog into lightweight project-local orchestration state under `.orpheum/`

The goal is not to build a heavyweight autonomous orchestrator. The goal is to make AI-assisted delivery more repeatable: explicit scenario selection, explicit artifact expectations, explicit checks, and a small, machine-readable control surface that an agent can use inside an ordinary project repo.

## What Orpheum Is

At a high level, Orpheum has three layers:

1. Catalog layer: this repository contains the reusable SDLC building blocks.
2. Session layer: the CLI applies a selected scenario into a target project and creates a local `.orpheum/` control directory.
3. Project output layer: durable deliverables live in the target project, not in Orpheum.

In practice, that means:

- humans and agents can inspect available scenarios with the CLI
- a scenario can be applied to a project without copying the whole catalog into it
- prompts, state, and checks are derived from machine-readable metadata rather than prose scraping

## Current CLI Surface

The Rust workspace lives at the repo root and currently ships these v1 commands:

- `orpheum init`
- `orpheum update`
- `orpheum scenario list`
- `orpheum scenario show <id>`
- `orpheum scenario apply <id>`
- `orpheum session close`
- `orpheum status`
- `orpheum prompt current`
- `orpheum check run`
- `orpheum doctor`

All implemented commands support `--json`.

The CLI is intentionally metadata-first. It resolves scenarios from YAML frontmatter embedded in catalog Markdown files under:

- [scenarios](C:/Users/ericw/Projects/orpheum/scenarios/README.md)
- [roles](C:/Users/ericw/Projects/orpheum/roles/README.md)
- [workflows](C:/Users/ericw/Projects/orpheum/workflows/README.md)
- [artifacts](C:/Users/ericw/Projects/orpheum/artifacts/README.md)
- [checks](C:/Users/ericw/Projects/orpheum/checks/README.md)

Catalog-dependent commands resolve the catalog in this order:

- `--catalog <path>`
- repo-local `.codex/orpheum/config.json` written by `orpheum init`
- `ORPHEUM_CATALOG=<path>`
- runtime discovery from the current working directory or executable location when you are working from the Orpheum repo itself
- the embedded canonical catalog shipped with the installed CLI

That means `cargo install orpheum` now works on a clean machine without a separate catalog checkout. External catalog paths are override and development mechanisms, not a required second install step.

If you are wiring Orpheum into a local agent workflow, use the dedicated consumer guide:

- [docs/agent-usage.md](C:/Users/ericw/Projects/orpheum/docs/agent-usage.md)

## Quick Start

Install the published CLI:

```bash
cargo install orpheum
```

List shipped scenarios from the embedded catalog:

```bash
orpheum scenario list
```

If you want a project to follow a local development catalog checkout instead, point Orpheum at that repo explicitly:

```bash
orpheum --catalog /path/to/orpheum scenario list
```

Build and run the CLI from this repository:

```bash
cargo run -p orpheum -- scenario list
```

Initialize a consumer project for local agents:

```bash
cargo run -p orpheum -- init
```

Initialize a consumer project and persist an explicit external catalog override:

```bash
cargo run -p orpheum -- init --catalog /path/to/orpheum
```

Inspect a scenario:

```bash
cargo run -p orpheum -- scenario show project-planning
```

Apply a scenario to a target project:

```bash
cargo run -p orpheum -- scenario apply project-planning --project /path/to/project
```

After applying a scenario, the target project gets a small `.orpheum/` control surface:

- `.orpheum/ACTIVE.md`
- `.orpheum/session.json`
- `.orpheum/scenario.json`
- `.orpheum/state.json`
- `.orpheum/prompts/current.md`
- `.orpheum/logs/checks.json`

From the target project root, you can then run:

```bash
orpheum init
orpheum update
orpheum session close
orpheum status
orpheum prompt current
orpheum check run
orpheum doctor
```

`orpheum init` installs or refreshes a project-local skill at `.codex/skills/orpheum/SKILL.md`, uses the embedded catalog by default, persists `.codex/orpheum/config.json` only when an external catalog override is active, writes a repo-root `ORPHEUM.md` onboarding file, and adds `.orpheum/` to an existing `.gitignore` when that entry is missing.

`orpheum update` is the explicit refresh path for existing projects. Use it when a newer CLI warns that local Orpheum guidance should be refreshed.

`orpheum session close` is the explicit safe-close path for finalized sessions. It archives the current `.orpheum/` control directory into `.orpheum-archive/` so a project can move on to the next scenario without manual cleanup.

## Default SDLC Scenario Chain

The current standard path through the catalog is:

1. [Project Discovery](C:/Users/ericw/Projects/orpheum/scenarios/project-discovery.definition.md)
2. [Project Planning](C:/Users/ericw/Projects/orpheum/scenarios/project-planning.definition.md)
3. [Delivery Slice Planning](C:/Users/ericw/Projects/orpheum/scenarios/delivery-slice-planning.definition.md)
4. [Implementation and Release Prep](C:/Users/ericw/Projects/orpheum/scenarios/implementation-and-release-prep.definition.md)
5. [Review Remediation Loop](C:/Users/ericw/Projects/orpheum/scenarios/review-remediation-loop.definition.md) when bounded remediation is needed
6. [Verification And Release Gate](C:/Users/ericw/Projects/orpheum/scenarios/verification-and-release-gate.definition.md) when a distinct final downstream gate is needed
7. [Release Feedback To Reprioritization](C:/Users/ericw/Projects/orpheum/scenarios/release-feedback-to-reprioritization.definition.md)

For security-sensitive or approval-sensitive work, use [Secure Delivery / Secure Feature Lifecycle](C:/Users/ericw/Projects/orpheum/scenarios/secure-delivery-feature-lifecycle.definition.md) instead of the standard chain when those concerns need to shape the work end to end.

For designing and hardening scenarios themselves, see [Scenario Implementation](C:/Users/ericw/Projects/orpheum/scenarios/scenario-implementation.definition.md).

## Repository Structure

- [crates/orpheum-cli](C:/Users/ericw/Projects/orpheum/crates/orpheum-cli) contains the `orpheum` binary and command-line interface.
- [crates/orpheum-catalog](C:/Users/ericw/Projects/orpheum/crates/orpheum-catalog) contains the embedded canonical catalog bundled into the shipped CLI.
- [crates/orpheum-core](C:/Users/ericw/Projects/orpheum/crates/orpheum-core) contains catalog loading, dependency resolution, session management, prompt generation, checks, and doctor logic.
- [scenarios](C:/Users/ericw/Projects/orpheum/scenarios/README.md) contains reusable multi-role SDLC scenarios.
- [roles](C:/Users/ericw/Projects/orpheum/roles/README.md) contains reusable role packages.
- [workflows](C:/Users/ericw/Projects/orpheum/workflows/README.md) contains reusable role-owned workflow definitions.
- [artifacts](C:/Users/ericw/Projects/orpheum/artifacts/README.md) contains reusable artifact definitions and templates.
- [checks](C:/Users/ericw/Projects/orpheum/checks/README.md) contains validation patterns used by roles and scenarios.
- [notes](C:/Users/ericw/Projects/orpheum/notes/README.md) contains design notes, sourcing notes, and other durable working context.
- [skills](C:/Users/ericw/Projects/orpheum/skills/README.md) contains vendored or locally maintained skills used by the catalog.

## Metadata Contract

The CLI does not infer dependencies from prose at runtime.

Instead, the canonical catalog files carry YAML frontmatter that provides the machine-readable contract for:

- scenario dependencies
- workflow inputs and outputs
- role default workflows and skills
- artifact output paths and attached checks
- check execution mode and target artifacts

That frontmatter is the integration contract the CLI relies on. The surrounding Markdown remains the human-facing explanation.

## Validation Model

The v1 check engine is intentionally narrow. It currently supports:

- artifact presence checks
- required heading checks

`orpheum check run` records results in `.orpheum/logs/checks.json` and mirrors summarized status back into `.orpheum/state.json`.

## Product Specification Convention

When this repository defines product behavior, it should prefer [Allium](https://github.com/juxt/allium) over ad hoc prose requirements unless there is a clear reason not to.

That preference still applies even though Orpheum now has a working CLI. The catalog should continue to optimize for reusable structure, explicit reasoning, and durable operating conventions.

## Development Notes

- Workspace root: [Cargo.toml](C:/Users/ericw/Projects/orpheum/Cargo.toml)
- Binary crate: [crates/orpheum-cli/Cargo.toml](C:/Users/ericw/Projects/orpheum/crates/orpheum-cli/Cargo.toml)
- Core crate: [crates/orpheum-core/Cargo.toml](C:/Users/ericw/Projects/orpheum/crates/orpheum-core/Cargo.toml)
- CLI integration tests: [crates/orpheum-cli/tests/cli.rs](C:/Users/ericw/Projects/orpheum/crates/orpheum-cli/tests/cli.rs)

Useful commands:

```bash
cargo fmt
cargo test
cargo run -p orpheum -- doctor --json
```

`orpheum doctor` will report catalog or project-health issues, including whether `.orpheum/` is ignored in the target project.
