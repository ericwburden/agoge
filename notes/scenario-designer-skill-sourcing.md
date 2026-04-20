# Scenario Designer Skill Sourcing

## Purpose

Capture which local skills support the [`Scenario Designer`](D:/Projects/orpheum/roles/scenario-designer.md) role, which gaps are intentionally deferred, and which public orchestration and process patterns informed the package's first-pass support decisions.

## External Pattern Summary

The Scenario Designer package is shaped by five recurring public patterns:

- multi-agent and multi-role systems separate specialist behavior from orchestration or routing logic
- orchestration is expressed through explicit handoffs, selectors, process steps, or flow transitions rather than by assuming agents will coordinate implicitly
- reusable coordination patterns need explicit state, shared-context, or variable flow rather than chat-only continuity
- human checkpoints, approvals, and conditional routing remain first-class parts of the orchestration layer
- process or workflow frameworks treat sequencing, branching, synchronization, and failure routing as design concerns distinct from the work done inside individual participants

These patterns are visible in:

- OpenAI's [Swarm](https://github.com/openai/swarm), which reduces multi-agent orchestration to agents and handoffs and treats delegation as an explicit primitive rather than an implied continuation
- AutoGen's [Handoffs](https://microsoft.github.io/autogen/dev/user-guide/core-user-guide/design-patterns/handoffs.html), [Selector Group Chat](https://microsoft.github.io/autogen/dev/user-guide/agentchat-user-guide/selector-group-chat.html), and [AgentChat overview](https://microsoft.github.io/autogen/stable/user-guide/agentchat-user-guide/index.html), which distinguish specialized agents from team-level selection, handoff, and graph or workflow patterns
- CrewAI's [Flows](https://docs.crewai.com/en/concepts/flows), [Quickstart](https://docs.crewai.com/en/quickstart), and [Hierarchical Process](https://docs.crewai.com/en/learn/hierarchical-process), which treat flows as the owner of state and execution order while crews and agents perform the work inside the steps
- Microsoft's Semantic Kernel [Process Framework](https://learn.microsoft.com/en-us/semantic-kernel/frameworks/process/process-framework) and [Process Framework Core Components](https://learn.microsoft.com/en-us/semantic-kernel/frameworks/process/process-core-components), which define processes as structured sequences of steps with patterns, events, and reusable step composition
- Camunda's [Processes](https://docs.camunda.io/docs/components/concepts/processes/), [BPMN primer](https://docs.camunda.io/docs/components/modeler/bpmn/bpmn-primer/), and [agentic orchestration](https://docs.camunda.io/docs/next/components/agentic-orchestration/), which treat process orchestration as the coordination of people, systems, APIs, human tasks, and AI agents through explicit flow control and job or task handoff

The common pattern is consistent:

- roles or agents own bounded work
- a separate orchestration or process layer owns sequencing and routing
- handoffs, selectors, or process transitions are explicit
- reusable multi-role patterns deserve their own design surface

## Local Skill Support

### Direct Support

- [`scenario-composition`](D:/Projects/orpheum/skills/scenario-composition/SKILL.md)
  - Preferred direct-support skill for turning a repeatable multi-role activity plus the relevant role packages into explicit scenario boundary framing, role participation logic, handoff contracts, branching rules, synchronization points, readiness posture, and revisit logic.
- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md)
  - Preferred synthesis skill for combining role-package context, scenario goals, and public reference material into a durable local scenario package.
- [`pattern-adaptation`](D:/Projects/orpheum/skills/pattern-adaptation/SKILL.md)
  - Preferred adaptation skill for translating tool-coupled or framework-specific orchestration patterns into repo-native scenario structure.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md)
  - Preferred downstream packaging skill for turning reviewed scenario outputs into a reusable handoff.

### Useful As-Is

- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md)
  - Useful for normalizing role-design workshops, scenario-design sessions, or rough lifecycle-planning notes before artifacts are drafted.
- [`content-research-writer`](D:/Projects/orpheum/skills/content-research-writer/SKILL.md)
  - Useful when scenario shape depends materially on external orchestration, workflow, or process references that should be sourced explicitly.

