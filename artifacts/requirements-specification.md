---
id: requirements-specification
kind: artifact
title: Requirements Specification
version: 1
summary: Capture verified business requirements in a structured form that emphasizes
  clarity, traceability, business rationale, scope, and separation from implementation
  detail.
template: true
default_output_path: docs/discovery/requirements-specification.md
checks:
- requirements-specification
- traceability
- business-analyst-boundary
---

# Requirements Specification

## Purpose

Capture verified business requirements in a structured form that emphasizes clarity, traceability, business rationale, scope, and separation from implementation detail.

Use this artifact after enough discovery has occurred to state the requirements clearly and verify that they support the business objective.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live BA work.

## Completion Guidance

This artifact is complete when the requirements are clear, verifiable, scoped, and traceable to business goals or process needs, and when unresolved questions are explicitly separated from confirmed requirements.

## Related Checks

- Primary: [`requirements-specification.check.md`](D:/Projects/agoge/checks/requirements-specification.check.md)
- Cross-cutting: [`traceability.check.md`](D:/Projects/agoge/checks/traceability.check.md)
- Cross-cutting: [`business-analyst-boundary.check.md`](D:/Projects/agoge/checks/business-analyst-boundary.check.md)

## Source Context

Reference the business objective and process context this specification depends on.

## Requirement Summary

Provide a short summary of what this requirement set covers.

## Verified Requirements

List each verified requirement in a clear and testable form.

For each requirement, capture:

- requirement statement
- business rationale
- related objective or process need
- verification basis or supporting evidence
- any relevant scope boundary

## Non-Requirements

List items that were proposed or discussed but are not confirmed requirements.

## Assumptions

List assumptions that influence the requirements but are not yet verified.

## Open Questions

List unresolved questions that still affect completeness or confidence.

## Constraints

List constraints that materially affect the requirement set.

If the project includes AI-enabled or agentic behavior, keep business constraints and AI or agent-specific constraints clearly separated.

## Locked Business Constraints

Capture business constraints that downstream planning, architecture, or implementation should treat as fixed unless a human explicitly reopens them.

For each locked constraint, capture:

- the constraint statement
- why it is currently fixed
- the downstream implications
- what would justify reopening it

## Acceptance Considerations

Describe what would allow a stakeholder to verify that the requirement has been met.

For AI-enabled projects, include any business-facing acceptance expectations for agent outputs.

## Specification Relationship

If an Allium specification or other behavioral specification already exists, record how this requirement set relates to it.

Capture:

- the existing specification or specification area in scope
- where current discovery confirms existing specified behavior
- where current discovery clarifies or narrows existing specified behavior
- where current discovery exposes specification gaps, conflicts, or outdated behavior
- whether any verified behavior is ready for Allium promotion now or only after more discovery

## Candidate Allium Promotion

Identify any stable business rules or observable behaviors that may be ready to move into Allium once discovery is complete.
