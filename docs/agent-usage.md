# Agent Usage

This guide is the consumer-facing contract for agents working in a project that uses `orpheum`.

Use it when:

- the project has the `orpheum` CLI available
- a local Orpheum skill was installed with `orpheum init`
- a project may already have an active `.orpheum/` session

## Purpose

Orpheum gives an agent a small, explicit control surface for scenario-driven SDLC work.

Use Orpheum to:

- discover available scenarios
- inspect scenario structure before applying one
- create an active project-local session
- read authoritative state from `.orpheum/`
- generate the current recommended prompt
- run scenario checks before claiming work is ready

## Authoritative Sources

Prefer these sources in order:

1. `orpheum ... --json`
2. `.orpheum/session.json`
3. `.orpheum/scenario.json`
4. `.orpheum/state.json`
5. `.orpheum/logs/checks.json`

Treat these as derived views rather than source of truth:

- `.orpheum/ACTIVE.md`
- `.orpheum/prompts/current.md`
- surrounding catalog prose
- stale chat context

## Normal Agent Loop

When there is no active session:

1. `orpheum scenario list --json`
2. `orpheum scenario show <id> --json`
3. `orpheum scenario apply <id> --json`

When there is an active session:

1. `orpheum status --json`
2. `orpheum prompt current --json`
3. perform the current slice of work
4. `orpheum check run --json`
5. `orpheum status --json` again if you need refreshed session state

When a session is finalized and `status --json` reports `cleanup_ready: true`:

1. `orpheum session close --json`
2. `orpheum scenario apply <id> --json`

When the environment looks wrong:

1. `orpheum doctor --json`

When the CLI warns that local Orpheum guidance is stale:

1. `orpheum update --json`

## Session Files

When a scenario is active, expect these files under `.orpheum/`:

- `ACTIVE.md`
- `session.json`
- `scenario.json`
- `state.json`
- `prompts/current.md`
- `logs/checks.json`

Use them like this:

- `session.json`: active session identity and project-local control metadata
- `scenario.json`: resolved snapshot of the applied scenario
- `state.json`: mutable workflow, artifact, and check progress
- `logs/checks.json`: latest check report and summarized outcomes

## What `orpheum init` Does

`orpheum init` is the project-onboarding step for agents.

It:

- installs a local skill at `.codex/skills/orpheum/SKILL.md`
- uses the embedded catalog by default
- persists `.codex/orpheum/config.json` only when an external catalog override is active
- writes a repo-root `ORPHEUM.md` onboarding file
- appends `.orpheum/` to an existing `.gitignore` if that line is missing

It does not:

- apply a scenario
- create `.orpheum/` session files
- infer or select an active scenario
- create a new `.gitignore` when none exists

`orpheum update` uses the same refresh path for an already initialized project and is the recommended response when the CLI warns that the active session was created by an older Orpheum version.

## Core Rules

- Prefer `--json` whenever you need authoritative machine-readable state.
- Do not infer scenario dependencies from prose when the CLI or session JSON can tell you directly.
- Do not create `.orpheum/` by hand. Use `orpheum scenario apply <id>`.
- Do not guess the active scenario from nearby docs if `.orpheum/scenario.json` exists.
- Run `orpheum check run --json` before claiming scenario-associated outputs are ready.
- Use `orpheum session close --json` instead of touching `.orpheum/` by hand when a finalized session is ready to be archived safely.
- If current session files disagree with chat context, trust the current session files.

## Recommended Decisions

- If there is no active session, start with `scenario list` or `scenario show`.
- If there is an active session, start with `status --json`.
- If check status is stale or unclear, run `check run --json`.
- If `status --json` says `cleanup_ready` is true, run `session close --json` before applying the next scenario.
- If the project looks partially initialized, run `doctor --json`.
- If you need a project to follow a local development checkout of the catalog, run `orpheum update --catalog <path>`.
- During semantic artifact review for discovery or planning scenarios, use Planning Mode or the host environment's nearest equivalent until decision changes and cross-artifact reconciliation are complete.

## Non-Goals

Do not use Orpheum as a reason to:

- scrape catalog prose for dependency resolution
- silently skip checks
- keep working from stale prompt text when session state has changed
- treat `.orpheum/` as the durable home for finished project artifacts