### Allium-Aware Support

- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md)
  - Useful when scenario gates or downstream sequencing materially depend on existing behavioral specifications.
- [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md)
  - Useful when scenario design reveals a real specification gap that should be refined upstream.
- [`propagate`](C:/Users/ericw/.codex/skills/allium/skills/propagate/SKILL.md)
  - Useful when mature behavioral specifications should shape verification, implementation, or release steps inside a scenario.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md)
  - Useful when scenario confidence depends on checking for specification-to-implementation drift at a gate or handoff.

### Upstream Or Adjacent Only

- [`role-definition`](D:/Projects/orpheum/skills/role-definition/SKILL.md)
  - Upstream Role Builder support rather than a default Scenario Designer dependency because scenarios should consume role packages, not redefine them.
- [`role-support-system-design`](D:/Projects/orpheum/skills/role-support-system-design/SKILL.md)
  - Upstream Role Builder support when a scenario reveals that a role package itself needs stronger workflows, checks, or artifacts.
- [`requirements-verification`](D:/Projects/orpheum/skills/requirements-verification/SKILL.md)
  - Useful when scenario gating depends heavily on validated requirements, but still adjacent rather than core to the scenario-design method.

## New Repo-Native Skills Added

- [`scenario-composition`](D:/Projects/orpheum/skills/scenario-composition/SKILL.md)
  - Added as the direct-support skill for the role's most specific repeated method: scenario boundary framing, role participation logic, handoff-contract authoring, branching and synchronization discipline, readiness-posture wording, and revisit-trigger clarity.

The package still reuses the existing local-Markdown synthesis, adaptation, and handoff skills alongside that new role-specific skill.

## Why Existing Skills Were Not Quite Enough

### The missing problem is scenario composition discipline, not a new orchestration engine

This repository already has reusable role packages and role-local workflows.

The main missing need for a Scenario Designer role is not a runtime engine. It is a repeatable way to:

- define the scenario boundary
- select the participating roles and role-owned workflows
- make handoffs and integration requirements explicit
- preserve gates, approvals, and synchronization points
- package the resulting composition for downstream use

The package therefore spends its reusable structure on composition discipline, traceability, review posture, and downstream packaging rather than on implementation of a new automation runtime.

The review of the first pass showed that this method had one role-specific seam that was too important to leave entirely to generic synthesis:

- defining the scenario boundary cleanly
- selecting participating roles and workflows with discipline
- authoring handoff contracts and synchronization points explicitly
- stating scenario readiness and revisit triggers without drifting into project management

### Scenario design can be expressed through artifacts and checks first

The main new requirement was not merely "write a workflow." It was:

- define scenario intent and lifecycle window explicitly
- preserve traceability back to role-owned workflows and artifacts
- map handoffs, shared dependencies, and branching rules
- review the scenario posture explicitly
- package the result for downstream adopters or execution-planning consumers

Those concerns are being handled in v1 through:

- [`scenario-definition.md`](D:/Projects/orpheum/artifacts/scenario-definition.md)
- [`scenario-integration-map.md`](D:/Projects/orpheum/artifacts/scenario-integration-map.md)
- [`scenario-review.md`](D:/Projects/orpheum/artifacts/scenario-review.md)
- [`scenario-handoff.md`](D:/Projects/orpheum/artifacts/scenario-handoff.md)
- [`scenario-traceability.check.md`](D:/Projects/orpheum/checks/scenario-traceability.check.md)
- [`scenario-designer-boundary.check.md`](D:/Projects/orpheum/checks/scenario-designer-boundary.check.md)

That keeps the package operational without prematurely inventing a broad scenario-specific skill before real usage patterns are clearer.

### Public systems support the separate layer even when they do not name a dedicated "Scenario Designer" role

The public implementations do not converge on a role literally named `Scenario Designer`.

They do converge on a separate design surface for:

- orchestration
- delegation
- process modeling
- workflow design
- selector or routing logic
- state and transition management

That is the key reusable method this repository needed to capture.

The package therefore adopts the separate-layer idea and expresses it as a reusable repo-native role instead of importing a framework-specific object model directly.

