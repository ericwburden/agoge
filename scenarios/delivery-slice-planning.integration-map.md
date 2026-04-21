# Delivery Slice Planning Scenario Integration Map

## Purpose

Capture how the `Delivery Slice Planning` scenario composes role-owned workflows into one coherent bounded-slice shaping phase, including handoffs, dependencies, optional branches, and downstream routing expectations.

## Scenario In Scope

This integration map applies to the reusable `Delivery Slice Planning` scenario defined in [delivery-slice-planning.definition.md](C:/Users/ericw/Projects/orpheum/scenarios/delivery-slice-planning.definition.md).

## Participating Role-Owned Workflows

- [`product-owner-direction.md`](C:/Users/ericw/Projects/orpheum/workflows/product-owner-direction.md)
- [`product-owner-review.md`](C:/Users/ericw/Projects/orpheum/workflows/product-owner-review.md)
- [`product-owner-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/product-owner-handoff.md)
- [`solution-architect-design.md`](C:/Users/ericw/Projects/orpheum/workflows/solution-architect-design.md)
- [`solution-architect-review.md`](C:/Users/ericw/Projects/orpheum/workflows/solution-architect-review.md)
- [`solution-architect-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/solution-architect-handoff.md)
- [`technical-planner-planning.md`](C:/Users/ericw/Projects/orpheum/workflows/technical-planner-planning.md)
- [`technical-planner-review.md`](C:/Users/ericw/Projects/orpheum/workflows/technical-planner-review.md)
- [`technical-planner-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/technical-planner-handoff.md)
- optional [`qa-verification-planning.md`](C:/Users/ericw/Projects/orpheum/workflows/qa-verification-planning.md)
- optional [`security-compliance-specialist-scoping.md`](C:/Users/ericw/Projects/orpheum/workflows/security-compliance-specialist-scoping.md)
- optional [`security-compliance-specialist-review.md`](C:/Users/ericw/Projects/orpheum/workflows/security-compliance-specialist-review.md)
- optional [`security-compliance-specialist-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/security-compliance-specialist-handoff.md)

## Workflow Inputs, Outputs, And Shared Artifacts

- Product Owner workflows consume the broader planning and product context, then produce:
  - updated product direction
  - slice-oriented backlog prioritization
  - product decision review
  - product handoff
- Solution Architect workflows consume the current requirements and slice-oriented product posture, then produce:
  - architecture confirmation or refinement artifacts for the selected slice
  - architecture review
  - architecture handoff
- Technical Planner workflows consume the reviewed architecture and requirements package, then produce:
  - slice-sized implementation strategy
  - sequencing and dependencies for the selected slice
  - implementation plan review
  - implementation handoff
- Optional QA / Verification Lead workflows consume reviewed requirements, architecture, and implementation handoff, then produce:
  - early verification strategy and verification matrix for the slice when those expectations materially constrain the slice before implementation begins
- Optional Security / Compliance Specialist workflows consume the reviewed planning context and relevant obligations, then produce:
  - security/compliance scope
  - controls and evidence matrix
  - security/compliance review
  - security/compliance handoff

Shared artifacts and context that move across the scenario:

- broader reviewed planning package from `Project Planning` or equivalent upstream work
- current product priority posture and acceptance-oriented guardrails for the next candidate
- selected slice boundary, exclusions, and deferred work after architecture and planning confirmation
- architecture constraints, interface seams, and dependency hotspots that still govern the slice
- slice-level sequencing, readiness conditions, and implementation handoff context
- optional early verification expectations and optional security/compliance constraints when those concerns materially shape slice selection

## Handoff Contracts

- broader planning package -> Product Owner
  - Product Owner should receive the broader reviewed plan as the context for selecting the next priority candidate, not as a license to re-plan the whole project from scratch
- Product Owner -> Solution Architect
  - Solution Architect should receive an explicit next-priority posture with current priority, acceptance-sensitive guardrails, and clear exclusions rather than only a broad roadmap statement
