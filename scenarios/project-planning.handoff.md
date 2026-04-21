# Project Planning Scenario Handoff

## Purpose

Package the reusable `Project Planning` scenario for downstream adopters, workflow authors, or project leads so they can use it without reconstructing the scenario from the Scenario Designer package and role-local workflows.

## Current Scenario Summary

`Project Planning` is a reusable multi-role scenario that starts with validated discovery and current product direction, optionally incorporates security/compliance framing when that posture materially constrains the work, then produces reviewed architecture and reviewed implementation planning before downstream implementation work begins.

Primary participating roles:

- `Business Analyst`
- `Product Owner`
- `Solution Architect`
- `Technical Planner`
- optional `Security / Compliance Specialist`

## Scenario Package Included

- [project-planning.definition.md](C:/Users/ericw/Projects/orpheum/scenarios/project-planning.definition.md)
- [project-planning.integration-map.md](C:/Users/ericw/Projects/orpheum/scenarios/project-planning.integration-map.md)
- [project-planning.review.md](C:/Users/ericw/Projects/orpheum/scenarios/project-planning.review.md)
- supporting rationale in [scenario-recommendations.md](C:/Users/ericw/Projects/orpheum/notes/scenario-recommendations.md)

## Current Readiness Posture

`ready`

This scenario is ready for downstream adoption and local tailoring as a reusable planning-phase scenario.

Limits:

- it is not a live project plan
- it does not replace role-local workflows
- it does not include implementation, code review, verification execution, or release handling
- optional security/compliance participation should remain conditional rather than automatic

## Role And Workflow Routing Guidance

Use the scenario in this broad order:

1. Business Analyst discovery and requirements handoff
2. Product Owner direction, review, and handoff
3. optional Security / Compliance Specialist scoping may begin once reviewed requirements and obligation context are available if those constraints materially shape architecture or planning
4. Solution Architect design, review, and handoff
5. optional Security / Compliance Specialist review and handoff should complete before architecture or planning is treated as settled when that branch materially constrains them
6. Technical Planner planning, review, and handoff

Downstream consumers should preserve:

- explicit review gates between role stages
- traceable handoffs between role-owned artifacts
- the distinction between product direction, architecture, and planning
- the rule that security/compliance constraints must land before architecture or planning is treated as settled when that branch is invoked
- the optional nature of the security/compliance branch unless local context requires it

## Entry Conditions For The Next Consumer

Before using this scenario, the next consumer should confirm:

- the project actually needs a reusable planning phase
- validated discovery exists or Business Analyst work will be included explicitly
- the role packages referenced in the scenario are available
- the local context does not require a broader delivery-management role that the repository does not yet define

## Active Conditions, Risks, And Watchouts

- avoid treating validated requirements as if they already settle product direction
- avoid letting architecture compensate for weak product direction
- avoid letting planning compensate for weak architecture
- avoid treating optional security/compliance participation as a late packaging step when it actually constrains architecture or planning
- avoid invoking the security/compliance branch without a real trigger
- avoid reading this scenario as a full SDLC or sprint-management framework

## Follow-Up Owners

- Scenario Designer for future scenario-level hardening
- Role Builder if repeated usage exposes gaps in the participating role packages

## Revisit Triggers

- repeated usage shows ambiguous security/compliance trigger conditions
- repeated usage shows the scenario needs stronger local tailoring guidance
- a new delivery-management role is added that changes the right scenario shape
- downstream implementation-oriented scenarios establish stronger handoff expectations that should be reflected here

## Recommended Next Consumer

- `Scenario Designer`
  - when tailoring this reusable scenario for another context
- `Role Builder`
  - when repeated usage reveals a missing role-package capability
- project leads or workflow authors
  - when choosing and applying a reusable planning-phase scenario to real work