### A narrow repo-native composition skill is justified, but broader scenario specialization is still premature

The new [`scenario-composition`](D:/Projects/orpheum/skills/scenario-composition/SKILL.md) skill captures the part of the role that was most likely to drift in repeated usage:

- scenario boundary framing
- role participation logic
- handoff-contract and branching-rule authoring
- synchronization-point discipline
- scenario-readiness and revisit-trigger wording

That closes the biggest support gap in the first-pass package without overreaching into runtime execution, simulation, or project management tooling.

### Broader scenario-specific specialization would still be premature duplication

The existing [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) skill already expresses the right synthesis posture.

The existing [`pattern-adaptation`](D:/Projects/orpheum/skills/pattern-adaptation/SKILL.md) skill already expresses the right translation posture for tool-coupled public patterns.

The existing [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) skill already expresses the right downstream packaging posture.

The package still does not need a broad flow-engine, process-modeling, or orchestration-runtime skill.

## Role-Specific Design Decisions

### Keep Scenario Designer separate from Role Builder

The package intentionally keeps role-package design and scenario-package design separate.

The Scenario Designer is allowed to:

- compose role-owned workflows into a reusable multi-role pattern
- define scenario-level sequencing, handoffs, and integration requirements
- review and package the scenario for downstream use

The Scenario Designer is not supposed to:

- redefine the participating roles
- invent missing role-local workflows
- absorb role-package support-system design as a default response

This boundary is enforced in the role definition and in [`scenario-designer-boundary.check.md`](D:/Projects/orpheum/checks/scenario-designer-boundary.check.md).

### Keep Scenario Designer separate from project management and live execution control

The package intentionally keeps reusable scenario design distinct from project-instance delivery management.

The Scenario Designer is allowed to:

- define a reusable lifecycle pattern
- make gates, dependencies, and handoffs explicit
- identify where human checkpoints or approvals matter

The Scenario Designer is not supposed to:

- manage staffing or status
- run the sprint board
- own daily delivery coordination as a default responsibility
- let a reusable scenario package be mistaken for an active project plan

This boundary is enforced in the role definition and in [`scenario-designer-boundary.check.md`](D:/Projects/orpheum/checks/scenario-designer-boundary.check.md).

### Keep the first package artifact-first

The package was derived from artifact responsibilities first:

- scenario definition
- scenario integration map
- scenario review
- scenario handoff

The workflows and checks then follow that artifact chain.

This matches the broader repo pattern established by the existing SDLC roles and keeps the role easier to validate and hand off.

## Allium Relationship

The Scenario Designer role should treat Allium as a behavioral scenario anchor, not as the format for its artifacts.

That means:

- consume existing Allium when scenario entry conditions, handoffs, or gates materially depend on known behavior
- use [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md) when scenario design reveals a real specification gap
- use [`propagate`](C:/Users/ericw/.codex/skills/allium/skills/propagate/SKILL.md) when mature behavioral specifications should shape downstream steps inside the scenario
- use [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when confidence depends on checking for behavioral drift at a scenario-sensitive gate

The artifacts themselves remain Markdown-first in this repository.

## Implementation Decision

For this repository, the right move is:

- keep the Scenario Designer role repo-neutral
- use [`scenario-composition`](D:/Projects/orpheum/skills/scenario-composition/SKILL.md) as the direct-support method for explicit scenario boundary, choreography, and scenario-posture work
- reuse the existing synthesis, adaptation, handoff, and Allium-aware skills around that new role-specific skill
- embed scenario composition, integration discipline, traceability, review posture, and boundary control directly in the artifacts, workflows, and checks
- keep the role focused on reusable multi-role scenario design rather than runtime orchestration, role definition, or project-instance execution management

## Follow-Up Considerations

If future Scenario Designer work becomes repetitive or exposes consistent friction beyond the current support set, the next likely additions would be:

- a stronger scenario-simulation or dry-run support skill if teams repeatedly struggle to validate branching and synchronization assumptions before adoption
- a stronger bridge skill if scenario defects repeatedly originate in gaps between scenario design and role-package evolution

For now, the main role-specific support gap is closed.