- Product Owner and Solution Architect -> Technical Planner
  - Technical Planner should receive a candidate that has current product priority plus preserved architectural constraints, then turn it into a bounded slice rather than inventing both product priority and slice boundary inside execution planning
- Technical Planner -> optional QA / Verification Lead
  - QA / Verification Lead should receive a slice-sized implementation handoff when early verification framing is needed before implementation begins
- Technical Planner and optional QA / Verification Lead or Security / Compliance Specialist -> downstream implementation-oriented work
  - `Implementation and Release Prep` should receive one bounded slice package with explicit readiness conditions instead of a broad project plan or vague milestone theme

## Branching Rules And Decision Logic

- If the broader planning package is still unstable, route upstream rather than pretending slice selection can compensate for missing project-level direction.
- If Product Owner cannot distinguish the next priority candidate from deferred or excluded work, keep that product-shaping work open rather than letting planning invent current priority.
- If the selected slice crosses important architectural seams or dependencies in a way the current architecture handoff does not support clearly, route through Solution Architect confirmation before the slice is treated as stable.
- If the selected slice still represents too much of the project to act as one honest implementation boundary, keep slicing rather than handing the package downstream as though it were ready.
- If early verification expectations materially shape what counts as a viable slice, invoke QA / Verification Lead before implementation begins.
- If obligations, controls, trust boundaries, or approval-sensitive evidence materially shape what counts as a viable slice, invoke Security / Compliance Specialist before the slice is treated as settled.

## Parallelism And Synchronization Points

- Product Owner direction refinement and early architecture confirmation may overlap once the broader planning package is understood.
- Early security/compliance scoping may overlap with slice shaping when obligations are already known.
- Early QA / Verification planning may overlap with late Technical Planner work when the main question is evidence expectation rather than implementation evidence review.
- The scenario must reconverge at:
  - an explicit slice boundary before Technical Planner work is treated as conclusive
  - reviewed architecture-sensitive constraints before the selected slice is treated as technically honest
  - reviewed planning posture before downstream implementation consumes the package
  - optional verification or security/compliance posture before the slice is treated as settled when those branches materially constrain it

## Shared Context, State, And Dependency Assumptions

- The scenario assumes the participating role packages remain the source of truth for role-local execution.
- The scenario assumes a broader reviewed planning package already exists and now needs decomposition into one bounded delivery unit.
- The scenario assumes slice selection is a distinct activity from both project-level planning and implementation execution.
- The scenario assumes the output of this scenario should be small enough to serve as one honest input package for `Implementation and Release Prep`.

## Failure Handling And Escalation Routing

- If product direction for the next slice is weak, route back to Product Owner rather than letting downstream planning guess which work is current.
- If the selected slice depends on unresolved architecture decisions, route back to Solution Architect rather than letting Technical Planner normalize the gap.
- If the slice cannot be expressed as one bounded implementation unit, continue narrowing it or route back to broader planning rather than pushing oversized scope downstream.
- If verification constraints are the main blocker, route to QA / Verification Lead rather than hiding the issue in planning prose.
- If security/compliance constraints are the main blocker, route to Security / Compliance Specialist before downstream implementation begins.

## Coordination Risks And Watchouts

- Product Owner and Technical Planner boundaries can blur if product ordering starts drifting into technical slice decomposition without explicit architectural or planning support.
- Solution Architect and Technical Planner boundaries can blur if slice selection quietly redefines architecture-sensitive seams.
- This scenario is easy to overread as sprint administration; it should stay focused on bounded slice shaping, not delivery-status tracking.
- This scenario is also easy to underuse, with teams jumping directly from project planning to implementation; the explicit bounded-slice handoff should remain visible.
- Optional QA / Verification Lead or Security / Compliance Specialist participation can become ceremonial if trigger conditions are not kept explicit.

## Recommended Next Step

Use the Delivery Slice Planning review artifact to make readiness, participant fit, conditions, and downstream routing into `Implementation and Release Prep` explicit before treating this scenario as adoption-ready.
