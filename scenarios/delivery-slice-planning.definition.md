# Delivery Slice Planning Scenario Definition

## Purpose

Capture the reusable `Delivery Slice Planning` scenario that turns a broader reviewed planning package into one bounded delivery slice with explicit scope boundaries, sequencing assumptions, readiness conditions, and downstream handoff context.

Use this scenario when a team already has project-level planning but needs a disciplined multi-role step before implementation begins on the next slice.

## Scenario Name And Intent

`Delivery Slice Planning`

This scenario exists to compose the repository's product, architecture, planning, and optional quality or security roles into one reusable slice-shaping phase that reduces ambiguity between "we have a project plan" and "we have one bounded slice that is honestly ready for downstream implementation and release preparation."

## Lifecycle Window And Trigger Conditions

This scenario sits between reviewed project planning and bounded-slice implementation-oriented delivery work.

Trigger it when:

- reviewed requirements, architecture, and implementation planning already exist for a larger project, initiative, or roadmap segment
- the current package is too broad to serve as one honest implementation boundary
- the next delivery slice needs explicit in-scope versus out-of-scope definition before coding begins
- implementation, verification, or release-preparation work would otherwise need to infer slice boundaries from broader project planning artifacts

## Participating Roles And Why

- [`Product Owner`](C:/Users/ericw/Projects/orpheum/roles/product-owner.md)
  - selects the next priority candidate from the broader product posture and preserves priority, value, and acceptance-oriented guardrails for downstream slice shaping
- [`Solution Architect`](C:/Users/ericw/Projects/orpheum/roles/solution-architect.md)
  - confirms that the selected slice still respects the reviewed architecture, interface seams, and major technical constraints
- [`Technical Planner`](C:/Users/ericw/Projects/orpheum/roles/technical-planner.md)
  - turns the broader implementation strategy into one bounded slice plan with explicit sequencing, dependency, readiness, and exclusion logic
- optional [`QA / Verification Lead`](C:/Users/ericw/Projects/orpheum/roles/qa-verification-lead.md)
  - participates when the slice needs early verification framing before implementation begins, especially where confidence targets or evidence expectations materially constrain the slice
- optional [`Security / Compliance Specialist`](C:/Users/ericw/Projects/orpheum/roles/security-compliance-specialist.md)
  - participates when obligations, trust boundaries, approval-sensitive controls, or evidence expectations materially shape slice boundaries or readiness

## Entry Conditions

- a broader reviewed planning package already exists, typically from `Project Planning` or an equivalent reviewed planning chain
- the next delivery target is still broad enough that a bounded slice decision is required before implementation begins
- the participating role packages are available and treated as the source of truth for role-local workflows
- the scenario is being used as a reusable slice-shaping phase, not as a sprint board, staffing plan, or status ritual

## Target Outputs And Exit Conditions

The scenario completes successfully when the downstream slice package includes:

- explicit product rationale for why this slice is next
- explicit in-scope, out-of-scope, and deferred boundaries for the selected slice
- preserved architectural constraints, interface seams, and decision hotspots that still govern the slice
- a reviewed slice-sized implementation strategy, sequencing posture, and implementation handoff
- optional early verification framing and optional security/compliance guidance when those concerns materially constrain the slice before implementation begins

Exit condition:

- downstream implementation-oriented work can begin on one bounded delivery slice without redefining product priority, architecture constraints, or planning boundaries from the broader project package

## Core Sequence

1. Consume the broader reviewed planning package and identify the next priority candidate through Product Owner outputs.
2. Confirm that the candidate can become a bounded slice without silently crossing important architecture, interface, dependency, or trust-boundary seams.
3. Turn that candidate into an explicit bounded slice through Solution Architect and Technical Planner outputs, including slice-sized implementation strategy, dependency posture, readiness view, and downstream implementation handoff.
4. Optionally add early verification framing when confidence targets, evidence expectations, or acceptance-sensitive hotspots materially shape the slice before coding begins.
5. Optionally add security/compliance framing when obligations, controls, or approval-sensitive constraints materially shape slice boundaries or readiness.
6. Review the resulting slice package before it is treated as the next bounded delivery unit.
7. Hand the completed slice package downstream to `Implementation and Release Prep`.

## Decision Gates And Human Checkpoints

- the selected priority candidate must be explicit enough that downstream roles can shape one honest slice from it
- product review must be explicit before downstream roles treat that candidate as the current priority unit
- architecture-sensitive constraints must remain explicit when the selected slice crosses important seams or dependencies
- planning review must be explicit before the slice is treated as implementation-ready
- verification or security/compliance review should become explicit gates when those concerns materially constrain what counts as an honest slice
- human approval remains visible when the slice boundary depends on unresolved tradeoffs, sensitive controls, or risky scope compression

## Scenario Constraints And Non-Goals

- This scenario does not replace role-local workflows; it composes them.
- This scenario does not replace broader project planning, architecture, or implementation planning.
- This scenario does not absorb implementation execution, code review, verification execution, or release preparation.
- This scenario is not a sprint administration layer, staffing mechanism, or progress tracker.
- This scenario should stay reusable across projects and should not be overfit to one team cadence or delivery ceremony.

## Open Questions And Design Gaps

- Repeated usage may show that the repository needs a narrower remediation or re-slicing scenario when a slice proves too large or unstable mid-delivery.
- Repeated usage may show a need for stronger default rules about when QA / Verification Lead should join before implementation rather than only downstream.
- Repeated usage may show a need for stronger trigger rules around optional security/compliance participation for slice-sensitive trust boundaries.

## Recommended Next Step

Use the Delivery Slice Planning integration map to make slice selection, handoffs, optional branches, and downstream routing into `Implementation and Release Prep` explicit.
