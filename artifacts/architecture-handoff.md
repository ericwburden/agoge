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

## Interface, Dependency, And Integration Hotspots

List the interface seams, ownership boundaries, integrations, external dependencies, and architectural hotspots that downstream roles should watch closely.

## Verification Focus Areas

Identify the architectural assumptions, risk areas, or boundary conditions that verification work should pay particular attention to.

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
