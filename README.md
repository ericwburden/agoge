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

Roles in this repository should be Allium-aware even when they are not Allium-first. Each role package should make explicit:

- when the role consumes existing Allium specifications
- when the role helps identify behavior that is ready to become or update an Allium specification
- when the role must route uncertainty back to discovery or specification work instead of improvising behavior

The current Allium capability is already provided by the installed skills:

- [allium](C:/Users/ericw/.codex/skills/allium/SKILL.md)
- [elicit](C:/Users/ericw/.codex/skills/allium/skills/elicit/SKILL.md)
- [tend](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md)
- [distill](C:/Users/ericw/.codex/skills/allium/skills/distill/SKILL.md)
- [propagate](C:/Users/ericw/.codex/skills/allium/skills/propagate/SKILL.md)
- [weed](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md)

This repo does not currently need a separate generic “Allium interaction” skill on top of that package.

## Repository Structure

- [`roles/`](D:/Projects/agoge/roles) stores reusable role templates and role operating patterns.
- [`artifacts/`](D:/Projects/agoge/artifacts) stores reusable scaffolding for plans, reviews, retros, and other repeatable project artifacts.
- [`notes/`](D:/Projects/agoge/notes) stores structured working notes that capture lessons learned, observations, and candidate practices before they become standards.
- [`checks/`](D:/Projects/agoge/checks) stores evaluation and validation patterns for judging workflow quality and adherence to standards.
- [`workflows/`](D:/Projects/agoge/workflows) stores reusable multi-skill workflow definitions.
- [`skills/`](D:/Projects/agoge/skills) stores vendored or imported skill content used as building blocks.

Use the templates in `roles/` when starting a new role definition or defining a new agent operating mode. Use the templates in `workflows/` when you want repeatable orchestration across multiple skills, tools, and validation steps. Use `artifacts/` for reusable artifact formats, `notes/` for maturing lessons into durable guidance, and `checks/` for the criteria that determine whether a workflow is actually working.

The first concrete role in [`roles/`](D:/Projects/agoge/roles) is [`business-analyst.md`](D:/Projects/agoge/roles/business-analyst.md), which is intended for project kickoff and discovery work focused on requirements and process understanding.

The first concrete artifact set in [`artifacts/`](D:/Projects/agoge/artifacts) is also aligned to the Business Analyst role, covering business objectives, process analysis, requirements specification, and downstream requirements handoff. These checked-in files are reusable definitions; live project work should use instantiated copies in a project workspace.

The first concrete check set in [`checks/`](D:/Projects/agoge/checks) is aligned to that same Business Analyst role and artifact chain, acting as a definition-of-done quality gate for BA outputs.

The intended BA lifecycle across this repo is: select an artifact definition, instantiate a project-specific working copy, populate it, run the required checks, remediate failures with the linked skills, and only then hand the output downstream.

The next concrete role in [`roles/`](D:/Projects/agoge/roles) is [`role-builder.md`](D:/Projects/agoge/roles/role-builder.md), which is intended for designing reusable agent roles together with their artifact, workflow, check, skill, and adoption handoff packages.

The intended Role Builder lifecycle in this repo is: role idea or workshop intake, role definition, support-system design, quality review, and adoption handoff. The dedicated `role-*` skills are the primary operating path for that lifecycle, with generic synthesis skills used only as supporting input steps when needed.

In this repository, `Role Builder hardening pass` is the explicit short trigger for the standard Role Builder quality-review step.

The next concrete role in [`roles/`](D:/Projects/agoge/roles) is [`solution-architect.md`](D:/Projects/agoge/roles/solution-architect.md), which is intended for turning validated BA outputs into an explicit solution shape, architectural decision record, reviewed architecture package, and downstream technical handoff.

The next concrete role in [`roles/`](D:/Projects/agoge/roles) is [`technical-planner.md`](D:/Projects/agoge/roles/technical-planner.md), which is intended for turning reviewed architecture and validated requirements into an implementation strategy, explicit sequencing and dependency handling, a reviewed planning package, and downstream implementation handoff.

The next concrete role in [`roles/`](D:/Projects/agoge/roles) is [`qa-verification-lead.md`](D:/Projects/agoge/roles/qa-verification-lead.md), which is intended for turning reviewed requirements, architecture, planning, and implementation context into an explicit verification strategy, traceable coverage matrix, evidence review, and downstream verification handoff.

The next concrete role in [`roles/`](D:/Projects/orpheum/roles) is [`implementation-engineer.md`](D:/Projects/orpheum/roles/implementation-engineer.md), which is intended for turning reviewed planning and architecture direction into a concrete change set, explicit implementation evidence, reviewed implementation-package readiness, and a downstream handoff for review and verification roles.

The next concrete role in [`roles/`](D:/Projects/orpheum/roles) is [`code-reviewer.md`](D:/Projects/orpheum/roles/code-reviewer.md), which is intended for turning a completed implementation package into explicit review scope, concrete findings, an independent review decision, and a downstream handoff for implementation, verification, or release-adjacent consumers.

The next concrete role in [`roles/`](D:/Projects/orpheum/roles) is [`release-handoff-manager.md`](D:/Projects/orpheum/roles/release-handoff-manager.md), which is intended for turning reviewed implementation, review, and verification outputs into explicit release scope, release posture, rollout caveats, and a downstream release or adoption handoff.

The next concrete role in [`roles/`](D:/Projects/orpheum/roles) is [`product-owner.md`](D:/Projects/orpheum/roles/product-owner.md), which is intended for turning validated requirements, product feedback, and delivery learnings into explicit product direction, backlog prioritization, reviewed product decisions, and a downstream handoff for solutioning, planning, and approval work.

The next concrete role in [`roles/`](D:/Projects/orpheum/roles) is [`security-compliance-specialist.md`](D:/Projects/orpheum/roles/security-compliance-specialist.md), which is intended for turning reviewed delivery context, obligations, risk surfaces, and control expectations into explicit security/compliance scope, control mapping, reviewed posture, and downstream handoff guidance.

The next concrete role in [`roles/`](D:/Projects/orpheum/roles) is [`scenario-designer.md`](D:/Projects/orpheum/roles/scenario-designer.md), which is intended for turning a repeatable multi-role activity into an explicit scenario package with participating roles, sequencing, handoffs, integration requirements, reviewed readiness, and downstream adoption guidance.

## Vendored Skills

The [`skills/`](D:/Projects/agoge/skills) directory is imported from the `awesome-codex-skills` project as a squashed Git subtree.

- Upstream source: [ComposioHQ/awesome-codex-skills](https://github.com/ComposioHQ/awesome-codex-skills)
- Fork: [ericwburden/awesome-codex-skills](https://github.com/ericwburden/awesome-codex-skills)

This keeps the skills available inside this repository while preserving a clear path for future refreshes from upstream or from the fork.

## Next Steps

- Define contribution conventions and evaluation criteria for new patterns.
- Decide how local changes to vendored subtree content should be managed and synced.
- Add initial role and workflow patterns using the templates in `roles/` and `workflows/`.
- Add initial standards and connect them to templates, notes, and checks.
