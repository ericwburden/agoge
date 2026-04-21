# Project Planning Scenario Integration Map

## Purpose

Capture how the Project Planning scenario composes role-owned workflows into one coherent pre-implementation planning phase, including handoffs, dependencies, optional branches, and downstream routing expectations.

## Scenario In Scope

This integration map applies to the reusable `Project Planning` scenario defined in [project-planning.definition.md](C:/Users/ericw/Projects/orpheum/scenarios/project-planning.definition.md).

## Participating Role-Owned Workflows

- [`business-analyst-kickoff.md`](C:/Users/ericw/Projects/orpheum/workflows/business-analyst-kickoff.md)
- [`business-analyst-process-analysis.md`](C:/Users/ericw/Projects/orpheum/workflows/business-analyst-process-analysis.md)
- [`business-analyst-requirements-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/business-analyst-requirements-handoff.md)
- [`product-owner-direction.md`](C:/Users/ericw/Projects/orpheum/workflows/product-owner-direction.md)
- [`product-owner-review.md`](C:/Users/ericw/Projects/orpheum/workflows/product-owner-review.md)
- [`product-owner-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/product-owner-handoff.md)
- [`solution-architect-design.md`](C:/Users/ericw/Projects/orpheum/workflows/solution-architect-design.md)
- [`solution-architect-review.md`](C:/Users/ericw/Projects/orpheum/workflows/solution-architect-review.md)
- [`solution-architect-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/solution-architect-handoff.md)
- [`technical-planner-planning.md`](C:/Users/ericw/Projects/orpheum/workflows/technical-planner-planning.md)
- [`technical-planner-review.md`](C:/Users/ericw/Projects/orpheum/workflows/technical-planner-review.md)
- [`technical-planner-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/technical-planner-handoff.md)
- optional [`security-compliance-specialist-scoping.md`](C:/Users/ericw/Projects/orpheum/workflows/security-compliance-specialist-scoping.md)
- optional [`security-compliance-specialist-review.md`](C:/Users/ericw/Projects/orpheum/workflows/security-compliance-specialist-review.md)
- optional [`security-compliance-specialist-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/security-compliance-specialist-handoff.md)

## Workflow Inputs, Outputs, And Shared Artifacts

- Business Analyst workflows produce:
  - business objectives
  - process analysis
  - requirements specification
  - requirements handoff
- Product Owner workflows consume validated requirements and supporting evidence, then produce:
  - product direction
  - backlog prioritization
  - product decision review
  - product handoff
- Solution Architect workflows consume requirements and product-direction inputs, then produce:
  - solution architecture
  - architecture decisions
  - architecture review
  - architecture handoff
- Technical Planner workflows consume reviewed architecture and validated requirements, then produce:
  - implementation strategy
  - sequencing and dependencies
  - implementation plan review
  - implementation handoff
- Optional Security / Compliance Specialist workflows consume reviewed delivery and obligation context, then produce:
  - security/compliance scope
  - controls and evidence matrix
  - security/compliance review
  - security/compliance handoff

Shared artifacts and context that move across the scenario:

- validated requirements
- product direction and current priority posture
- architecture constraints and major decisions
- sequencing, dependency, and readiness assumptions
- optional risk, control, and approval-sensitive guidance

## Handoff Contracts

- Business Analyst -> Product Owner
  - Product Owner should receive validated requirements and discovery context without re-running discovery by default.
- Business Analyst and Product Owner -> Solution Architect
  - Solution Architect should receive clear problem framing, current priorities, acceptance-sensitive constraints, and explicit unresolved business questions.
- Solution Architect -> Technical Planner
  - Technical Planner should receive reviewed architecture and decision context without reconstructing solution shape from raw notes.
- Security / Compliance Specialist -> Solution Architect or Technical Planner
  - When included, the security/compliance package should preserve obligation, control, and approval-sensitive constraints without drifting into detailed implementation ownership.
- Technical Planner -> downstream implementation-oriented work
  - Downstream consumers should receive a reviewed implementation strategy and implementation handoff rather than only an architecture summary.

## Branching Rules And Decision Logic

- If validated discovery does not exist, the scenario should start with Business Analyst work rather than skipping directly to Product Owner or Solution Architect work.
- If current priority direction is already explicit and stable, Product Owner work may be lighter but should not be silently omitted when priority posture still matters.
- If security, compliance, or trust-boundary-sensitive constraints materially affect architecture or planning, invoke the Security / Compliance Specialist branch.
- If product direction, architecture, or planning review is blocked, the scenario should route remediation back to the earliest blocking role package rather than moving forward optimistically.

## Parallelism And Synchronization Points

- Product Owner and early security/compliance framing may overlap when the same governing constraints materially shape direction and architecture.
- Security/compliance scoping may proceed in parallel with architecture drafting when the obligations are already known, but the scenario must reconverge before architecture and planning are treated as settled.
- The scenario must reconverge at:
  - reviewed product posture before architecture depends on it
  - reviewed architecture before implementation planning depends on it
  - reviewed planning posture before downstream implementation consumes the package

## Shared Context, State, And Dependency Assumptions

- The scenario assumes the participating role packages remain the source of truth for role-local execution.
- The scenario assumes the project is mature enough to justify architecture and planning rather than exploratory discovery only.
- The scenario assumes that architecture and planning are downstream of validated needs and explicit product direction.
- The scenario assumes that optional security/compliance participation is triggered by real obligations or risk posture, not by process inflation.

## Failure Handling And Escalation Routing

- If discovery is weak, route upstream to Business Analyst or specification refinement.
- If product direction is ambiguous, route back to Product Owner rather than allowing architecture to absorb the ambiguity.
- If architecture is blocked, route back to Solution Architect rather than letting planning invent the missing solution shape.
- If planning is blocked, route back to Technical Planner rather than treating downstream implementation as the place to discover the missing plan.
- If security/compliance constraints are unresolved and materially affect planning, route back to Security / Compliance Specialist before downstream implementation starts.

## Coordination Risks And Watchouts

- Business Analyst and Product Owner boundaries can blur if discovery updates are mistaken for product prioritization.
- Product Owner and Solution Architect boundaries can blur if architecture starts compensating for weak product direction.
- Solution Architect and Technical Planner boundaries can blur if planning starts inventing the missing solution shape.
- Optional security/compliance participation can become performative if the trigger conditions are not kept explicit.
- This scenario is easy to over-expand into a general project-management layer; downstream routing and review gates must stay visible.

## Recommended Next Step

Use the Project Planning review artifact to make readiness, conditions, and remediation routing explicit before treating this scenario as adoption-ready.
