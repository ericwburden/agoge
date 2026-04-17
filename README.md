# agoge

`agoge` is a training ground for Codex instructions, skills, workflows, and supporting delivery patterns.

The project exists to collect lessons learned across different projects and turn them into a cohesive software delivery lifecycle for AI-assisted work. The emphasis is on repeatable practice, clear operating conventions, reusable scaffolding, and pragmatic evaluation of what actually improves outcomes.

## Goals

- Capture effective instruction and workflow patterns.
- Turn one-off project lessons into reusable operating guidance.
- Create a safe place to test AI-assisted SDLC approaches.
- Document conventions that improve reliability, speed, and collaboration.

## Initial Scope

- Project-level guidance for agents working in this repository.
- A place to develop and refine reusable instructions and workflows.
- Documentation-first setup so future structure can evolve intentionally.

## Product Specifications

Product specifications in this project should prefer [Allium](https://github.com/juxt/allium) over ad hoc prose or loosely structured requirements notes.

Allium is installed globally in the local Codex environment and should be treated as the default specification format when defining expected product behavior.

## Repository Structure

- [`agents/`](D:/Projects/agoge/agents) stores reusable `AGENTS.md` templates and agent operating patterns.
- [`artifacts/`](D:/Projects/agoge/artifacts) stores reusable scaffolding for plans, reviews, retros, and other repeatable project artifacts.
- [`notes/`](D:/Projects/agoge/notes) stores structured working notes that capture lessons learned, observations, and candidate practices before they become standards.
- [`checks/`](D:/Projects/agoge/checks) stores evaluation and validation patterns for judging workflow quality and adherence to standards.
- [`workflows/`](D:/Projects/agoge/workflows) stores reusable multi-skill workflow definitions.
- [`skills/`](D:/Projects/agoge/skills) stores vendored or imported skill content used as building blocks.

Use the templates in `agents/` when starting a new project or defining a new agent operating mode. Use the templates in `workflows/` when you want repeatable orchestration across multiple skills, tools, and validation steps. Use `artifacts/` for reusable artifact formats, `notes/` for maturing lessons into durable guidance, and `checks/` for the criteria that determine whether a workflow is actually working.

The first concrete role in [`agents/`](D:/Projects/agoge/agents) is [`business-analyst.md`](D:/Projects/agoge/agents/business-analyst.md), which is intended for project kickoff and discovery work focused on requirements and process understanding.

The first concrete artifact set in [`artifacts/`](D:/Projects/agoge/artifacts) is also aligned to the Business Analyst role, covering business objectives, process analysis, requirements specification, and downstream requirements handoff.

## Vendored Skills

The [`skills/`](D:/Projects/agoge/skills) directory is imported from the `awesome-codex-skills` project as a squashed Git subtree.

- Upstream source: [ComposioHQ/awesome-codex-skills](https://github.com/ComposioHQ/awesome-codex-skills)
- Fork: [ericwburden/awesome-codex-skills](https://github.com/ericwburden/awesome-codex-skills)

This keeps the skills available inside this repository while preserving a clear path for future refreshes from upstream or from the fork.

## Next Steps

- Define contribution conventions and evaluation criteria for new patterns.
- Decide how local changes to vendored subtree content should be managed and synced.
- Add initial agent and workflow patterns using the templates in `agents/` and `workflows/`.
- Add initial standards and connect them to templates, notes, and checks.
