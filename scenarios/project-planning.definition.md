# Project Planning Scenario Definition

## Purpose

Capture the reusable Project Planning scenario that turns validated discovery and product direction into a reviewed architecture and downstream-ready implementation planning package before implementation begins.

Use this scenario when a team needs a clear multi-role planning phase rather than ad hoc transition from requirements into coding.

## Scenario Name And Intent

`Project Planning`

This scenario exists to compose the repository's upstream planning roles into one reusable pre-implementation phase that reduces ambiguity before implementation, review, and verification work begins.

## Lifecycle Window And Trigger Conditions

This scenario sits between validated discovery and implementation-oriented delivery work.

Trigger it when:

- validated business requirements or equivalent discovery already exist
- product direction needs to be made explicit before technical design and planning proceed
- architecture and implementation planning need a durable handoff chain rather than scattered notes
- a project or major feature area is about to move from problem framing into execution preparation

## Participating Roles And Why

- [`Business Analyst`](C:/Users/ericw/Projects/orpheum/roles/business-analyst.md)
  - supplies validated business objectives, process understanding, requirements, and a downstream-ready requirements handoff
- [`Product Owner`](C:/Users/ericw/Projects/orpheum/roles/product-owner.md)
  - turns validated needs and feedback context into current product direction and explicit prioritization posture
- [`Solution Architect`](C:/Users/ericw/Projects/orpheum/roles/solution-architect.md)
  - turns validated and prioritized inputs into a reviewed solution shape and architecture handoff
- [`Technical Planner`](C:/Users/ericw/Projects/orpheum/roles/technical-planner.md)
  - turns reviewed architecture into an explicit implementation strategy, sequencing plan, and implementation handoff
- optional [`Security / Compliance Specialist`](C:/Users/ericw/Projects/orpheum/roles/security-compliance-specialist.md)
  - participates when security, compliance, or trust-boundary-sensitive constraints must materially shape planning outputs before implementation starts

## Entry Conditions

- business objectives, process analysis, requirements specification, and requirements handoff already exist, or equivalent validated discovery inputs are available
- the project or feature area is mature enough for product-direction and architecture work
- the participating role packages are available and treated as the source of truth for role-local workflows
- the scenario is being used as a reusable planning phase, not as a live project plan or staffing mechanism

## Target Outputs And Exit Conditions

The scenario completes successfully when the downstream planning package includes:

- explicit product direction and backlog prioritization
- reviewed solution architecture and architecture handoff
- reviewed implementation strategy, sequencing and dependencies, and implementation handoff
- optional reviewed security/compliance posture and downstream handoff when the context requires it

Exit condition:

- downstream implementation-adjacent roles can begin with reviewed architecture, planning outputs, and any materially constraining security/compliance guidance rather than reconstructing intent from discovery and meeting history

## Core Sequence

1. Consume validated discovery and requirements from Business Analyst outputs.
2. Establish current product direction and prioritization posture through Product Owner outputs.
3. Turn validated and prioritized inputs into reviewed architecture through Solution Architect outputs.
4. Turn reviewed architecture into reviewed implementation planning through Technical Planner outputs.
5. Optionally insert security/compliance framing before or alongside architecture and planning when obligations or trust boundaries materially constrain the work.
6. Hand the completed planning package downstream for implementation and verification-oriented work.

## Decision Gates And Human Checkpoints

- product direction must be explicit enough to shape architecture rather than leaving competing priorities unresolved
- architecture review must be explicit before planning outputs are treated as stable
- planning review must be explicit before downstream implementation work relies on the package
- security/compliance review should become an explicit gate when obligations, data sensitivity, or trust boundaries materially affect the project
- human approval remains visible when product, architecture, or compliance tradeoffs exceed the scenario's default authority

## Scenario Constraints And Non-Goals

- This scenario does not replace role-local workflows; it composes them.
- This scenario does not absorb implementation execution, code review, verification execution, or release packaging.
- This scenario does not act as a sprint board, staffing plan, or project-status mechanism.
- This scenario should stay reusable across projects and should not be overfit to a single delivery environment.

## Open Questions And Design Gaps

- The optional position of Security / Compliance Specialist may need a stronger default rule once more secure-delivery scenarios are implemented.
- The boundary between Product Owner direction and late Business Analyst clarification may need a sharper routing rule if repeated usage shows ambiguity.

## Recommended Next Step

Use the Project Planning integration map to make handoffs, dependencies, optional security/compliance participation, and downstream routing explicit.
