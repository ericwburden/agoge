---
id: architecture-handoff
kind: artifact
title: Architecture Handoff
version: 1
summary: Capture the downstream-ready architecture summary, decision framing, dependency
  hotspots, and unresolved issues that planning, implementation, and verification
  roles need to continue the work.
template: true
default_output_path: docs/architecture/architecture-handoff.md
checks:
- architecture-handoff
- architecture-traceability
- solution-architect-boundary
---

# Architecture Handoff

## Purpose

Capture the downstream-ready architecture summary, decision framing, dependency hotspots, and unresolved issues that planning, implementation, and verification roles need to continue the work.

Use this artifact to hand architecture downstream without requiring later roles to reconstruct the design rationale from earlier architectural notes.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Solution Architect work.

## Completion Guidance

This artifact is complete when downstream roles can understand the architecture they are expected to work within, the decisions already made, the review status of the architecture, the interface or dependency hotspots that still matter, and the next design questions that remain.

## Related Checks

- Primary: [`architecture-handoff.check.md`](D:/Projects/agoge/checks/architecture-handoff.check.md)
- Cross-cutting: [`architecture-traceability.check.md`](D:/Projects/agoge/checks/architecture-traceability.check.md)
- Cross-cutting: [`solution-architect-boundary.check.md`](D:/Projects/agoge/checks/solution-architect-boundary.check.md)

## Handoff Summary

Provide a short summary of what architecture is being handed off and why.

## Architecture Summary

Summarize the proposed solution shape, major components, and key architectural decisions.

## Review Status And Key Findings

Summarize the current architecture review status, the most important findings, and any material conditions on downstream use.

## Locked Decisions To Preserve

Summarize the architecture decisions, constraints, boundary commitments, and interface expectations that downstream roles should not silently revisit.

## Semantic Review Status

State whether semantic artifact review is complete, what materially changed during that review, and whether any cross-artifact reconciliation is still pending.

## Readiness Ownership And Conditions

If the architecture is not simply ready, make the ownership and clearance conditions explicit.

Capture:

- the decision owner or approver for the current readiness state
- any condition owners responsible for required follow-up, clarification, or approval
- the conditions that must be satisfied before downstream work should treat the architecture as fully ready

## Interface, Dependency, And Integration Hotspots

List the interface seams, ownership boundaries, integrations, external dependencies, and architectural hotspots that downstream roles should watch closely.

## Verification Focus Areas

Identify the architectural assumptions, risk areas, or boundary conditions that verification work should pay particular attention to.

## Architecture Fitness Criteria

Summarize the architecture-level criteria that downstream verification or planning should preserve, validate, or monitor.

## Specification Relationship

If Allium or another behavioral specification materially constrained the architecture, summarize that relationship here.

Capture:

- which specifications were treated as architectural inputs
- where the architecture preserves or relies on existing specified behavior
- which specification gaps, conflicts, or unstable behaviors still matter downstream
- whether any architectural issue should route back to upstream specification work rather than be solved in planning or implementation

## Unresolved Decisions And Risks

List unresolved architectural decisions, significant risks, and tradeoffs still in play.

## Recommended Downstream Consumers

Identify which roles should consume this handoff, such as:

- technical planner
- implementation engineer
- QA or verification lead
- code reviewer

## Next Decision Points

Describe the most important downstream architecture-adjacent decisions that remain, without turning this artifact into implementation planning or task decomposition.
