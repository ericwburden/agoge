# Technical Planner Skill Sourcing

## Purpose

Capture which local skills already support the [`Technical Planner`](D:/Projects/agoge/roles/technical-planner.md) role, which gaps are intentionally being handled inside the first package shape, and which future additions are most likely if real planning work exposes friction.

## Local Skill Support

### Useful As-Is

- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md)
  - Useful for normalizing planning workshops, dependency reviews, architecture-to-implementation working sessions, and rollout discussions before the planning artifacts are drafted.
- [`content-research-writer`](D:/Projects/agoge/skills/content-research-writer/SKILL.md)
  - Useful when sequencing or migration choices depend on external standards, platform constraints, rollout patterns, or comparison research that should be sourced explicitly.

### Useful With Existing Local-Markdown Positioning

- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md)
  - Useful for synthesizing local architecture notes, requirement context, dependency references, rollout constraints, and planning assumptions before implementation strategy or handoff work begins.
- [`spec-to-implementation`](D:/Projects/agoge/skills/spec-to-implementation/SKILL.md)
  - Useful as the core local-Markdown planning skill because it already translates mature requirements into implementation-oriented structure, phases, dependencies, and task-ready framing.
- [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md)
  - Useful for packaging reviewed planning outputs into a downstream-ready implementation handoff without inventing a new handoff skill too early.

## New Repo-Native Skills Added

None in the first Technical Planner package.

That is an intentional design choice rather than a missing implementation.

The current package is using the existing local-Markdown planning and packaging skills first, with role-specific behavior embedded in:

- the [`Technical Planner`](D:/Projects/agoge/roles/technical-planner.md) role definition
- the Technical Planner artifact set in [`artifacts/`](D:/Projects/agoge/artifacts)
- the Technical Planner workflow set in [`workflows/`](D:/Projects/agoge/workflows)
- the Technical Planner check set in [`checks/`](D:/Projects/agoge/checks)

## Why Existing Skills Were Enough For V1

### `spec-to-implementation` already covers the main planning method

The Technical Planner role needed a repeatable way to turn reviewed architecture and validated requirements into:

- an implementation approach
- work decomposition
- dependency handling
- sequencing
- risk-aware downstream framing

`spec-to-implementation` already provides most of that underlying method in local Markdown form.

The role package therefore adapts that skill into a tighter role-specific operating model instead of adding a new planning skill before real usage proves the gap.

### Planning review can be expressed through artifacts and checks first

The main new requirement was not "generate a plan" in the abstract. It was:

- review the plan explicitly
- preserve role boundaries
- make traceability across BA -> architecture -> planning explicit
- package the plan for downstream implementation and verification roles

Those concerns are being handled in v1 through:

- [`implementation-plan-review.md`](D:/Projects/agoge/artifacts/implementation-plan-review.md)
- [`implementation-handoff.md`](D:/Projects/agoge/artifacts/implementation-handoff.md)
- [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md)
- [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md)

That keeps the package lean while still making the role operational.

### A dedicated handoff skill would be premature duplication

The existing [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md) skill already expresses the right downstream packaging posture:

- summarize what is being handed off
- preserve traceability and context
- separate verified or ready content from open questions
- avoid drifting into implementation ownership

The Technical Planner handoff needed different content, not a fundamentally different packaging method.

## Role-Specific Design Decisions

### Keep Technical Planner separate from Solution Architect

The package intentionally keeps architecture direction and execution planning separate.

The planner is allowed to:

- decide slice shape
- decide ordering
- surface enabling work
- identify spikes and decision gates

The planner is not supposed to:

- redefine requirements
- silently re-architect the system
- treat unresolved architecture questions as settled

This boundary is enforced in the role definition and in [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md).

### Keep Technical Planner separate from project-management administration

The package intentionally stops short of sprint-board ownership, staffing management, and routine delivery-status tracking.

The goal is technical execution clarity, not a generic project-manager role.

This is why the package emphasizes:

- implementation strategy
- sequencing and dependencies
- readiness conditions
- review and downstream handoff

instead of backlog grooming or day-to-day delivery control.

### Keep the first package artifact-first

The package was derived from artifact responsibilities first:

- implementation strategy
- sequencing and dependencies
- plan review
- implementation handoff

The workflows and checks then follow that artifact chain.

This matches the broader repo pattern established by Business Analyst and Solution Architect and keeps the role easier to validate and hand off.

## Allium Relationship

The Technical Planner role should treat Allium as an upstream behavioral constraint, not as the format for planning artifacts.

That means:

- consume existing Allium when it materially constrains sequencing or verification-sensitive implementation work
- route unstable behavior back upstream rather than inventing it in planning
- use [`propagate`](C:/Users/ericw/.codex/skills/allium/skills/propagate/SKILL.md) only when specification maturity should influence downstream verification direction

The planning artifacts themselves remain Markdown-first in this repository.

## Implementation Decision

For this repository, the right move is:

- keep the Technical Planner role repo-neutral
- reuse the existing local-Markdown planning and handoff skills before introducing new dedicated planner skills
- embed planning review, traceability, and boundary discipline directly in the artifacts, workflows, and checks
- keep the role focused on execution structure rather than project-management administration

## Follow-Up Considerations

If future Technical Planner work becomes repetitive or exposes consistent friction, the next likely additions would be:

- a dedicated planning-review skill for producing stronger readiness findings and remediation routing
- a dedicated sequencing or dependency-mapping skill if cross-workstream ordering turns out to need richer support than `spec-to-implementation` provides
- a stronger bridge to QA or verification once the repo has a dedicated verification role package

For now, those concerns are handled adequately by the current artifact chain and existing skills.
