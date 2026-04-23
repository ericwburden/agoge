---
id: implementation-handoff
kind: artifact
title: Implementation Handoff
version: 1
summary: Capture the downstream-ready planning summary, ordered slices, dependency
  hotspots, readiness conditions, and unresolved issues that implementation and verification
  roles need to continue the work.
template: true
default_output_path: docs/planning/implementation-handoff.md
checks:
- implementation-handoff
- planning-traceability
- technical-planner-boundary
---

# Implementation Handoff

## Purpose

Capture the downstream-ready planning summary, ordered slices, dependency hotspots, readiness conditions, and unresolved issues that implementation and verification roles need to continue the work.

Use this artifact to hand the plan downstream without requiring later roles to reconstruct the execution structure from earlier planning notes.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Technical Planner work.

## Completion Guidance

This artifact is complete when downstream roles can understand the implementation structure they are expected to work within, the planning review status, the main sequencing and dependency constraints that still matter, and the next execution-adjacent decisions that remain.

## Related Checks

- Primary: [`implementation-handoff.check.md`](D:/Projects/agoge/checks/implementation-handoff.check.md)
- Cross-cutting: [`planning-traceability.check.md`](D:/Projects/agoge/checks/planning-traceability.check.md)
- Cross-cutting: [`technical-planner-boundary.check.md`](D:/Projects/agoge/checks/technical-planner-boundary.check.md)

## Handoff Summary

Provide a short summary of what implementation plan is being handed off and why.

## Planning Summary

Summarize the implementation approach, major workstreams, and key sequencing decisions.

## Review Status And Key Findings

Summarize the current planning review status, the most important findings, and any material conditions on downstream use.

## Locked Decisions To Preserve

Summarize the planning decisions, slice boundaries, sequencing commitments, and readiness constraints that downstream roles should not silently revisit.

## Semantic Review Status

State whether semantic artifact review is complete, what materially changed during that review, and whether any cross-artifact reconciliation is still pending.

## Readiness Ownership And Conditions

If the plan is not simply ready, make the ownership and clearance conditions explicit.

Capture:

- the decision owner or approver for the current readiness state
- any condition owners responsible for required follow-up, clarification, or approval
- the conditions that must be satisfied before downstream work should treat the plan as fully ready

## Ordered Slices And Dependency Hotspots

List the intended implementation order, major slice boundaries, external dependencies, and planning hotspots that downstream roles should watch closely.

## Slice Exit Criteria Summary

Summarize the most important slice-level completion or handoff conditions that downstream implementation and verification roles must preserve.

## Verification And Test Strategy Touchpoints

Identify the assumptions, boundary conditions, and high-risk areas that verification work should pay particular attention to as implementation proceeds.

## Rollout, Migration, And Control-Point Watchouts

Identify rollout-sensitive steps, migration boundaries, cutover concerns, or human-control-point constraints that downstream implementation should preserve.

## Specification Relationship

If Allium or another behavioral specification materially constrained the plan, summarize that relationship here.

Capture:

- which specifications were treated as planning inputs
- where the implementation plan preserves or relies on existing specified behavior
- which specification gaps, conflicts, or unstable behaviors still matter downstream
- whether any planning issue should route back to upstream specification or architecture work rather than be solved in implementation

## Unresolved Decisions And Risks

List unresolved planning decisions, significant risks, and tradeoffs still in play.

## Deferred Or Not Included

Summarize the work that this implementation plan intentionally defers, excludes, or leaves for a later planning or delivery stage.

## Recommended Downstream Consumers

Identify which roles should consume this handoff, such as:

- implementation engineer
- QA or verification lead
- code reviewer
- release or handoff manager

## Next Decision Points

Describe the most important downstream planning-adjacent decisions that remain, without turning this artifact into a sprint board or ticket backlog.
