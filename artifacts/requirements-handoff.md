---
id: requirements-handoff
kind: artifact
title: Requirements Handoff
version: 1
summary: Capture the downstream-ready summary of verified requirements, traceability,
  risks, unresolved questions, and role-to-role handoff information.
template: true
default_output_path: docs/discovery/requirements-handoff.md
checks:
- requirements-handoff
- traceability
- business-analyst-boundary
---

# Requirements Handoff

## Purpose

Capture the downstream-ready summary of verified requirements, traceability, risks, unresolved questions, and role-to-role handoff information.

Use this artifact to prepare product, architecture, delivery, or implementation roles to continue the work without losing the business context established during discovery.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live BA work.

## Completion Guidance

This artifact is complete when a downstream role can understand the business problem, the verified requirements, how those requirements trace back to objectives and process needs, and what unresolved risks or questions still matter.

## Related Checks

- Primary: [`requirements-handoff.check.md`](D:/Projects/agoge/checks/requirements-handoff.check.md)
- Cross-cutting: [`traceability.check.md`](D:/Projects/agoge/checks/traceability.check.md)
- Cross-cutting: [`business-analyst-boundary.check.md`](D:/Projects/agoge/checks/business-analyst-boundary.check.md)

## Handoff Summary

Provide a short summary of what is being handed off and why.

## Business Problem And Desired Outcome

State the problem being addressed and the desired business outcome.

## Verified Requirements

Summarize the requirements that are ready for downstream use.

## Traceability

Show how the requirements map back to:

- business objectives
- process needs
- major business rules

## Risks And Dependencies

List material risks, dependencies, constraints, or external conditions that downstream roles need to know.

## Assumptions And Open Questions

List assumptions still in play and questions that remain unresolved.

## Confirmation Status

State whether the framing and requirement set have been confirmed with stakeholders, partially confirmed, or are still pending.

Capture:

- overall confirmation status
- which stakeholders or stakeholder groups have responded
- any material disagreements, corrections, or pending confirmations that downstream roles should account for

## Specification Relationship

If Allium or another behavioral specification is in scope, summarize how the BA outputs relate to it.

Capture:

- whether an existing specification was reviewed
- which requirements or behaviors confirm the specification
- which findings clarify, narrow, or extend the specification
- which gaps, conflicts, or outdated behaviors still need specification work
- whether any verified behavior is ready for deliberate Allium promotion

## Human Oversight And AI Notes

If the system includes AI-enabled or agentic behavior, capture:

- expected human review or approval points
- escalation triggers
- acceptance expectations for agent outputs
- AI or agent-specific constraints separate from business objectives

## Recommended Downstream Consumers

Identify which roles should consume this handoff, such as:

- product owner
- architect
- delivery lead
- implementation agent

## Next Decision Points

Describe the most important downstream decisions that remain, without turning this artifact into implementation planning.
